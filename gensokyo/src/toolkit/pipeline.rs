
use gsvk::core::device::GsDevice;
use gsvk::core::swapchain::GsChain;

use gsvk::pipeline::graphics::{ GraphicsPipelineBuilder, GraphicsPipelineConfig };
use gsvk::pipeline::pass::{ GsRenderPass, RenderPassBuilder };
use gsvk::pipeline::pass::{ RenderAttachement, RenderAttachementPrefab };
use gsvk::pipeline::pass::{ RenderDependency, SubpassStage };
use gsvk::pipeline::shader::{ GsShaderInfo, VertexInputDescription };

use gsvk::pipeline::error::PipelineError;

pub struct PipelineKit {

    device: GsDevice,
    chain : GsChain,
}

impl PipelineKit {

    pub(crate) fn init(device: &GsDevice, chain: &GsChain) -> PipelineKit {

        PipelineKit {
            device : device.clone(),
            chain  : chain.clone(),
        }
    }

    pub fn pass_builder(&self) -> RenderPassBuilder {

        RenderPassBuilder::new(&self.device, &self.chain)
    }

    pub fn graphics_pipeline_builder(&self) -> Result<GraphicsPipelineBuilder, PipelineError> {

        GraphicsPipelineBuilder::new(&self.device)
    }

    pub fn pipeline_config(&self, shaders: impl Into<Vec<GsShaderInfo>>, input: VertexInputDescription, render_pass: GsRenderPass) -> GraphicsPipelineConfig {
        GraphicsPipelineConfig::new(shaders, input, render_pass, self.chain.extent())
    }

    pub fn present_attachment(&self) -> RenderAttachement {
        RenderAttachement::setup(RenderAttachementPrefab::PresentAttachment, self.chain.format())
    }

    pub fn subpass_dependency(&self, src: SubpassStage, dst: SubpassStage) -> RenderDependency {
        RenderDependency::setup(src, dst)
    }
}
