
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;
use core::physical::{ HaPhysicalDevice, MemorySelector };

use resources::repository::HaImageRepository;
use resources::image::{ ImageViewItem, ImageDescInfo, ImageViewDescInfo };
use resources::image::{ HaImage, HaImageView };
use resources::buffer::{BufferConfig, BufferUsageFlag, BufferSubItem};
use resources::memory::MemoryPropertyFlag;
use resources::image::{ ImageLayout, ImageStorageInfo, load_texture };
use resources::allocator::buffer::HaBufferAllocator;
use resources::memory::{ HaMemoryAbstract, HaDeviceMemory };
use resources::command::{ HaCommandRecorder, CommandBufferUsageFlag };
use resources::error::{ ImageError, AllocatorError };
use pipeline::stages::PipelineStageFlag;
use pipeline::pass::AccessFlag;

use utility::memory::bind_to_alignment;
use utility::marker::{ VulkanFlags, VulkanEnum };

use std::path::Path;
use std::ptr;

// TODO: Currently not support multi imageview for an image.

pub struct HaImageAllocator<'re> {

    physical: &'re HaPhysicalDevice,
    device  : &'re HaLogicalDevice,

    images  : Vec<HaImage>,
    storages: Vec<ImageStorageInfo>,
    spaces  : Vec<vk::DeviceSize>,

    image_descs: Vec<ImageDescInfo>,
    view_descs : Vec<ImageViewDescInfo>,

    memory_selector: MemorySelector<'re>,
    mem_flag: vk::MemoryPropertyFlags,
}

impl<'re> HaImageAllocator<'re> {

    pub(super) fn new(physical: &'re HaPhysicalDevice, device: &'re HaLogicalDevice) -> HaImageAllocator<'re> {

        HaImageAllocator {
            physical,
            device,

            images  : vec![],
            storages: vec![],
            spaces  : vec![],
            image_descs: vec![],
            view_descs : vec![],
            memory_selector: MemorySelector::init(physical),
            mem_flag: vk::MemoryPropertyFlags::empty(),
        }
    }

    pub fn attach_image(&mut self, path: &Path, image_desc: ImageDescInfo, view_desc: ImageViewDescInfo) -> Result<usize, AllocatorError> {

        let storage = load_texture(path)?;
        let image = HaImage::config(self.device, &image_desc, storage.dimension, storage.format)?;

        // create the buffer
        // TODO: Make this flag as a choose
        let required_memory_flag = vk::MEMORY_PROPERTY_DEVICE_LOCAL_BIT;
        self.memory_selector.try(image.requirement.memory_type_bits, required_memory_flag)?;

        self.mem_flag = self.mem_flag | required_memory_flag;
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
        let mut allocator = HaBufferAllocator::new(self.physical, self.device);
        let mut staging_buffer_items = vec![];

        for storage in self.storages.iter() {
            let mut staging_buffer_config = BufferConfig::init(
                &[BufferUsageFlag::TransferSrcBit],
                &[
                    MemoryPropertyFlag::HostVisibleBit,
                    MemoryPropertyFlag::HostCoherentBit,
                ],
                &[]
            );
            let _ = staging_buffer_config.add_item(storage.size);
            let item = allocator.attach_buffer(staging_buffer_config)?.pop().unwrap();
            staging_buffer_items.push(item);
        }
        let mut staging_repository = allocator.allocate()?;

        // 2.send textures to the staging buffer
        for (i, item) in staging_buffer_items.iter().enumerate() {
            staging_repository.tranfer_data(self.device, &self.storages[i].data, item)?;
        }

        // 3.create image buffer and memories
        // TODO: Reduce duplicate code same in resources::allocator::buffer.
        let optimal_memory_index = self.memory_selector.optimal_memory()?;
        let allocate_size = self.spaces.iter().sum();
        // allocate memory
        let memory = HaDeviceMemory::allocate(
            self.physical, self.device,
            allocate_size,
            optimal_memory_index,
            self.mem_flag
        )?;

        // bind images to memory
        let mut offset = 0;
        for (i, image) in self.images.iter().enumerate() {
            memory.bind_to_image(self.device, image, offset)?;
            offset += self.spaces[i];
        }

        // 4.create image view for each image
        let mut views = vec![];
        for i in 0..self.images.len() {
            let view = HaImageView::config(self.device, &self.images[i], &self.view_descs[i], self.storages[i].format)?;
            views.push(view);
        }

        // 5.create command buffer
        let mut transfer = self.device.transfer();
        {
            let command_buffer = transfer.command()?;

            let recorder = command_buffer.setup_record(self.device);
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
            recorder.finish()?;
        }

        transfer.excute()?;

        // 10.clean resources.
        staging_repository.cleanup(self.device);

        // finial done.
        let image_ownership_transfer = self.images.drain(..).collect();
        let repository = HaImageRepository::store(image_ownership_transfer, views, memory);
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
        self.mem_flag = vk::MemoryPropertyFlags::empty();
    }
}
