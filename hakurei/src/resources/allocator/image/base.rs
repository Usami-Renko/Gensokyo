
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::{ HaDevice, HaLogicalDevice };
use core::physical::{ HaPhyDevice, MemorySelector };

use resources::repository::HaImageRepository;
use resources::image::{ ImageViewItem, ImageDescInfo, ImageViewDescInfo };
use resources::image::{ HaImage, HaImageView };
use resources::buffer::{ StagingBufferConfig, StagingBufferUsage, BufferSubItem };
use resources::buffer::BufferConfigModifiable;
use resources::image::{ ImageLayout, ImageStorageInfo, load_texture };
use resources::allocator::{ HaBufferAllocator, BufferStorageType, ImgMemAlloAbstract };
use resources::allocator::{ DeviceImgMemAllocator, CachedImgMemAllocator };
use resources::memory::HaMemoryType;
use resources::command::{ HaCommandRecorder, CommandBufferUsageFlag };
use resources::error::{ ImageError, AllocatorError };
use pipeline::stages::PipelineStageFlag;
use pipeline::pass::AccessFlag;

use utility::memory::bind_to_alignment;
use utility::marker::{ VulkanFlags, VulkanEnum };

use std::path::Path;
use std::ptr;

// TODO: Currently not support multi imageview for an image.

pub struct HaImageAllocator {

    physical: HaPhyDevice,
    device  : HaDevice,

    images  : Vec<HaImage>,
    storages: Vec<ImageStorageInfo>,
    spaces  : Vec<vk::DeviceSize>,

    image_descs: Vec<ImageDescInfo>,
    view_descs : Vec<ImageViewDescInfo>,

    ty: ImageStorageType,
    allocator: Box<ImgMemAlloAbstract>,
    require_mem_flag: vk::MemoryPropertyFlags,
    memory_selector : MemorySelector,
}

impl HaImageAllocator {

    pub(crate) fn new(physical: &HaPhyDevice, device: &HaDevice, ty: ImageStorageType) -> HaImageAllocator {

        HaImageAllocator {

            physical: physical.clone(),
            device  : device.clone(),

            images  : vec![],
            storages: vec![],
            spaces  : vec![],

            image_descs: vec![],
            view_descs : vec![],

            ty,
            allocator: ty.allocator(),
            require_mem_flag: ty.memory_type().property_flags(),
            memory_selector : MemorySelector::init(physical),
        }
    }

    pub fn attach_image(&mut self, path: &Path, image_desc: ImageDescInfo, view_desc: ImageViewDescInfo) -> Result<usize, AllocatorError> {

        let storage = load_texture(path)?;
        let image = HaImage::config(&self.device, &image_desc, storage.dimension, storage.format)?;

        self.memory_selector.try(image.requirement.memory_type_bits, self.require_mem_flag)?;

        let aligment_space = bind_to_alignment(image.requirement.size, image.requirement.alignment);

        let image_index = self.images.len();

        self.storages.push(storage);
        self.images.push(image);
        self.image_descs.push(image_desc);
        self.view_descs.push(view_desc);
        self.spaces.push(aligment_space);

        Ok(image_index)
    }

    pub fn allocate(&mut self) -> Result<HaImageRepository, AllocatorError> {

        if self.images.is_empty() {
            return Err(AllocatorError::Image(ImageError::NoImageAttachError))
        }

        // 1.create staging buffer and memories
        let mut staging_allocator = HaBufferAllocator::new(&self.physical, &self.device, BufferStorageType::Staging);
        let staging_buffer_config = StagingBufferConfig::new(StagingBufferUsage::ImageCopySrc);
        let mut staging_buffer_items = vec![];

        for storage in self.storages.iter() {
            let mut config = staging_buffer_config.clone();
            let _ = config.add_item(storage.size);
            let item = staging_allocator.attach_staging_buffer(config)?.pop().unwrap();
            staging_buffer_items.push(item);
        }

        let mut staging_repository = staging_allocator.allocate()?;

        // 2.send textures to the staging buffer
        {
            let mut uploader = staging_repository.data_uploader()?;
            for (i, item) in staging_buffer_items.iter().enumerate() {
                uploader.upload(item, &self.storages[i].data)?;
            }
            uploader.done()?;
        }

        // 3.create image buffer and memories
        let optimal_memory_index = self.memory_selector.optimal_memory()?;
        let mem_type = self.physical.memory.memory_type(optimal_memory_index);

        self.allocator.allocate(
            &self.device, self.spaces.iter().sum(), optimal_memory_index, Some(mem_type)
        )?;

        {
            let memory = self.allocator.borrow_memory()?;

            // bind images to memory
            let mut offset = 0;
            for (i, image) in self.images.iter().enumerate() {
                memory.bind_to_image(&self.device, image, offset)?;
                offset += self.spaces[i];
            }
        }

        // 4.create image view for each image
        let mut views = vec![];
        for i in 0..self.images.len() {
            let view = HaImageView::config(&self.device, &self.images[i], &self.view_descs[i], self.storages[i].format)?;
            views.push(view);
        }

        // 5.create command buffer
        let mut transfer = HaLogicalDevice::transfer(&self.device);
        {
            let command_buffer = transfer.command()?;

            let recorder = command_buffer.setup_record();
            let _ = recorder.begin_record(&[CommandBufferUsageFlag::OneTimeSubmitBit])?;

            // 6.transition image layout (from undefine to transferDst)

            let mut image_barriers = vec![];
            for (image, view_desc) in self.images.iter().zip(&self.view_descs) {
                let barrier = vk::ImageMemoryBarrier {
                    s_type: vk::StructureType::ImageMemoryBarrier,
                    p_next: ptr::null(),
                    src_access_mask: vk::AccessFlags::empty(),
                    dst_access_mask: [AccessFlag::TransferWriteBit].flags(),
                    old_layout: ImageLayout::Undefined.value(),
                    new_layout: ImageLayout::TransferDstOptimal.value(),
                    // TODO: Current ignore queue family ownership transfer.
                    // srcQueueFamilyIndex is the source queue family for a queue family ownership transfer.
                    src_queue_family_index : vk::VK_QUEUE_FAMILY_IGNORED,
                    // dstQueueFamilyIndex is the destination queue family for a queue family ownership transfer.
                    dst_queue_family_index : vk::VK_QUEUE_FAMILY_IGNORED,
                    image: image.handle,
                    subresource_range: view_desc.subrange.clone(),
                };
                image_barriers.push(barrier);
            }
            let _ = recorder.pipeline_barrrier(
                PipelineStageFlag::TopOfPipeBit,
                PipelineStageFlag::TransferBit,
                // dependencies specifying how execution and memory dependencies are formed
                &[],
                &[],
                &[],
                &image_barriers
            );

            // 7.copy textures from buffer to image
            // do image barrier transition and copy buffer to image.
            for (i, buffer_item) in staging_buffer_items.iter().enumerate() {
                let image_item = ImageViewItem {
                    image_handle: self.images[i].handle,
                    view_handle : views[i].handle,
                    view_index  : i,
                };
                self.copy_buffer_to_image(&recorder, buffer_item, &image_item)?;
            }

            // 8.transition image layout (from transferDst to shader read only optimal)
            for barrier in image_barriers.iter_mut() {
                barrier.src_access_mask = [AccessFlag::TransferWriteBit].flags();
                barrier.dst_access_mask = [AccessFlag::ShaderReadBit].flags();
                barrier.old_layout = ImageLayout::TransferDstOptimal.value();
                barrier.new_layout = ImageLayout::ShaderReadOnlyOptimal.value();
            }
            let _ = recorder.pipeline_barrrier(
                PipelineStageFlag::TransferBit,
                PipelineStageFlag::FragmentShaderBit,
                &[],
                &[],
                &[],
                &image_barriers
            );


            // 9.submit command buffer
            recorder.end_record()?;
        }

        transfer.excute()?;

        // 10.clean resources.
        staging_repository.cleanup();

        // finial done.
        let repository = HaImageRepository::store(
            &self.device,
            self.images.drain(..).collect(),
            views,
            self.allocator.take_memory()?
        );
        Ok(repository)
    }

    fn copy_buffer_to_image(&self, recorder: &HaCommandRecorder, from_item: &BufferSubItem, to_item: &ImageViewItem) -> Result<(), AllocatorError> {

        let subsource = &self.view_descs[to_item.view_index].subrange;
        let dimension = self.storages[to_item.view_index].dimension;
        // TODO: Only support one region.
        let copy_regions = [
            vk::BufferImageCopy {
                buffer_offset: from_item.offset,
                // TODO: the following options are not configurable.
                buffer_row_length  : 0,
                buffer_image_height: 0,
                image_subresource: vk::ImageSubresourceLayers {
                    aspect_mask: subsource.aspect_mask,
                    mip_level  : subsource.base_mip_level,
                    layer_count: subsource.layer_count,
                    base_array_layer: subsource.base_array_layer,
                },
                // imageOffset selects the initial x, y, z offsets in texels of the sub-region of the source or destination image data.
                image_offset: vk::Offset3D { x: 0, y: 0, z: 0 },
                // imageExtent is the size in texels of the image to copy in width, height and depth.
                image_extent: dimension.clone(),
            },
        ];

        let _ = recorder.copy_buffer_to_image(
            from_item.handle,
            to_item.image_handle,
            // TODO: make dst_layout configurable.
            vk::ImageLayout::TransferDstOptimal,
            &copy_regions
        );
        Ok(())
    }

    pub fn reset(&mut self) {

        unsafe {
            for image in self.images.iter() {
                self.device.handle.destroy_image(image.handle, None);
            }
        }

        self.images.clear();
        self.storages.clear();
        self.spaces.clear();
        self.memory_selector.reset();
        self.require_mem_flag = self.ty.memory_type().property_flags();
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageStorageType {
    Device,
    Cached,
}

impl ImageStorageType {

    fn allocator(&self) -> Box<ImgMemAlloAbstract> {
        match self {
            | ImageStorageType::Device => Box::new(DeviceImgMemAllocator::new()),
            | ImageStorageType::Cached => Box::new(CachedImgMemAllocator::new()),
        }
    }

    fn memory_type(&self) -> HaMemoryType {
        match self {
            | ImageStorageType::Cached  => HaMemoryType::CachedMemory,
            | ImageStorageType::Device  => HaMemoryType::DeviceMemory,
        }
    }
}
