
use ash::vk;

use crate::core::GsDevice;

use crate::image::barrier::GsImageBarrier;
use crate::image::allocator::ImageAllotInfo;
use crate::image::instance::traits::ImageBarrierBundleAbs;

use crate::memory::transfer::DataCopyer;
use crate::command::GsCmdCopyApi;
use crate::error::VkResult;

//  Depth Stencil Image Barrier Bundle
pub struct DSImageBarrierBundle {

    info_indices: Vec<usize>,
}

impl ImageBarrierBundleAbs for DSImageBarrierBundle {

    fn make_barrier_transform(&mut self, _device: &GsDevice, copyer: &DataCopyer, infos: &mut Vec<ImageAllotInfo>) -> VkResult<()> {

        let final_barriers = self.info_indices.iter()
            .map(|&index| self.final_barrier(&mut infos[index])).collect();

        let _ = copyer.recorder().image_pipeline_barrier(
            vk::PipelineStageFlags::TOP_OF_PIPE,
            vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
            vk::DependencyFlags::empty(),
            final_barriers
        );

        Ok(())
    }
}

impl DSImageBarrierBundle {

    pub fn new(indices: Vec<usize>) -> DSImageBarrierBundle {
        DSImageBarrierBundle {
            info_indices: indices,
        }
    }

    fn final_barrier(&self, info: &mut ImageAllotInfo) -> GsImageBarrier {

        info.final_layout = vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL;

        GsImageBarrier::new(&info.image, info.view_desc.subrange)
            .access_mask(
                vk::AccessFlags::empty(),
                vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_READ | vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE
            )
            .layout(info.image_desc.property.initial_layout, info.final_layout)
            .build()
    }
}
