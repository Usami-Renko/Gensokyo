
use ash::vk;

use core::device::HaDevice;
use core::physical::HaPhyDevice;

use pipeline::stages::PipelineStageFlag;
use pipeline::pass::AccessFlag;

use resources::allocator::{ HaBufferAllocator, BufferStorageType, ImageAllocateInfo };
use resources::buffer::{ BufferSubItem, StagingBufferConfig, StagingBufferUsage };
use resources::buffer::BufferConfigModifiable;
use resources::image::ImagePipelineStage;
use resources::image::{ ImageSource, ImageLayout };
use resources::image::ImageBarrierBundleAbs;
use resources::command::HaCommandRecorder;
use resources::repository::HaBufferRepository;
use resources::error::AllocatorError;

use utility::marker::{ VulkanEnum, VulkanFlags };

use std::ptr;

pub(crate) struct SampleImageBarrierBundle {

    info_indices: Vec<usize>,
    dst_stage   : ImagePipelineStage,

    staging_allocator : HaBufferAllocator,
    staging_repository: Option<HaBufferRepository>,
}

impl ImageBarrierBundleAbs for SampleImageBarrierBundle {

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

    pub fn new(physical: &HaPhyDevice, device: &HaDevice, dst_stage: ImagePipelineStage, indices: Vec<usize>) -> SampleImageBarrierBundle {
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