
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::{ HaDevice, HaLogicalDevice };
use core::physical::{ HaPhyDevice, MemorySelector };

use resources::repository::{ HaImageRepository, HaBufferRepository };
use resources::image::{ ImageDescInfo, ImageViewDescInfo };
use resources::image::{ HaImage, HaImageView, ImagePipelineStage };
use resources::image::{ HaSampler, SampleImageInfo, HaSampleImage };
use resources::image::{ DepthStencilImageInfo, HaDepthStencilImage, DepthImageUsage };
use resources::buffer::{ StagingBufferConfig, StagingBufferUsage, BufferSubItem };
use resources::buffer::BufferConfigModifiable;
use resources::image::{ ImageLayout, ImageStorageInfo };
use resources::image::{ ImageSource, ImageVarietyType };
use resources::allocator::{ HaBufferAllocator, BufferStorageType, ImgMemAlloAbstract };
use resources::allocator::{ DeviceImgMemAllocator, CachedImgMemAllocator };
use resources::memory::HaMemoryType;
use resources::command::{ HaCommandRecorder, CommandBufferUsageFlag };
use resources::error::{ ImageError, AllocatorError };
use pipeline::stages::PipelineStageFlag;
use pipeline::pass::AccessFlag;

use utility::memory::bind_to_alignment;
use utility::marker::{ VulkanFlags, VulkanEnum };
use utility::dimension::Dimension2D;

use std::path::Path;
use std::collections::hash_map::{ HashMap, RandomState };
use std::ptr;

// TODO: Currently not support multi imageview for an image.

pub struct HaImageAllocator {

    physical: HaPhyDevice,
    device  : HaDevice,

    image_infos: Vec<ImageAllocateInfo>,

    storage_type: ImageStorageType,
    allocator: Box<ImgMemAlloAbstract>,
    require_mem_flag: vk::MemoryPropertyFlags,
    memory_selector : MemorySelector,
}

impl HaImageAllocator {

    pub(crate) fn new(physical: &HaPhyDevice, device: &HaDevice, ty: ImageStorageType) -> HaImageAllocator {

        HaImageAllocator {

            physical: physical.clone(),
            device  : device.clone(),

            image_infos: vec![],

            storage_type: ty,
            allocator: ty.allocator(),
            require_mem_flag: ty.memory_type().property_flags(),
            memory_selector : MemorySelector::init(physical),
        }
    }

    pub fn attach_sample_image(&mut self, path: &Path, info: SampleImageInfo) -> Result<HaSampleImage, AllocatorError> {

        let storage = ImageStorageInfo::from_load2d(path)?;
        let image = HaImage::config(&self.device, &info.image_desc, storage.dimension, storage.format)?;
        self.memory_selector.try(image.requirement.memory_type_bits, self.require_mem_flag)?;

        let sampler = HaSampler::new(&self.device, info.sampler_desc)?;
        let result = HaSampleImage::setup(sampler, info.binding, info.count, self.image_infos.len());

        let image_info = ImageAllocateInfo::new(ImageVarietyType::SampleImage(info.pipeline_stage), storage, image, info.image_desc, info.view_desc);
        self.image_infos.push(image_info);

        Ok(result)
    }

    pub fn attach_depth_stencil_image(&mut self, info: DepthStencilImageInfo, dimension: Dimension2D) -> Result<HaDepthStencilImage, AllocatorError> {

        let storage = ImageStorageInfo::from_unload(dimension, info.usage.dst_format(&self.physical));
        let image = HaImage::config(&self.device, &info.image_desc, storage.dimension, storage.format)?;
        self.memory_selector.try(image.requirement.memory_type_bits, self.require_mem_flag)?;

        let result = HaDepthStencilImage::setup(info.binding, info.count, self.image_infos.len(), storage.format);

        let mut view_desc = info.view_desc;
        view_desc.reset_depth_image_aspect_mask(storage.format);

        let image_info = ImageAllocateInfo::new(ImageVarietyType::DepthStencilImage(info.usage), storage, image, info.image_desc, view_desc);
        self.image_infos.push(image_info);

        Ok(result)
    }

    pub fn allocate(&mut self) -> Result<HaImageRepository, AllocatorError> {

        if self.image_infos.is_empty() {
            return Err(AllocatorError::Image(ImageError::NoImageAttachError))
        }

        // 1.create image buffer and memories.
        let optimal_memory_index = self.memory_selector.optimal_memory()?;
        let mem_type = self.physical.memory.memory_type(optimal_memory_index);
        let total_space = self.image_infos.iter()
            .fold(0, |sum, image_info| {
                sum + image_info.space
            });

        self.allocator.allocate(
            &self.device, total_space, optimal_memory_index, Some(mem_type)
        )?;

        {
            let memory = self.allocator.borrow_memory()?;

            // bind images to memory.
            let mut offset = 0;
            for image_info in self.image_infos.iter() {
                memory.bind_to_image(&self.device, &image_info.image, offset)?;
                offset += image_info.space;
            }
        }

        // 2.create image view for each image.
        let mut views = vec![];
        for image_info in self.image_infos.iter() {
            views.push(image_info.generate_view(&self.device)?);
        }

        // 3.create command buffer.
        let mut transfer = HaLogicalDevice::transfer(&self.device);

        let mut barrier_bundles = {

            let command_buffer = transfer.command()?;

            let recorder = command_buffer.setup_record();
            recorder.begin_record(&[CommandBufferUsageFlag::OneTimeSubmitBit])?;

            // 4. make image barrier transitions.
            let mut barrier_bundles = collect_barrier_bundle(&self.physical, &self.device, &self.image_infos);
            for bundle in barrier_bundles.iter_mut() {
                bundle.make_transfermation(&recorder, &self.image_infos)?;
            }

            // 5.submit command buffer.
            recorder.end_record()?;

            barrier_bundles
        };

        // 6.execute the command.
        transfer.excute()?;

        barrier_bundles.iter_mut()
            .for_each(|bundle| bundle.cleanup());

        // clear the image_infos, and give the images ownership to HaImageRepository.
        let images = self.image_infos.drain(..)
            .map(|info| info.image).collect::<Vec<_>>();

        // final done.
        let repository = HaImageRepository::store(&self.device, images, views, self.allocator.take_memory()?);
        Ok(repository)
    }

    pub fn reset(&mut self) {

        self.image_infos.iter().for_each(|image_info| {
            image_info.cleanup(&self.device);
        });

        self.memory_selector.reset();
        self.require_mem_flag = self.storage_type.memory_type().property_flags();
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

trait ImageBarrierBundle {

    fn make_transfermation(&mut self, recorder: &HaCommandRecorder, infos: &Vec<ImageAllocateInfo>) -> Result<(), AllocatorError>;
    fn cleanup(&mut self);
}

struct SampleImageBarrierBundle {

    info_indices: Vec<usize>,
    dst_stage   : ImagePipelineStage,

    staging_allocator : HaBufferAllocator,
    staging_repository: Option<HaBufferRepository>,
}

impl ImageBarrierBundle for SampleImageBarrierBundle {

    fn make_transfermation(&mut self, recorder: &HaCommandRecorder, infos: &Vec<ImageAllocateInfo>) -> Result<(), AllocatorError> {

        // create staging buffer and memories
        let (mut staging_repository, buffer_items) = self.create_staging_repository(infos)?;
        // send textures to the staging buffer
        self.upload_staging_data(&mut staging_repository, &buffer_items, infos)?;

        // make image barrier transition for data transfer.
        let transfer_barriers = self.info_indices.iter()
            .map(|&index| self.transfer_barrier(&infos[index])).collect::<Vec<_>>();
        recorder.pipeline_barrrier(
            PipelineStageFlag::TopOfPipeBit.value(),
            PipelineStageFlag::TransferBit.value(),
            &[], // dependencies specifying how execution and memory dependencies are formed.
            &[],
            &[],
            &transfer_barriers
        );

        // copy buffer to image.
        for (i, &index) in self.info_indices.iter().enumerate() {
            copy_buffer_to_image(recorder, &buffer_items[i], &infos[index])?;
        }

        // make image barrier transition for final layout.
        let final_barriers = self.info_indices.iter()
            .map(|&index| self.final_barrier(&infos[index])).collect::<Vec<_>>();
        let _ = recorder.pipeline_barrrier(
            PipelineStageFlag::TransferBit.value(),
            self.dst_stage.to_stage_flag().value(),
            &[],
            &[],
            &[],
            &final_barriers
        );

        self.staging_repository = Some(staging_repository);

        Ok(())
    }

    fn cleanup(&mut self) {
        if let Some(ref mut repository) = self.staging_repository {
            repository.cleanup();
        }
    }
}

impl SampleImageBarrierBundle {

    fn new(physical: &HaPhyDevice, device: &HaDevice, dst_stage: ImagePipelineStage, indices: Vec<usize>) -> SampleImageBarrierBundle {
        SampleImageBarrierBundle {
            info_indices: indices, dst_stage,
            staging_allocator : HaBufferAllocator::new(physical, device, BufferStorageType::Staging),
            staging_repository: None,
        }
    }

    fn create_staging_repository(&mut self, infos: &Vec<ImageAllocateInfo>) -> Result<(HaBufferRepository, Vec<BufferSubItem>), AllocatorError> {

        let staging_buffer_config = StagingBufferConfig::new(StagingBufferUsage::ImageCopySrc);
        let mut staging_buffer_items = vec![];

        for &index in self.info_indices.iter() {
            let mut config = staging_buffer_config.clone();
            let _ = config.add_item(infos[index].space);
            let item = self.staging_allocator.attach_staging_buffer(config)?.pop().unwrap();
            staging_buffer_items.push(item);
        }

        Ok((self.staging_allocator.allocate()?, staging_buffer_items))
    }

    fn upload_staging_data(&self, staging_repository: &mut HaBufferRepository, items: &[BufferSubItem], infos: &Vec<ImageAllocateInfo>) -> Result<(), AllocatorError> {

        let mut uploader = staging_repository.data_uploader()?;

        for (&info_index, buffer_item) in self.info_indices.iter().zip(items.iter()) {

            match infos[info_index].storage.source {
                | ImageSource::UploadData(ref source) => {
                    uploader.upload(buffer_item, &source.data)?;
                },
                | _ => panic!(),
            }
        }

        uploader.done()?;
        Ok(())
    }

    fn transfer_barrier(&self, info: &ImageAllocateInfo) -> vk::ImageMemoryBarrier {
        vk::ImageMemoryBarrier {
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
            image: info.image.handle,
            subresource_range: info.view_desc.subrange.clone(),
        }
    }

    fn final_barrier(&self, info: &ImageAllocateInfo) -> vk::ImageMemoryBarrier {
        vk::ImageMemoryBarrier {
            s_type: vk::StructureType::ImageMemoryBarrier,
            p_next: ptr::null(),
            src_access_mask: [AccessFlag::TransferWriteBit].flags(),
            dst_access_mask: [AccessFlag::ShaderReadBit].flags(),
            old_layout: ImageLayout::TransferDstOptimal.value(),
            new_layout: ImageLayout::ShaderReadOnlyOptimal.value(),
            src_queue_family_index : vk::VK_QUEUE_FAMILY_IGNORED,
            dst_queue_family_index : vk::VK_QUEUE_FAMILY_IGNORED,
            image: info.image.handle,
            subresource_range: info.view_desc.subrange.clone(),
        }
    }
}

//  Depth Stencil Image Barrier Bundle
struct DepSteImageBarrierBundle {

    info_indices: Vec<usize>,
    usage: DepthImageUsage,
}

impl ImageBarrierBundle for DepSteImageBarrierBundle {

    fn make_transfermation(&mut self, recorder: &HaCommandRecorder, infos: &Vec<ImageAllocateInfo>) -> Result<(), AllocatorError> {

        let final_barriers = self.info_indices.iter()
            .map(|&index| self.final_barrier(&infos[index])).collect::<Vec<_>>();

        let _ = recorder.pipeline_barrrier(
            PipelineStageFlag::TopOfPipeBit.value(),
            self.usage.dst_stage_flag().value(),
            &[], &[], &[],
            &final_barriers
        );

        Ok(())
    }

    fn cleanup(&mut self) {
        // nothing to clean, leave this func empty...
    }
}

impl DepSteImageBarrierBundle {

    fn new(usage: DepthImageUsage, indices: Vec<usize>) -> DepSteImageBarrierBundle {
        DepSteImageBarrierBundle {
            info_indices: indices, usage,
        }
    }

    fn final_barrier(&self, info: &ImageAllocateInfo) -> vk::ImageMemoryBarrier {
        vk::ImageMemoryBarrier {
            s_type: vk::StructureType::ImageMemoryBarrier,
            p_next: ptr::null(),
            src_access_mask: vk::AccessFlags::empty(),
            dst_access_mask: match self.usage {
                | DepthImageUsage::Attachment => [
                    AccessFlag::DepthStencilAttachmentReadBit,
                    AccessFlag::DepthStencilAttachmentWriteBit,
                ].flags(),
                | DepthImageUsage::ShaderRead(_format, _pipeline_stage) => {
                    // Not Test here
                    unimplemented!()
                },
            },
            old_layout: ImageLayout::Undefined.value(),
            new_layout: match self.usage {
                | DepthImageUsage::Attachment       => ImageLayout::DepthStencilAttachmentOptimal.value(),
                | DepthImageUsage::ShaderRead(_, _) => ImageLayout::DepthStencilReadOnlyOptimal.value(),
            },
            src_queue_family_index : vk::VK_QUEUE_FAMILY_IGNORED,
            dst_queue_family_index : vk::VK_QUEUE_FAMILY_IGNORED,
            image: info.image.handle,
            subresource_range: info.view_desc.subrange.clone(),
        }
    }
}

struct ImageAllocateInfo {

    type_: ImageVarietyType,

    image: HaImage,
    image_desc: ImageDescInfo,
    view_desc : ImageViewDescInfo,

    storage   : ImageStorageInfo,
    space     : vk::DeviceSize,
}

impl ImageAllocateInfo {

    fn new(type_: ImageVarietyType, storage: ImageStorageInfo, image: HaImage, image_desc: ImageDescInfo, view_desc: ImageViewDescInfo) -> ImageAllocateInfo {

        let space = bind_to_alignment(image.requirement.size, image.requirement.alignment);

        ImageAllocateInfo {
            type_, image, image_desc, view_desc, storage, space,
        }
    }

    fn generate_view(&self, device: &HaDevice) -> Result<HaImageView, ImageError> {

        let view = HaImageView::config(device, &self.image, &self.view_desc, self.storage.format)?;
        Ok(view)
    }

    fn cleanup(&self, device: &HaDevice) {
        unsafe {
            device.handle.destroy_image(self.image.handle, None);
        }
    }
}

fn collect_barrier_bundle(physical: &HaPhyDevice, device: &HaDevice, image_infos: &[ImageAllocateInfo]) -> Vec<Box<ImageBarrierBundle>> {

    let mut barrier_indices: HashMap<ImageVarietyType, Vec<usize>, RandomState> = HashMap::new();

    for (index, image_info) in image_infos.iter().enumerate() {

        // make the logic a little strange to avoid borrow conflict.
        let is_found = {
            if let Some(indices) = barrier_indices.get_mut(&image_info.type_) {
                indices.push(index);
                true
            } else {
                false
            }
        };

        if is_found == false {
            barrier_indices.insert(image_info.type_, vec![index]);
        }
    };

    let bundles = barrier_indices.into_iter().map(|(image_type, indices)| {

        match image_type {
            | ImageVarietyType::SampleImage(stage) => {
                Box::new(SampleImageBarrierBundle::new(physical, device, stage.clone(), indices)) as Box<ImageBarrierBundle>
            },
            | ImageVarietyType::DepthStencilImage(usage) => {
                Box::new(DepSteImageBarrierBundle::new(usage.clone(), indices)) as Box<ImageBarrierBundle>
            },
        }

    }).collect::<Vec<_>>();

    bundles
}

fn copy_buffer_to_image(recorder: &HaCommandRecorder, from_buffer: &BufferSubItem, to_image: &ImageAllocateInfo) -> Result<(), AllocatorError> {

    let subsource = &to_image.view_desc.subrange;
    let dimension = to_image.storage.dimension;
    // TODO: Only support one region.
    let copy_regions = [
        vk::BufferImageCopy {
            buffer_offset: from_buffer.offset,
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
        from_buffer.handle,
        to_image.image.handle,
        // TODO: make dst_layout configurable.
        vk::ImageLayout::TransferDstOptimal,
        &copy_regions
    );
    Ok(())
}