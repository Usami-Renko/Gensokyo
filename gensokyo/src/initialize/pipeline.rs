
use crate::error::GsResult;

use gsvk::pipeline::graphics::GfxPipelineConfig;
use gsvk::pipeline::graphics::{ GfxPipelineBuilder, GfxMultiPipelineBuilder, GfxPipelineSetBuilder };
use gsvk::pipeline::pass::{ GsRenderPass, RenderPassBuilder, RenderAttachmentCI, Present };

use crate::initialize::initializer::AssetInitializer;
use crate::initialize::traits::{ FromInitializer, TryFromInitializer, TryFromInitializerP1 };

impl TryFromInitializer for GfxPipelineBuilder {

    fn new(initializer: &AssetInitializer) -> GsResult<GfxPipelineBuilder> {
        Ok(GfxPipelineBuilder::create(&initializer.device)?)
    }
}

impl TryFromInitializer for GfxMultiPipelineBuilder {

    fn new(initializer: &AssetInitializer) -> GsResult<GfxMultiPipelineBuilder> {
        Ok(GfxMultiPipelineBuilder::create(&initializer.device)?)
    }
}

impl TryFromInitializerP1<GfxPipelineConfig> for GfxPipelineSetBuilder {

    fn new(initializer: &AssetInitializer, template: GfxPipelineConfig) -> GsResult<GfxPipelineSetBuilder> {
        Ok(GfxPipelineSetBuilder::create(&initializer.device, template)?)
    }
}

impl FromInitializer for RenderAttachmentCI<Present> {

    fn new(initializer: &AssetInitializer) -> RenderAttachmentCI<Present> {
        RenderAttachmentCI::create(Present, initializer.swapchain.format())
    }
}

impl FromInitializer<RenderPassBuilder> for GsRenderPass {

    fn new(initializer: &AssetInitializer) -> RenderPassBuilder {
        GsRenderPass::builder(&initializer.device, &initializer.swapchain)
    }
}
