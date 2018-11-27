
use gsvk::core::device::HaDevice;

use gsvk::pipeline::graphics::GraphicsPipelineBuilder;
use gsvk::pipeline::pass::RenderPassBuilder;

use gsvk::pipeline::error::PipelineError;

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

    pub fn pipeline_graphics_builder(&self) -> Result<GraphicsPipelineBuilder, PipelineError> {

        GraphicsPipelineBuilder::new(&self.device)
    }
}
