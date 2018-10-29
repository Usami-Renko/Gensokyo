
use ash::vk;

use core::device::HaDevice;
use core::physical::HaPhyDevice;

use pipeline::stages::PipelineStageFlag;
use pipeline::pass::AccessFlag;

use resources::allocator::{ HaBufferAllocator, BufferStorageType, ImageAllocateInfo };
use resources::buffer::{ HaImgsrcBlock, ImgsrcBlockInfo, BufferBlockEntity };
use resources::image::ImagePipelineStage;
use resources::image::{ ImageSource, ImageLayout };
use resources::image::ImageBarrierBundleAbs;
use resources::repository::{ HaBufferRepository, DataCopyer };
use resources::error::AllocatorError;

use utility::marker::{ VulkanEnum, VulkanFlags };

use std::ptr;

pub(crate) struct SampleImageBarrierBundle {

    info_indices: Vec<usize>,
    dst_stage: ImagePipelineStage,

    staging_allocator : HaBufferAllocator,
    staging_repository: Option<HaBufferRepository>,
}

impl ImageBarrierBundleAbs for SampleImageBarrierBundle {

    fn make_transfermation(&mut self, copyer: &DataCopyer, infos: &mut Vec<ImageAllocateInfo>) -> Result<(), AllocatorError> {

        // create staging buffer and memories
        let (mut staging_repository, buffer_blocks) = self.create_staging_repository(infos)?;
        // send textures to the staging buffer
        self.upload_staging_data(&mut staging_repository, &buffer_blocks, infos)?;

        // make image barrier transition for data transfer.
        let transfer_barriers = self.info_indices.iter()
            .map(|&index| self.transfer_barrier(&mut infos[index])).collect::<Vec<_>>();
        copyer.recorder().pipeline_barrrier(
            PipelineStageFlag::TopOfPipeBit.value(),
            PipelineStageFlag::TransferBit.value(),
            &[], // dependencies specifying how execution and memory dependencies are formed.
            &[],
            &[],
            &transfer_barriers
        );

        // copy buffer to image.
        for (i, &index) in self.info_indices.iter().enumerate() {
            copyer.copy_buffer_to_image(buffer_blocks[i].get_buffer_item(), &infos[index]);
        }

        // make image barrier transition for final layout.
        let final_barriers = self.info_indices.iter()
            .map(|&index| self.final_barrier(&mut infos[index])).collect::<Vec<_>>();
        let _ = copyer.recorder().pipeline_barrrier(
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

    fn create_staging_repository(&mut self, infos: &Vec<ImageAllocateInfo>) -> Result<(HaBufferRepository, Vec<HaImgsrcBlock>), AllocatorError> {

        let mut staging_buffers = vec![];

        for &index in self.info_indices.iter() {
            let img_info = ImgsrcBlockInfo::new(infos[index].space);
            let staging_buffer = self.staging_allocator.append_imgsrc(img_info)?;
            staging_buffers.push(staging_buffer);
        }

        Ok((self.staging_allocator.allocate()?, staging_buffers))
    }

    fn upload_staging_data(&self, staging_repository: &mut HaBufferRepository, img_data_blocks: &[HaImgsrcBlock], infos: &Vec<ImageAllocateInfo>) -> Result<(), AllocatorError> {

        let mut uploader = staging_repository.data_uploader()?;

        for (&info_index, img_block) in self.info_indices.iter().zip(img_data_blocks.iter()) {

            match infos[info_index].storage.source {
                | ImageSource::UploadData(ref source) => {
                    uploader.upload(img_block, &source.data)?;
                },
                | _ => panic!(),
            }
        }

        uploader.done()?;

        Ok(())
    }

    fn transfer_barrier(&self, info: &mut ImageAllocateInfo) -> vk::ImageMemoryBarrier {

        let new_layout = ImageLayout::TransferDstOptimal;

        let barrier = vk::ImageMemoryBarrier {
            s_type: vk::StructureType::ImageMemoryBarrier,
            p_next: ptr::null(),
            src_access_mask: vk::AccessFlags::empty(),
            dst_access_mask: [AccessFlag::TransferWriteBit].flags(),
            old_layout: info.image_desc.initial_layout,
            new_layout: new_layout.value(),
            // TODO: Current ignore queue family ownership transfer.
            // srcQueueFamilyIndex is the source queue family for a queue family ownership transfer.
            src_queue_family_index: vk::VK_QUEUE_FAMILY_IGNORED,
            // dstQueueFamilyIndex is the destination queue family for a queue family ownership transfer.
            dst_queue_family_index: vk::VK_QUEUE_FAMILY_IGNORED,
            image: info.image.handle,
            subresource_range: info.view_desc.subrange.clone(),
        };

        info.final_layout = new_layout;

        barrier
    }

    fn final_barrier(&self, info: &mut ImageAllocateInfo) -> vk::ImageMemoryBarrier {

        let new_layout = ImageLayout::ShaderReadOnlyOptimal;

        let barrier = vk::ImageMemoryBarrier {
            s_type: vk::StructureType::ImageMemoryBarrier,
            p_next: ptr::null(),
            src_access_mask: [AccessFlag::TransferWriteBit].flags(),
            dst_access_mask: [AccessFlag::ShaderReadBit].flags(),
            old_layout: info.final_layout.value(),
            new_layout: new_layout.value(),
            src_queue_family_index: vk::VK_QUEUE_FAMILY_IGNORED,
            dst_queue_family_index: vk::VK_QUEUE_FAMILY_IGNORED,
            image: info.image.handle,
            subresource_range: info.view_desc.subrange.clone(),
        };

        info.final_layout = new_layout;

        barrier
    }
}
