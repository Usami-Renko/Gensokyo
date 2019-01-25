
use ash::vk;
use ash::version::DeviceV1_0;

use crate::command::record::{ GsCmdRecorder, GsVkCommandType };
use crate::command::infos::{ CmdViewportInfo, CmdScissorInfo, CmdDepthBiasInfo, CmdDepthBoundInfo };
use crate::command::traits::CmdPipelineAbs;

use crate::pipeline::target::{ GsPipelineStage, GsVkPipelineType };
use crate::descriptor::DescriptorSet;
use crate::buffer::instance::{ GsVertexBuffer, GsIndexBuffer };
use crate::utils::phantom::Graphics;
use crate::types::{ vkuint, vksint, vkfloat, vkbytes };

use gsma::collect_handle;

impl GsVkCommandType for Graphics {
    // Empty...
}

pub trait GsCmdGraphicsApi {

    fn begin_render_pass(&self, pipeline: &impl CmdPipelineAbs, framebuffer_index: usize) -> &Self;

    /// Set the viewport dynamically.
    /// Before using this function, the `ViewportStateType::Dynamic` or `ViewportStateType::DynamicViewportFixedScissor` must be set to ViewportState in pipeline creation(by calling `GraphicsPipelineConfig::setup_viewport()`).
    ///
    /// `first_viewport` is the index of the first viewport whose parameters are updated by the command.
    ///
    /// `viewports` specifies the new value to use as viewports.
    fn set_viewport(&self, first_viewport: vkuint, viewports: &[CmdViewportInfo]) -> &Self;

    /// Set the scissor rectangles dynamically.
    /// Before using this function, the `ViewportStateType::Dynamic` or `ViewportStateType::FixedViewportDynamicScissor` must be set to ViewportState in pipeline creation(by calling `GraphicsPipelineConfig::setup_viewport()`).
    ///
    /// `first_scissor` is the index of the first scissor whose state is updated by the command.
    ///
    /// `scissors` specifies the new value to use as scissor rectangles.
    fn set_scissor(&self, first_scissor: vkuint, scissors: &[CmdScissorInfo]) -> &Self;

    /// Set the line width dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `GsRasterizerState::set_line_width()` on RasterizerState during pipeline creation.
    ///
    /// `width` specifies the new value to use as the width of rasterized line segments.
    fn set_line_width(&self, width: vkfloat) -> &Self;

    /// Set the depth bias dynamically.
   /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `GsRasterizerState::set_depth_bias()` on RasterizerState during pipeline creation.
   ///
   /// `bias` specifies the new value to use as depth bias.
    fn set_depth_bias(&self, bias: CmdDepthBiasInfo) -> &Self;

    /// Set the blend constants dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `GsBlendState::set_blend_constants()` on BlendState during pipeline creation.
    ///
    /// `constants` specifies the new value to use as blend constants.
    fn set_blend_constants(&self, constants: [vkfloat; 4]) -> &Self;

    /// Set the depth bound dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `DepthTest::set_depth_bound()` on DepthStencilState during pipeline creation.
    ///
    /// `bound` specifies the new value to use as depth bound.
    fn set_depth_bound(&self, bound: CmdDepthBoundInfo) -> &Self;

    /// Set the stencil compare mask dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `StencilTest::set_compare_mask()` on DepthStencilState during pipeline creation.
    ///
    /// `face` specifies the set of stencil state for which to update the compare mask.
    ///
    /// `mask` specifies the new value to use as the stencil compare mask.
    fn set_stencil_compare_mask(&self, face: vk::StencilFaceFlags, mask: vkuint) -> &Self;

    /// Set the stencil write mask dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `StencilTest::set_write_mask()` on DepthStencilState during pipeline creation.
    ///
    /// `face` specifies the set of stencil state for which to update the write mask.
    ///
    /// `mask` specifies the new value to use as the stencil write mask.
    fn set_stencil_write_mask(&self, face: vk::StencilFaceFlags, mask: vkuint) -> &Self;

    /// Set the stencil reference dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `StencilTest::set_reference()` on DepthStencilState during pipeline creation.
    ///
    /// `face` specifies the set of stencil state for which to update the reference value.
    ///
    /// `reference` specifies the set of stencil state for which to update the reference value.
    fn set_stencil_reference(&self, face: vk::StencilFaceFlags, reference: vkuint) -> &Self;

    fn push_constants(&self, stage: GsPipelineStage, offset: vkuint, data: &[u8]) -> &Self;

    fn bind_pipeline(&self) -> &Self;

    /// `first_binding` is correspond to `vk::VertexInputBindingDescription.binding` value.
    fn bind_vertex_buffers(&self, first_binding: vkuint, buffers: &[&GsVertexBuffer]) -> &Self;

    fn bind_index_buffer(&self, buffer: &GsIndexBuffer, offset: vkbytes) -> &Self;

    fn bind_descriptor_sets(&self, first_set: vkuint, sets: &[&DescriptorSet]) -> &Self;

    fn bind_descriptor_sets_dynamic(&self, first_set: vkuint, sets: &[&DescriptorSet], dynamics: &[vkuint]) -> &Self;

    fn draw(&self, vertex_count: vkuint, instance_count: vkuint, first_vertex: vkuint, first_instance: vkuint) -> &Self;

    fn draw_indexed(&self, index_count: vkuint, instance_count: vkuint, first_index: vkuint, vertex_offset: vksint, first_instance: vkuint) -> &Self;

    fn end_render_pass(&self) -> &Self;
}

impl GsCmdGraphicsApi for GsCmdRecorder<Graphics> {

    fn begin_render_pass(&self, pipeline: &impl CmdPipelineAbs, framebuffer_index: usize) -> &Self {

        let begin_info = pipeline.render_pass().begin_info(framebuffer_index);
        unsafe {
            self.device.handle.cmd_begin_render_pass(self.cmd_handle, &begin_info, self.cmd_usage.contents());
        } self
    }

    fn set_viewport(&self, first_viewport: vkuint, viewports: &[CmdViewportInfo]) -> &Self {

        let ports: Vec<vk::Viewport> = viewports.iter()
            .map(|p| p.0).collect();
        unsafe {
            self.device.handle.cmd_set_viewport(self.cmd_handle, first_viewport, &ports);
        } self
    }

    fn set_scissor(&self, first_scissor: vkuint, scissors: &[CmdScissorInfo]) -> &Self {

        let scissors: Vec<vk::Rect2D> = scissors.iter()
            .map(|s| s.0).collect();
        unsafe {
            self.device.handle.cmd_set_scissor(self.cmd_handle, first_scissor, &scissors);
        } self
    }

    fn set_line_width(&self, width: vkfloat) -> &Self {
        unsafe {
            self.device.handle.cmd_set_line_width(self.cmd_handle, width);
        } self
    }

    fn set_depth_bias(&self, bias: CmdDepthBiasInfo) -> &Self {
        unsafe {
            self.device.handle.cmd_set_depth_bias(self.cmd_handle, bias.constant_factor, bias.clamp, bias.slope_factor);
        } self
    }

    fn set_blend_constants(&self, constants: [vkfloat; 4]) -> &Self {
        unsafe {
            self.device.handle.cmd_set_blend_constants(self.cmd_handle, constants);
        } self
    }

    fn set_depth_bound(&self, bound: CmdDepthBoundInfo) -> &Self {
        unsafe {
            self.device.handle.cmd_set_depth_bounds(self.cmd_handle, bound.min_bound, bound.max_bound);
        } self
    }

    fn set_stencil_compare_mask(&self, face: vk::StencilFaceFlags, mask: vkuint) -> &Self {
        unsafe {
            self.device.handle.cmd_set_stencil_compare_mask(self.cmd_handle, face, mask);
        } self
    }

    fn set_stencil_write_mask(&self, face: vk::StencilFaceFlags, mask: vkuint) -> &Self {
        unsafe {
            self.device.handle.cmd_set_stencil_write_mask(self.cmd_handle, face, mask);
        } self
    }

    fn set_stencil_reference(&self, face: vk::StencilFaceFlags, reference: vkuint) -> &Self {
        unsafe {
            self.device.handle.cmd_set_stencil_reference(self.cmd_handle, face, reference);
        } self
    }

    fn push_constants(&self, stage: GsPipelineStage, offset: vkuint, data: &[u8]) -> &Self {
        unsafe {
            self.device.handle.cmd_push_constants(self.cmd_handle, self.pipeline_layout, stage.0, offset, data);
        } self
    }

    fn bind_pipeline(&self) -> &Self {
        unsafe {
            self.device.handle.cmd_bind_pipeline(self.cmd_handle, Graphics::BIND_POINT, self.pipeline_handle);
        } self
    }

    fn bind_vertex_buffers(&self, first_binding: vkuint, buffers: &[&GsVertexBuffer]) -> &Self {

        let mut handles = vec![];
        let mut offsets = vec![];

        for block in buffers.into_iter() {

            handles.push(block.render_info());
            // TODO: Add configuration for offset parameter.
            offsets.push(0);
        }

        unsafe {
            self.device.handle.cmd_bind_vertex_buffers(self.cmd_handle, first_binding, &handles, &offsets);
        } self
    }

    fn bind_index_buffer(&self, buffer: &GsIndexBuffer, offset: vkbytes) -> &Self {

        let (indices_handle, indices_type) = buffer.render_info();
        unsafe {
            self.device.handle.cmd_bind_index_buffer(self.cmd_handle, indices_handle, offset, indices_type);
        } self
    }

    fn bind_descriptor_sets(&self, first_set: vkuint, sets: &[&DescriptorSet]) -> &Self {

        let handles = collect_handle!(sets, entity);
        unsafe {
            self.device.handle.cmd_bind_descriptor_sets(self.cmd_handle, Graphics::BIND_POINT, self.pipeline_layout, first_set, &handles, &[]);
        } self
    }

    fn bind_descriptor_sets_dynamic(&self, first_set: vkuint, sets: &[&DescriptorSet], dynamics: &[vkuint]) -> &Self {

        let handles = collect_handle!(sets, entity);
        unsafe {
            self.device.handle.cmd_bind_descriptor_sets(self.cmd_handle, Graphics::BIND_POINT, self.pipeline_layout, first_set, &handles, dynamics);
        } self
    }

    fn draw(&self, vertex_count: vkuint, instance_count: vkuint, first_vertex: vkuint, first_instance: vkuint) -> &Self {
        unsafe {
            self.device.handle.cmd_draw(self.cmd_handle, vertex_count, instance_count, first_vertex, first_instance);
        } self
    }

    fn draw_indexed(&self, index_count: vkuint, instance_count: vkuint, first_index: vkuint, vertex_offset: vksint, first_instance: vkuint) -> &Self {
        unsafe {
            self.device.handle.cmd_draw_indexed(self.cmd_handle, index_count, instance_count, first_index, vertex_offset, first_instance);
        } self
    }

    fn end_render_pass(&self) -> &Self {
        // Ending the render pass will add an implicit barrier transitioning the frame buffer color attachment vk::IMAGE_LAYOUT_PRESENT_SRC_KHR for presenting it to the windowing system.
        unsafe {
            self.device.handle.cmd_end_render_pass(self.cmd_handle);
        } self
    }
}
