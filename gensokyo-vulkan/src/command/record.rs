
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::device::GsDevice;

use crate::command::buffer::{ GsCommandBuffer, CmdBufferUsage };
use crate::command::infos::{ CmdViewportInfo, CmdScissorInfo, CmdDepthBiasInfo, CmdDepthBoundInfo };
use crate::command::infos::CmdDescriptorSetBindInfo;
use crate::command::traits::IntoVKBarrier;
use crate::command::error::CommandError;

use crate::buffer::instance::{ GsVertexBuffer, GsIndexBuffer };
use crate::image::GsImageBarrier;
use crate::pipeline::graphics::GsGraphicsPipeline;

use crate::types::{ vkuint, vksint, vkfloat, vkbytes };

use std::ptr;

pub struct GsCommandRecorder {

    buffer: Option<GsCommandBuffer>,
    handle: vk::CommandBuffer,
    device: GsDevice,
    usage : CmdBufferUsage,
}

impl GsCommandRecorder {

    pub fn new(device: &GsDevice, command: GsCommandBuffer) -> GsCommandRecorder {

        let handle = command.handle;
        let usage = command.usage;

        GsCommandRecorder {
            buffer: Some(command),
            device: device.clone(),
            handle,
            usage,
        }
    }

    // TODO: Add configuration for vk::CommandBufferUsageFlags.
    pub fn begin_record(&self, flags: vk::CommandBufferUsageFlags) -> Result<&GsCommandRecorder, CommandError> {

        let begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::COMMAND_BUFFER_BEGIN_INFO,
            p_next: ptr::null(),
            flags,
            // TODO: Add configuration for secondary command buffer
            // Inheritance_info is used if commandBuffer is a secondary command buffer.
            // If this is a primary command buffer, then this value is ignored.
            p_inheritance_info: ptr::null(),
        };

        unsafe {
            self.device.handle.begin_command_buffer(self.handle, &begin_info)
                .or(Err(CommandError::RecordBeginError))?
        };

        Ok(self)
    }


    pub fn begin_render_pass(&self, pipeline: &GsGraphicsPipeline, framebuffer_index: usize) -> &GsCommandRecorder {

        let begin_info = pipeline.pass.begin_info(framebuffer_index);

        unsafe {
            self.device.handle.cmd_begin_render_pass(self.handle, &begin_info, self.usage.contents());
        }
        self
    }

    /// Set the viewport dynamically.
    /// Before using this function, the `ViewportStateType::Dynamic` or `ViewportStateType::DynamicViewportFixedScissor` must be set to ViewportState in pipeline creation(by calling `GraphicsPipelineConfig::setup_viewport()`).
    ///
    /// `first_viewport` is the index of the first viewport whose parameters are updated by the command.
    ///
    /// `viewports` specifies the new value to use as viewports.
    pub fn set_viewport(&self, first_viewport: vkuint, viewports: &[CmdViewportInfo]) -> &GsCommandRecorder {

        let ports: Vec<vk::Viewport> = viewports.iter()
            .map(|p| p.content).collect();
        unsafe {
            self.device.handle.cmd_set_viewport(self.handle, first_viewport, &ports)
        };
        self
    }

    /// Set the scissor rectangles dynamically.
    /// Before using this function, the `ViewportStateType::Dynamic` or `ViewportStateType::FixedViewportDynamicScissor` must be set to ViewportState in pipeline creation(by calling `GraphicsPipelineConfig::setup_viewport()`).
    ///
    /// `first_scissor` is the index of the first scissor whose state is updated by the command.
    ///
    /// `scissors` specifies the new value to use as scissor rectangles.
    pub fn set_scissor(&self, first_scissor: vkuint, scissors: &[CmdScissorInfo]) -> &GsCommandRecorder {

        let scissors: Vec<vk::Rect2D> = scissors.iter()
            .map(|s| s.content).collect();
        unsafe {
            self.device.handle.cmd_set_scissor(self.handle, first_scissor, &scissors)
        };
        self
    }

    /// Set the line width dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `GsRasterizerState::set_line_width()` on RasterizerState during pipeline creation.
    ///
    /// `width` specifies the new value to use as the width of rasterized line segments.
    pub fn set_line_width(&self, width: vkfloat) -> &GsCommandRecorder {
        unsafe {
            self.device.handle.cmd_set_line_width(self.handle, width)
        };
        self
    }

    /// Set the depth bias dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `GsRasterizerState::set_depth_bias()` on RasterizerState during pipeline creation.
    ///
    /// `bias` specifies the new value to use as depth bias.
    pub fn set_depth_bias(&self, bias: CmdDepthBiasInfo) -> &GsCommandRecorder {
        unsafe {
            self.device.handle.cmd_set_depth_bias(self.handle, bias.constant_factor, bias.clamp, bias.slope_factor)
        };
        self
    }

    /// Set the blend constants dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `GsBlendState::set_blend_constants()` on BlendState during pipeline creation.
    ///
    /// `constants` specifies the new value to use as blend constants.
    pub fn set_blend_constants(&self, constants: [vkfloat; 4]) -> &GsCommandRecorder {
        unsafe {
            self.device.handle.cmd_set_blend_constants(self.handle, constants)
        };
        self
    }

    /// Set the depth bound dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `DepthTest::set_depth_bound()` on DepthStencilState during pipeline creation.
    ///
    /// `bound` specifies the new value to use as depth bound.
    pub fn set_depth_bound(&self, bound: CmdDepthBoundInfo) -> &GsCommandRecorder {
        unsafe {
            self.device.handle.cmd_set_depth_bounds(self.handle, bound.min_bound, bound.max_bound)
        };
        self
    }

    /// Set the stencil compare mask dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `StencilTest::set_compare_mask()` on DepthStencilState during pipeline creation.
    ///
    /// `face` specifies the set of stencil state for which to update the compare mask.
    ///
    /// `mask` specifies the new value to use as the stencil compare mask.
    pub fn set_stencil_compare_mask(&self, face: vk::StencilFaceFlags, mask: vkuint) -> &GsCommandRecorder {
        unsafe {
            self.device.handle.cmd_set_stencil_compare_mask(self.handle, face, mask)
        };
        self
    }

    /// Set the stencil write mask dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `StencilTest::set_write_mask()` on DepthStencilState during pipeline creation.
    ///
    /// `face` specifies the set of stencil state for which to update the write mask.
    ///
    /// `mask` specifies the new value to use as the stencil write mask.
    pub fn set_stencil_write_mask(&self, face: vk::StencilFaceFlags, mask: vkuint) -> &GsCommandRecorder {
        unsafe {
            self.device.handle.cmd_set_stencil_write_mask(self.handle, face, mask)
        };
        self
    }

    /// Set the stencil reference dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `StencilTest::set_reference()` on DepthStencilState during pipeline creation.
    ///
    /// `face` specifies the set of stencil state for which to update the reference value.
    ///
    /// `reference` specifies the set of stencil state for which to update the reference value.
    pub fn set_stencil_reference(&self, face: vk::StencilFaceFlags, reference: vkuint) -> &GsCommandRecorder {
        unsafe {
            self.device.handle.cmd_set_stencil_reference(self.handle, face, reference)
        };
        self
    }

    pub fn bind_pipeline(&self, pipeline: &GsGraphicsPipeline) -> &GsCommandRecorder {

        unsafe {
            self.device.handle.cmd_bind_pipeline(self.handle, pipeline.bind_point(), pipeline.handle)
        };

        self
    }

    pub fn bind_vertex_buffers(&self, first_binding: vkuint, buffers: &[&GsVertexBuffer]) -> &GsCommandRecorder {

        let mut handles = vec![];
        let mut offsets  = vec![];

        for block in buffers.into_iter() {

            handles.push(block.render_info());
            // TODO: Add configuration for offset parameter.
            offsets.push(0);
        }

        unsafe {
            self.device.handle.cmd_bind_vertex_buffers(self.handle, first_binding, &handles, &offsets)
        };
        self
    }

    pub fn bind_index_buffer(&self, buffer: &GsIndexBuffer, offset: vkbytes) -> &GsCommandRecorder {

        let (indices_handle, indices_type) = buffer.render_info();

        unsafe {
            // TODO: Add configuration for IndexType.
            self.device.handle.cmd_bind_index_buffer(self.handle, indices_handle, offset, indices_type)
        };
        self
    }

    pub fn bind_descriptor_sets(&self, pipeline: &GsGraphicsPipeline, first_set: vkuint, infos: &[CmdDescriptorSetBindInfo]) -> &GsCommandRecorder {

        let mut handles = vec![];
        let mut dynamic_offsets = vec![];

        for set_info in infos.iter() {
            handles.push(set_info.set.entity.handle);

            if let Some(dyn_offsets) = set_info.dynamic_offsets {
                dynamic_offsets.extend(dyn_offsets.to_owned());
            }
        }

        unsafe {
            // TODO: Currently dynamic_offsets field is not configuration.
            self.device.handle.cmd_bind_descriptor_sets(
                self.handle, pipeline.bind_point(), pipeline.layout.handle, first_set, &handles, &dynamic_offsets)
        };
        self
    }

    pub(crate) fn copy_buffer(&self, src_buffer_handle: vk::Buffer, dst_buffer_handle: vk::Buffer, regions: &[vk::BufferCopy]) -> &GsCommandRecorder {
        unsafe {
            self.device.handle.cmd_copy_buffer(
                self.handle, src_buffer_handle, dst_buffer_handle, regions)
        };
        self
    }

    pub(crate) fn copy_buffer_to_image(&self, src_handle: vk::Buffer, dst_handle: vk::Image, dst_layout: vk::ImageLayout, regions: &[vk::BufferImageCopy]) -> &GsCommandRecorder {
        unsafe {
            self.device.handle.cmd_copy_buffer_to_image(
                self.handle, src_handle, dst_handle, dst_layout, regions)
        };
        self
    }

    pub(crate) fn copy_image_to_buffer(&self, src_handle: vk::Image, src_layout: vk::ImageLayout, dst_buffer: vk::Buffer, regions: &[vk::BufferImageCopy]) -> &GsCommandRecorder {
        unsafe {
            self.device.handle.cmd_copy_image_to_buffer(
                self.handle, src_handle, src_layout, dst_buffer, regions)
        };

        self
    }

    pub(crate) fn copy_image(&self,src_handle: vk::Image, src_layout: vk::ImageLayout, dst_handle: vk::Image, dst_layout: vk::ImageLayout, regions: &[vk::ImageCopy]) -> &GsCommandRecorder {
        unsafe {
            self.device.handle.cmd_copy_image(
                self.handle, src_handle, src_layout, dst_handle, dst_layout, regions)
        };

        self
    }

    pub fn draw(&self, vertex_count: vkuint, instance_count: vkuint, first_vertex: vkuint, first_instance: vkuint) -> &GsCommandRecorder {
        unsafe {
            self.device.handle.cmd_draw(self.handle, vertex_count, instance_count, first_vertex, first_instance)
        };
        self
    }

    pub fn draw_indexed(&self, index_count: vkuint, instance_count: vkuint, first_index: vkuint, vertex_offset: vksint, first_instance: vkuint) -> &GsCommandRecorder {
        unsafe {
            self.device.handle
                .cmd_draw_indexed(self.handle, index_count, instance_count, first_index, vertex_offset, first_instance)
        };
        self
    }
//    pub fn draw_indirect(&self, buffer: &GsBuffer, offset: vk::DeviceSize, draw_count: uint32_t, stride: uint32_t) -> &GsCommandRecorder {
//        unsafe {
//            self.device.handle.cmd_draw_indirect(self.buffer, buffer.handle, offset, draw_count, stride)
//        };
//        self
//    }
//    pub fn draw_indexed_indirect(&self, buffer: &GsBuffer, offset: vk::DeviceSize, draw_count: uint32_t, stride: uint32_t) -> &GsCommandRecorder {
//        unsafe {
//            self.device.handle.cmd_draw_indexed_indirect(self.buffer, buffer.handle, offset, draw_count, stride)
//        };
//        self
//    }


    #[inline]
    pub fn image_pipeline_barrrier(&self, src_stage: vk::PipelineStageFlags, dst_stage: vk::PipelineStageFlags, dependencies: vk::DependencyFlags, image_barries: Vec<GsImageBarrier>) -> &GsCommandRecorder {

        let barriers: Vec<vk::ImageMemoryBarrier> = image_barries.into_iter()
            .map(|b| b.into_barrier()).collect();

        self.pipeline_barrrier(src_stage, dst_stage, dependencies, &[], &[], &barriers)
    }

    fn pipeline_barrrier(&self, src_stage: vk::PipelineStageFlags, dst_stage: vk::PipelineStageFlags, dependencies: vk::DependencyFlags, memory_barries: &[vk::MemoryBarrier], buffer_barries: &[vk::BufferMemoryBarrier], image_barries: &[vk::ImageMemoryBarrier]) -> &GsCommandRecorder {

        unsafe {
            self.device.handle.cmd_pipeline_barrier(
                self.handle, src_stage, dst_stage, dependencies, memory_barries, buffer_barries, image_barries)
        };
        self
    }

    pub fn end_render_pass(&self) -> &GsCommandRecorder {
        unsafe {
            // Ending the render pass will add an implicit barrier transitioning the frame buffer color attachment vk::IMAGE_LAYOUT_PRESENT_SRC_KHR for presenting it to the windowing system.
            self.device.handle.cmd_end_render_pass(self.handle)
        };
        self
    }

    pub fn end_record(&mut self) -> Result<GsCommandBuffer, CommandError> {

        let _ = unsafe {
            self.device.handle.end_command_buffer(self.handle)
                .or(Err(CommandError::RecordEndError))?
        };

        let buffer = self.buffer.take().unwrap();

        Ok(buffer)
    }
}
