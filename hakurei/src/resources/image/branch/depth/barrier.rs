
use ash::vk;

use pipeline::stages::PipelineStageFlag;
use pipeline::pass::AccessFlag;

use resources::allocator::ImageAllocateInfo;
use resources::image::DepthImageUsage;
use resources::image::ImageLayout;
use resources::image::ImageBarrierBundleAbs;
use resources::repository::DataCopyer;
use resources::error::AllocatorError;

use utility::marker::{ VulkanFlags, VulkanEnum };

use std::ptr;

//  Depth Stencil Image Barrier Bundle
pub(crate) struct DepSteImageBarrierBundle {

    info_indices: Vec<usize>,
    usage: DepthImageUsage,
}

impl ImageBarrierBundleAbs for DepSteImageBarrierBundle {

    fn make_transfermation(&mut self, copyer: &DataCopyer, infos: &Vec<ImageAllocateInfo>) -> Result<(), AllocatorError> {

        let final_barriers = self.info_indices.iter()
            .map(|&index| self.final_barrier(&infos[index])).collect::<Vec<_>>();

        let _ = copyer.recorder().pipeline_barrrier(
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

    pub fn new(usage: DepthImageUsage, indices: Vec<usize>) -> DepSteImageBarrierBundle {
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
