
use core::device::HaDevice;

use pipeline::graphics::GraphicsPipelineBuilder;
use pipeline::pass::RenderPassBuilder;
use pipeline::stages::PipelineType;

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

    pub fn pipeline_builder(&self, type_: PipelineType) -> Result<GraphicsPipelineBuilder, PipelineError> {

        match type_ {
            | PipelineType::Graphics => GraphicsPipelineBuilder::new(&self.device),
            | PipelineType::Compute  => unimplemented!()
        }
    }
}
