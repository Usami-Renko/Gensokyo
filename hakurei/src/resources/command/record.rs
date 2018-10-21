
use ash::vk;
use ash::vk::{ uint32_t, int32_t, c_float };
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::command::buffer::HaCommandBuffer;
use resources::command::CmdDescriptorBindingInfos;
use resources::command::{ CmdViewportInfo, CmdScissorInfo, CmdDepthBiasInfo, CmdDepthBoundInfo };
use resources::buffer::BufferBlockEntity;
use resources::error::CommandError;

use pipeline::graphics::HaGraphicsPipeline;
use pipeline::state::StencilFaceFlag;
use pipeline::pass::DependencyFlag;
use utility::marker::{ VulkanFlags, VulkanEnum };

use std::ptr;

pub struct HaCommandRecorder<'buffer> {

    pub(super) buffer: &'buffer HaCommandBuffer,
    pub(super) device: HaDevice,
}

impl<'buffer> HaCommandRecorder<'buffer> {

    pub fn begin_record(&'buffer self, flags: &[CommandBufferUsageFlag])
        -> Result<&HaCommandRecorder<'buffer>, CommandError> {

        let begin_info = vk::CommandBufferBeginInfo {
            s_type: vk::StructureType::CommandBufferBeginInfo,
            p_next: ptr::null(),
            flags: flags.flags(),
            // TODO: Add configuration for secondary command buffer
            // Inheritance_info is used if commandBuffer is a secondary command buffer.
            // If this is a primary command buffer, then this value is ignored.
            p_inheritance_info: ptr::null(),
        };

        unsafe {
            self.device.handle.begin_command_buffer(self.buffer.handle, &begin_info)
                .or(Err(CommandError::RecordBeginError))?
        };

        Ok(self)
    }


    pub fn begin_render_pass(&self, pipeline: &HaGraphicsPipeline, framebuffer_index: usize)
        -> &HaCommandRecorder<'buffer> {

        let begin_info = vk::RenderPassBeginInfo {
            s_type: vk::StructureType::RenderPassBeginInfo,
            p_next: ptr::null(),
            render_pass: pipeline.pass.handle,
            framebuffer: pipeline.pass.framebuffers[framebuffer_index].handle,
            render_area: vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: pipeline.pass.framebuffer_extent,
            },
            clear_value_count: pipeline.pass.clear_values.len() as uint32_t,
            p_clear_values   : pipeline.pass.clear_values.as_ptr(),
        };

        unsafe {
            self.device.handle.cmd_begin_render_pass(self.buffer.handle,
                &begin_info,
                self.buffer.usage.usage());
        }
        self
    }

    /// Set the viewport dynamically.
    /// Before using this function, the `ViewportStateType::Dynamic` or `ViewportStateType::DynamicViewportFixedScissor` must be set to ViewportState in pipeline creation(by calling `GraphicsPipelineConfig::setup_viewport()`).
    ///
    /// `first_viewport` is the index of the first viewport whose parameters are updated by the command.
    ///
    /// `viewports` specifies the new value to use as viewports.
    pub fn set_viewport(&self, first_viewport: uint32_t, viewports: &[CmdViewportInfo]) -> &HaCommandRecorder<'buffer> {

        let ports = viewports.iter()
            .map(|p| p.content).collect::<Vec<_>>();
        unsafe {
            self.device.handle.cmd_set_viewport(self.buffer.handle, first_viewport, &ports)
        };
        self
    }

    /// Set the scissor rectangles dynamically.
    /// Before using this function, the `ViewportStateType::Dynamic` or `ViewportStateType::FixedViewportDynamicScissor` must be set to ViewportState in pipeline creation(by calling `GraphicsPipelineConfig::setup_viewport()`).
    ///
    /// `first_scissor` is the index of the first scissor whose state is updated by the command.
    ///
    /// `scissors` specifies the new value to use as scissor rectangles.
    pub fn set_scissor(&self, first_scissor: uint32_t, scissors: &[CmdScissorInfo]) -> &HaCommandRecorder<'buffer> {

        let scissors = scissors.iter()
            .map(|s| s.content).collect::<Vec<_>>();
        unsafe {
            self.device.handle.cmd_set_scissor(self.buffer.handle, first_scissor, &scissors)
        };
        self
    }

    /// Set the line width dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `HaRasterizerState::set_line_width()` on RasterizerState during pipeline creation.
    ///
    /// `width` specifies the new value to use as the width of rasterized line segments.
    pub fn set_line_width(&self, width: c_float) -> &HaCommandRecorder<'buffer> {
        unsafe {
            self.device.handle.cmd_set_line_width(self.buffer.handle, width)
        };
        self
    }

    /// Set the depth bias dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `HaRasterizerState::set_depth_bias()` on RasterizerState during pipeline creation.
    ///
    /// `bias` specifies the new value to use as depth bias.
    pub fn set_depth_bias(&self, bias: CmdDepthBiasInfo) -> &HaCommandRecorder<'buffer> {
        unsafe {
            self.device.handle.cmd_set_depth_bias(self.buffer.handle, bias.constant_factor, bias.clamp, bias.slope_factor)
        };
        self
    }

    /// Set the blend constants dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `HaBlendState::set_blend_constants()` on BlendState during pipeline creation.
    ///
    /// `constants` specifies the new value to use as blend constants.
    pub fn set_blend_constants(&self, constants: [c_float; 4]) -> &HaCommandRecorder<'buffer> {
        unsafe {
            self.device.handle.cmd_set_blend_constants(self.buffer.handle, constants)
        };
        self
    }

    /// Set the depth bound dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `DepthTest::set_depth_bound()` on DepthStencilState during pipeline creation.
    ///
    /// `bound` specifies the new value to use as depth bound.
    pub fn set_depth_bound(&self, bound: CmdDepthBoundInfo) -> &HaCommandRecorder<'buffer> {
        unsafe {
            self.device.handle.cmd_set_depth_bounds(self.buffer.handle, bound.min_bound, bound.max_bound)
        };
        self
    }

    /// Set the stencil compare mask dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `StencilTest::set_compare_mask()` on DepthStencilState during pipeline creation.
    ///
    /// `face` specifies the set of stencil state for which to update the compare mask.
    ///
    /// `mask` specifies the new value to use as the stencil compare mask.
    pub fn set_stencil_compare_mask(&self, face: StencilFaceFlag, mask: uint32_t) -> &HaCommandRecorder<'buffer> {
        unsafe {
            self.device.handle.cmd_set_stencil_compare_mask(self.buffer.handle, face.value(), mask)
        };
        self
    }

    /// Set the stencil write mask dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `StencilTest::set_write_mask()` on DepthStencilState during pipeline creation.
    ///
    /// `face` specifies the set of stencil state for which to update the write mask.
    ///
    /// `mask` specifies the new value to use as the stencil write mask.
    pub fn set_stencil_write_mask(&self, face: StencilFaceFlag, mask: uint32_t) -> &HaCommandRecorder<'buffer> {
        unsafe {
            self.device.handle.cmd_set_stencil_write_mask(self.buffer.handle, face.value(), mask)
        };
        self
    }

    /// Set the stencil reference dynamically.
    /// Before using this function, the `DynamicableValue::Dynamic` must be set in function `StencilTest::set_reference()` on DepthStencilState during pipeline creation.
    ///
    /// `face` specifies the set of stencil state for which to update the reference value.
    ///
    /// `reference` specifies the set of stencil state for which to update the reference value.
    pub fn set_stencil_reference(&self, face: StencilFaceFlag, reference: uint32_t) -> &HaCommandRecorder<'buffer> {
        unsafe {
            self.device.handle.cmd_set_stencil_reference(self.buffer.handle, face.value(), reference)
        };
        self
    }

    pub fn bind_pipeline(&self, pipeline: &HaGraphicsPipeline) -> &HaCommandRecorder<'buffer> {
        unsafe {
            self.device.handle.cmd_bind_pipeline(self.buffer.handle, pipeline.bind_point, pipeline.handle)
        };
        self
    }

    pub fn bind_vertex_buffers(&self, first_binding: uint32_t, blocks: &[&impl BufferBlockEntity]) -> &HaCommandRecorder<'buffer> {

        let mut handles = vec![];
        let mut offsets  = vec![];
        for &block in blocks.iter() {
            let item = block.get_buffer_item();
            handles.push(item.handle);
            offsets.push(item.offset);
        }

        unsafe {
            self.device.handle.cmd_bind_vertex_buffers(self.buffer.handle, first_binding, &handles, &offsets)
        };
        self
    }

    pub fn bind_index_buffer(&self, index_info: &impl BufferBlockEntity) -> &HaCommandRecorder<'buffer> {

        let item = index_info.get_buffer_item();
        unsafe {
            // TODO: Add configuration for IndexType.
            self.device.handle.cmd_bind_index_buffer(self.buffer.handle, item.handle, item.offset, vk::IndexType::Uint32)
        };
        self
    }

    pub fn bind_descriptor_sets(&self, pipeline: &HaGraphicsPipeline, first_set: uint32_t, binding_infos: CmdDescriptorBindingInfos) -> &HaCommandRecorder<'buffer> {
        unsafe {
            // TODO: Currently dynamic_offsets field is not configuration.
            self.device.handle.cmd_bind_descriptor_sets(self.buffer.handle, pipeline.bind_point, pipeline.layout.handle, first_set, &binding_infos.handles, &[])
        };
        self
    }

    pub(crate) fn copy_buffer(&self, src_buffer_handle: vk::Buffer, dst_buffer_handle: vk::Buffer, regions: &[vk::BufferCopy]) -> &HaCommandRecorder<'buffer> {
        unsafe {
            self.device.handle.cmd_copy_buffer(self.buffer.handle, src_buffer_handle, dst_buffer_handle, regions)
        };
        self
    }

    pub(crate) fn copy_buffer_to_image(&self, src_handle: vk::Buffer, dst_handle: vk::Image, dst_layout: vk::ImageLayout, regions: &[vk::BufferImageCopy])
        -> &HaCommandRecorder<'buffer> {
        unsafe {
            self.device.handle.cmd_copy_buffer_to_image(self.buffer.handle, src_handle, dst_handle, dst_layout, regions)
        };
        self
    }

    pub fn draw(&self, vertex_count: uint32_t, instance_count: uint32_t, first_vertex: uint32_t, first_instance: uint32_t) -> &HaCommandRecorder<'buffer> {
        unsafe {
            self.device.handle.cmd_draw(self.buffer.handle, vertex_count, instance_count, first_vertex, first_instance)
        };
        self
    }

    pub fn draw_indexed(&self, index_count: uint32_t, instance_count: uint32_t, first_index: uint32_t, vertex_offset: int32_t, first_instance: uint32_t) -> &HaCommandRecorder<'buffer> {
        unsafe {
            self.device.handle
                .cmd_draw_indexed(self.buffer.handle, index_count, instance_count,first_index, vertex_offset, first_instance)
        };
        self
    }
//    pub fn draw_indirect(&self, buffer: &HaBuffer, offset: vk::DeviceSize, draw_count: uint32_t, stride: uint32_t) -> &HaCommandRecorder<'buffer> {
//        unsafe {
//            self.device.handle.cmd_draw_indirect(self.buffer.handle, buffer.handle, offset, draw_count, stride)
//        };
//        self
//    }
//    pub fn draw_indexed_indirect(&self, buffer: &HaBuffer, offset: vk::DeviceSize, draw_count: uint32_t, stride: uint32_t) -> &HaCommandRecorder<'buffer> {
//        unsafe {
//            self.device.handle.cmd_draw_indexed_indirect(self.buffer.handle, buffer.handle, offset, draw_count, stride)
//        };
//        self
//    }

    pub(crate) fn pipeline_barrrier(&self, src_stage: vk::PipelineStageFlags, dst_stage: vk::PipelineStageFlags, dependencies: &[DependencyFlag], memory_barries: &[vk::MemoryBarrier], buffer_memory_barries: &[vk::BufferMemoryBarrier], image_memory_barries: &[vk::ImageMemoryBarrier], ) -> &HaCommandRecorder<'buffer> {
        unsafe {
            self.device.handle.cmd_pipeline_barrier(self.buffer.handle, src_stage, dst_stage, dependencies.flags(), memory_barries, buffer_memory_barries, image_memory_barries)
        };
        self
    }

    pub fn end_render_pass(&self) -> &HaCommandRecorder<'buffer> {
        unsafe {
            // Ending the render pass will add an implicit barrier transitioning the frame buffer color attachment vk::IMAGE_LAYOUT_PRESENT_SRC_KHR for presenting it to the windowing system.
            self.device.handle.cmd_end_render_pass(self.buffer.handle)
        };
        self
    }

    pub fn end_record(&self) -> Result<(), CommandError> {
        let _ = unsafe {
            self.device.handle.end_command_buffer(self.buffer.handle)
                .or(Err(CommandError::RecordEndError))?
        };
        Ok(())
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CommandBufferUsageFlag {
    /// OneTimeSubmitBit specifies that each recording of the command buffer will only be submitted once,
    /// and the command buffer will be reset and recorded again between each submission.
    OneTimeSubmitBit,
    /// RenderPassContinueBit specifies that a secondary command buffer is considered to be entirely inside a render pass.
    ///
    /// If this is a primary command buffer, then this bit is ignored.
    RenderPassContinueBit,
    /// SimultaneousUseBit specifies that a command buffer can be resubmitted to a queue while it is in the pending state,
    /// and recorded into multiple primary command buffers.
    SimultaneousUseBit,
}

impl VulkanFlags for [CommandBufferUsageFlag] {
    type FlagType = vk::CommandBufferUsageFlags;

    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::CommandBufferUsageFlags::empty(), |acc, flag| {
            match *flag {
                | CommandBufferUsageFlag::OneTimeSubmitBit      => acc | vk::COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT,
                | CommandBufferUsageFlag::RenderPassContinueBit => acc | vk::COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT,
                | CommandBufferUsageFlag::SimultaneousUseBit    => acc | vk::COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT,
            }
        })
    }
}
