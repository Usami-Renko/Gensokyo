
use core::device::HaDevice;

use pipeline::graphics::GraphicsPipelineBuilder;
use pipeline::pass::RenderPassBuilder;

use pipeline::error::PipelineError;

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

    pub fn graphics_pipeline_builder(&self) -> Result<GraphicsPipelineBuilder, PipelineError> {

        GraphicsPipelineBuilder::new(&self.device)
    }
}
