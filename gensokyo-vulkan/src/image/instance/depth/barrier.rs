
use ash::vk;

use core::physical::GsPhyDevice;
use core::device::GsDevice;

use image::barrier::GsImageBarrier;
use image::allocator::ImageAllocateInfo;
use image::instance::traits::ImageBarrierBundleAbs;
use memory::transfer::DataCopyer;
use memory::AllocatorError;

//  Depth Stencil Image Barrier Bundle
pub struct DepSteImageBarrierBundle {

    info_indices: Vec<usize>,
}

impl ImageBarrierBundleAbs for DepSteImageBarrierBundle {

    fn make_transfermation(&mut self, _physical: &GsPhyDevice, _device: &GsDevice, copyer: &DataCopyer, infos: &mut Vec<ImageAllocateInfo>) -> Result<(), AllocatorError> {

        let final_barriers = self.info_indices.iter()
            .map(|&index| self.final_barrier(&mut infos[index])).collect();

        let _ = copyer.recorder().image_pipeline_barrrier(
            vk::PipelineStageFlags::TOP_OF_PIPE,
            vk::PipelineStageFlags::EARLY_FRAGMENT_TESTS,
            vk::DependencyFlags::empty(),
            final_barriers
        );

        Ok(())
    }

    fn cleanup(&mut self) {
        // nothing to clean, leave this func empty...
    }
}

impl DepSteImageBarrierBundle {

    pub fn new(indices: Vec<usize>) -> DepSteImageBarrierBundle {
        DepSteImageBarrierBundle {
            info_indices: indices,
        }
    }

    fn final_barrier(&self, info: &mut ImageAllocateInfo) -> GsImageBarrier {

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
