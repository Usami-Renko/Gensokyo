
use vk::core::device::HaDevice;

use vk::pipeline::graphics::GraphicsPipelineBuilder;
use vk::pipeline::pass::RenderPassBuilder;
use vk::pipeline::stages::PipelineType;

use vk::pipeline::error::PipelineError;

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

    pub fn pipeline_builder(&self, typ: PipelineType) -> Result<GraphicsPipelineBuilder, PipelineError> {

        match typ {
            | PipelineType::Graphics => GraphicsPipelineBuilder::new(&self.device),
            | PipelineType::Compute  => unimplemented!()
        }
    }
}
