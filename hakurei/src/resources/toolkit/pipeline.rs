
use core::device::HaDevice;

use pipeline::graphics::GraphicsPipelineBuilder;
use pipeline::pass::RenderPassBuilder;

pub struct PipelineKit {

    device: HaDevice,
}

impl PipelineKit {

    pub fn init(device: &HaDevice) -> PipelineKit {
        PipelineKit {
            device  : device.clone(),
        }
    }

    pub fn pass_builder(&self) -> RenderPassBuilder {

        RenderPassBuilder::new(&self.device)
    }

    pub fn graphics_pipeline_builder(&self) -> GraphicsPipelineBuilder {

        GraphicsPipelineBuilder::new(&self.device)
    }
}
