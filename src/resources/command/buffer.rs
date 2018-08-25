
use ash::vk;

use core::device::HaLogicalDevice;

use swapchain::HaSwapchain;
use pipeline::graphics::HaGraphicsPipeline;
use resources::command::record::HaCommandRecorder;

use resources::error::CommandError;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CommandBufferUsage {
    /// UnitaryCommand specifies that only primary command buffer will be used in the render pass,
    /// and secondary command buffers will never use.
    UnitaryCommand,
    /// PrimaryCommand specifies that this buffer will be used as primary command buffer in render pass,
    /// and there will be secondary command buffer used in render pass.
    PrimaryCommand,
    /// SecondaryCommand specifies that this buffer will be used as secondary command buffer in the render pass,
    /// and there will be primary command buffer used in render pass.
    SecondaryCommand,
}
impl CommandBufferUsage {
    pub(super) fn level(&self) -> vk::CommandBufferLevel {
        match *self {
            | CommandBufferUsage::UnitaryCommand
            | CommandBufferUsage::PrimaryCommand   => vk::CommandBufferLevel::Primary,
            | CommandBufferUsage::SecondaryCommand => vk::CommandBufferLevel::Secondary,
        }
    }
    pub(super) fn usage(&self) -> vk::SubpassContents {
        match *self {
            // Inline specifies that the render pass commands will be embedded in the primary command buffer itself and no secondary command buffers will be executed.
            | CommandBufferUsage::UnitaryCommand   => vk::SubpassContents::Inline,
            // SecondaryCommandBuffer specifies that the render pass commands will be executed from secondary command buffers,
            // and vkCmdExecuteCommands is the only valid command on the command buffer until vkCmdNextSubpass or vkCmdEndRenderPass.
            | CommandBufferUsage::PrimaryCommand
            | CommandBufferUsage::SecondaryCommand => vk::SubpassContents::SecondaryCommandBuffers,
        }
    }
}

pub struct HaCommandBuffer {

    pub(super) handle: vk::CommandBuffer,
    pub(super) usage: CommandBufferUsage,
}

impl<'buffer, 'vk: 'buffer> HaCommandBuffer {

    pub fn setup_record(&'buffer self, device: &'vk HaLogicalDevice, swapchain: &'vk HaSwapchain, pipeline: &'vk HaGraphicsPipeline)
        -> Result<HaCommandRecorder<'buffer, 'vk>, CommandError> {

        let recorder = HaCommandRecorder {
            buffer: self,
            device,
            swapchain,
            pipeline,
        };

        Ok(recorder)
    }

}
