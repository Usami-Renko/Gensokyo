
use crate::error::GsResult;

use gsvk::core::device::GsDevice;
use gsvk::core::swapchain::GsChain;

use gsvk::pipeline::graphics::GfxPipelineConfig;
use gsvk::pipeline::graphics::{ GfxPipelineBuilder, GfxMultiPipelineBuilder, GfxPipelineSetBuilder };
use gsvk::pipeline::pass::{ GsRenderPass, RenderPassBuilder };
use gsvk::pipeline::pass::{ RenderAttachment, Present };
use gsvk::pipeline::pass::{ RenderDependency, SubpassStage };
use gsvk::pipeline::shader::{ GsShaderInfo, VertexInputDescription };

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

    pub fn gfx_builder(&self) -> GsResult<GfxPipelineBuilder> {

        Ok(GfxPipelineBuilder::new(&self.device)?)
    }

    pub fn gfx_multi_builder(&self) -> GsResult<GfxMultiPipelineBuilder> {

        Ok(GfxMultiPipelineBuilder::new(&self.device)?)
    }

    pub fn gfx_set_builder(&self, template: GfxPipelineConfig) -> GsResult<GfxPipelineSetBuilder> {

        Ok(GfxPipelineSetBuilder::new(&self.device, template)?)
    }

    pub fn pipeline_config(&self, shaders: impl Into<Vec<GsShaderInfo>>, input: VertexInputDescription, render_pass: GsRenderPass) -> GfxPipelineConfig {
        GfxPipelineConfig::new(shaders, input, render_pass, self.chain.dimension())
    }

    pub fn present_attachment(&self) -> RenderAttachment<Present> {
        RenderAttachment::setup(Present, self.chain.format())
    }

    pub fn subpass_dependency(&self, src: SubpassStage, dst: SubpassStage) -> RenderDependency {
        RenderDependency::setup(src, dst)
    }
}
