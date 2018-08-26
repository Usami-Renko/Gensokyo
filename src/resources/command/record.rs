
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use resources::command::buffer::HaCommandBuffer;
use resources::error::CommandError;

use pipeline::graphics::HaGraphicsPipeline;
use swapchain::HaSwapchain;
use utility::marker::VulkanFlags;

use std::ptr;

pub struct HaCommandRecorder<'buffer, 'vk> {

    pub(super) buffer:    &'buffer HaCommandBuffer,
    pub(super) device:    &'vk HaLogicalDevice,
    pub(super) swapchain: &'vk HaSwapchain,
    pub(super) pipeline:  &'vk HaGraphicsPipeline,
}

impl<'buffer, 'vk> HaCommandRecorder<'buffer, 'vk> {

    pub fn begin_record(&'buffer self, flags: &[CommandBufferUsageFlag])
        -> Result<&HaCommandRecorder<'buffer, 'vk>, CommandError> {

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


    pub fn begin_render_pass(&self, framebuffer_index: usize)
        -> &HaCommandRecorder<'buffer, 'vk> {

        let begin_info = vk::RenderPassBeginInfo {
            s_type: vk::StructureType::RenderPassBeginInfo,
            p_next: ptr::null(),
            render_pass: self.pipeline.pass.handle,
            framebuffer: self.swapchain.framebuffers[framebuffer_index].handle,
            render_area: vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: self.swapchain.extent,
            },
            clear_value_count: self.pipeline.pass.clear_values.len() as uint32_t,
            p_clear_values   : self.pipeline.pass.clear_values.as_ptr(),
        };

        unsafe {
            self.device.handle.cmd_begin_render_pass(self.buffer.handle,
                &begin_info,
                self.buffer.usage.usage());
        }

        self
    }

    pub fn bind_pipeline(&self) -> &HaCommandRecorder<'buffer, 'vk> {
        unsafe {
            self.device.handle.cmd_bind_pipeline(self.buffer.handle,
                self.pipeline.bind_point,
                self.pipeline.handle)
        };

        self
    }

    pub fn draw(&self, vertex_count: uint32_t, instance_count: uint32_t, first_vertex: uint32_t, first_instance: uint32_t)
        -> &HaCommandRecorder<'buffer, 'vk> {

        unsafe {
            self.device.handle.cmd_draw(self.buffer.handle,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance)
        };
        self
    }

    pub fn end_render_pass(&self) -> &HaCommandRecorder<'buffer, 'vk> {
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


#[allow(dead_code)]
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