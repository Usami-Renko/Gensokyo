
use gsvk::core::device::GsDevice;

use gsvk::pipeline::graphics::GraphicsPipelineBuilder;
use gsvk::pipeline::pass::RenderPassBuilder;

use gsvk::pipeline::error::PipelineError;

pub struct PipelineKit {

    device: GsDevice,
}

impl PipelineKit {

    pub fn init(device: &GsDevice) -> PipelineKit {
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
