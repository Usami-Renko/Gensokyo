
use ash::vk;
use ash::vk::{ uint32_t, int32_t };
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::command::buffer::HaCommandBuffer;
use resources::error::CommandError;

use pipeline::graphics::HaGraphicsPipeline;
use pipeline::stages::PipelineStageFlag;
use pipeline::pass::DependencyFlag;
use resources::repository::{ CmdVertexBindingInfos, CmdIndexBindingInfo, CmdDescriptorBindingInfos };
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

    pub fn bind_pipeline(&self, pipeline: &HaGraphicsPipeline) -> &HaCommandRecorder<'buffer> {
        unsafe {
            self.device.handle.cmd_bind_pipeline(self.buffer.handle, pipeline.bind_point, pipeline.handle)
        };
        self
    }

    pub fn bind_vertex_buffers(&self, first_binding: uint32_t, binding_infos: &CmdVertexBindingInfos) -> &HaCommandRecorder<'buffer> {
        unsafe {
            self.device.handle.cmd_bind_vertex_buffers(self.buffer.handle, first_binding, &binding_infos.handles, &binding_infos.offsets)
        };
        self
    }
    pub fn bind_index_buffers(&self, index_info: &CmdIndexBindingInfo) -> &HaCommandRecorder<'buffer> {
        unsafe {
            // TODO: Add configuration for IndexType.
            self.device.handle.cmd_bind_index_buffer(self.buffer.handle, index_info.handle, index_info.offset, vk::IndexType::Uint32)
        };
        self
    }
    pub fn bind_descriptor_sets(&self, pipeline: &HaGraphicsPipeline, first_set: uint32_t, binding_infos: &CmdDescriptorBindingInfos) -> &HaCommandRecorder<'buffer> {
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

    pub(crate) fn pipeline_barrrier(&self, src_stage: PipelineStageFlag, dst_stage: PipelineStageFlag, dependencies: &[DependencyFlag], memory_barries: &[vk::MemoryBarrier], buffer_memory_barries: &[vk::BufferMemoryBarrier], image_memory_barries: &[vk::ImageMemoryBarrier], ) -> &HaCommandRecorder<'buffer> {
        unsafe {
            self.device.handle.cmd_pipeline_barrier(self.buffer.handle, src_stage.value(), dst_stage.value(), dependencies.flags(), memory_barries, buffer_memory_barries, image_memory_barries)
        };
        self
    }

    pub fn end_render_pass(&self) -> &HaCommandRecorder<'buffer> {
        unsafe { self.device.handle.cmd_end_render_pass(self.buffer.handle) };
        self
    }

    pub fn finish(&self) -> Result<(), CommandError> {
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
