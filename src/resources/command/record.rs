
use ash::vk;
use ash::vk::{ uint32_t, int32_t };
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use resources::buffer::HaBuffer;
use resources::command::buffer::HaCommandBuffer;
use resources::error::CommandError;

use pipeline::graphics::pipeline::HaGraphicsPipeline;
use resources::repository::{ CmdVertexBindingInfos, CmdIndexBindingInfo, CmdDescriptorBindingInfos };
use utility::marker::VulkanFlags;

use std::ptr;

pub struct HaCommandRecorder<'buffer, 're> {

    pub(super) buffer:    &'buffer HaCommandBuffer,
    pub(super) device:    &'re HaLogicalDevice,
}

impl<'buffer, 're> HaCommandRecorder<'buffer, 're> {

    pub fn begin_record(&'buffer self, flags: &[CommandBufferUsageFlag])
        -> Result<&HaCommandRecorder<'buffer, 're>, CommandError> {

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
        -> &HaCommandRecorder<'buffer, 're> {

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

    pub fn bind_pipeline(&self, pipeline: &HaGraphicsPipeline) -> &HaCommandRecorder<'buffer, 're> {
        unsafe {
            self.device.handle.cmd_bind_pipeline(self.buffer.handle, pipeline.bind_point, pipeline.handle)
        };

        self
    }

    pub fn bind_vertex_buffers(&self, first_binding: uint32_t, binding_infos: &CmdVertexBindingInfos) -> &HaCommandRecorder<'buffer, 're> {
        unsafe {
            self.device.handle.cmd_bind_vertex_buffers(self.buffer.handle, first_binding, &binding_infos.handles, &binding_infos.offsets)
        };
        self
    }
    pub fn bind_index_buffers(&self, index_info: &CmdIndexBindingInfo) -> &HaCommandRecorder<'buffer, 're> {
        unsafe {
            // TODO: Add configuration for IndexType.
            self.device.handle.cmd_bind_index_buffer(self.buffer.handle, index_info.handle, index_info.offset, vk::IndexType::Uint32)
        };
        self
    }
    pub fn bind_descriptor_sets(&self, pipeline: &HaGraphicsPipeline, first_set: uint32_t, binding_infos: &CmdDescriptorBindingInfos) -> &HaCommandRecorder<'buffer, 're> {
        unsafe {
            // TODO: Currently dynamic_offsets field is not configuration.
            self.device.handle.cmd_bind_descriptor_sets(self.buffer.handle, pipeline.bind_point, pipeline.layout.handle, first_set, &binding_infos.handles, &[])
        };
        self
    }

    pub(crate) fn copy_buffer(&self, src_buffer: &HaBuffer, dst_buffer: &HaBuffer, region: &[vk::BufferCopy]) -> &HaCommandRecorder<'buffer, 're> {
        unsafe {
            self.device.handle.cmd_copy_buffer(self.buffer.handle, src_buffer.handle, dst_buffer.handle, region)
        };
        self
    }

    pub fn draw(&self, vertex_count: uint32_t, instance_count: uint32_t, first_vertex: uint32_t, first_instance: uint32_t) -> &HaCommandRecorder<'buffer, 're> {
        unsafe {
            self.device.handle.cmd_draw(self.buffer.handle, vertex_count, instance_count, first_vertex, first_instance)
        };
        self
    }
    pub fn draw_indexed(&self, index_count: uint32_t, instance_count: uint32_t, first_index: uint32_t, vertex_offset: int32_t, first_instance: uint32_t) -> &HaCommandRecorder<'buffer, 're> {
        unsafe {
            self.device.handle
                .cmd_draw_indexed(self.buffer.handle, index_count, instance_count,first_index, vertex_offset, first_instance)
        };
        self
    }
//    pub fn draw_indirect(&self, buffer: &HaBuffer, offset: vk::DeviceSize, draw_count: uint32_t, stride: uint32_t) -> &HaCommandRecorder<'buffer, 're> {
//        unsafe {
//            self.device.handle.cmd_draw_indirect(self.buffer.handle, buffer.handle, offset, draw_count, stride)
//        };
//        self
//    }
//    pub fn draw_indexed_indirect(&self, buffer: &HaBuffer, offset: vk::DeviceSize, draw_count: uint32_t, stride: uint32_t) -> &HaCommandRecorder<'buffer, 're> {
//        unsafe {
//            self.device.handle.cmd_draw_indexed_indirect(self.buffer.handle, buffer.handle, offset, draw_count, stride)
//        };
//        self
//    }

    pub fn end_render_pass(&self) -> &HaCommandRecorder<'buffer, 're> {
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

//    pub fn pipeline_barrrier(&self) -> Result<&HaCommandRecorder<'buffer, 're>, CommandError> {
//
//        let image_barrier = vk::ImageMemoryBarrier {
//            s_type: vk::StructureType::ImageMemoryBarrier,
//            p_next: ptr::null(),
//            src_access_mask: vk::ACCESS_MEMORY_READ_BIT,
//            dst_access_mask: vk::ACCESS_MEMORY_READ_BIT,
//            old_layout: vk::ImageLayout::Undefined,
//            new_layout: vk::ImageLayout::PresentSrcKhr,
//            src_queue_family_index: self.device.present_queue_index.unwrap() as uint32_t,
//            dst_queue_family_index: self.device.graphics_queue_index.unwrap() as uint32_t,
//            image: Image
//            subresource_range: ImageSubresourceRange
//        };
//
//        unimplemented!()
//    }
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
