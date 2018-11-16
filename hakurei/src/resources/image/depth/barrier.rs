
use vk::core::physical::HaPhyDevice;
use vk::core::device::HaDevice;

use vk::pipeline::stages::PipelineStageFlag;
use vk::pipeline::pass::AccessFlag;

use vk::resources::image::{ HaImageBarrier, ImageLayout };
use vk::resources::transfer::DataCopyer;
use vk::resources::error::AllocatorError;

use resources::allocator::image::ImageAllocateInfo;
use resources::image::traits::ImageBarrierBundleAbs;

//  Depth Stencil Image Barrier Bundle
pub struct DepSteImageBarrierBundle {

    info_indices: Vec<usize>,
}

impl ImageBarrierBundleAbs for DepSteImageBarrierBundle {

    fn make_transfermation(&mut self, physical: &HaPhyDevice, device: &HaDevice, copyer: &DataCopyer, infos: &mut Vec<ImageAllocateInfo>) -> Result<(), AllocatorError> {

        let final_barriers = self.info_indices.iter()
            .map(|&index| self.final_barrier(&mut infos[index])).collect();

        let _ = copyer.recorder().image_pipeline_barrrier(
            PipelineStageFlag::TopOfPipeBit,
            PipelineStageFlag::EarlyFragmentTestsBit,
            &[],
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

    fn final_barrier(&self, info: &mut ImageAllocateInfo) -> HaImageBarrier {

        info.final_layout = ImageLayout::DepthStencilAttachmentOptimal;

        HaImageBarrier::new(&info.image, &info.view_desc.subrange)
            .access_mask(&[], &[
                AccessFlag::DepthStencilAttachmentReadBit,
                AccessFlag::DepthStencilAttachmentWriteBit,
            ])
            .layout(info.image_desc.initial_layout, info.final_layout)
            .build()
    }
}
