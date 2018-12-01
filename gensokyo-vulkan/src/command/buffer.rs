
use ash::vk;

use core::device::GsDevice;

use command::record::GsCommandRecorder;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CmdBufferUsage {

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

impl CmdBufferUsage {

    pub(super) fn level(&self) -> vk::CommandBufferLevel {
        match self {
            | CmdBufferUsage::UnitaryCommand
            | CmdBufferUsage::PrimaryCommand   => vk::CommandBufferLevel::PRIMARY,
            | CmdBufferUsage::SecondaryCommand => vk::CommandBufferLevel::SECONDARY,
        }
    }

    pub(super) fn contents(&self) -> vk::SubpassContents {
        match self {
            // Inline specifies that the render pass commands will be embedded in the primary command buffer itself and no secondary command buffers will be executed.
            | CmdBufferUsage::UnitaryCommand   => vk::SubpassContents::INLINE,
            // SecondaryCommandBuffer specifies that the render pass commands will be executed from secondary command buffers,
            // and vkCmdExecuteCommands is the only valid command on the command buffer until vkCmdNextSubpass or vkCmdEndRenderPass.
            | CmdBufferUsage::PrimaryCommand
            | CmdBufferUsage::SecondaryCommand => vk::SubpassContents::SECONDARY_COMMAND_BUFFERS,
        }
    }
}

pub struct GsCommandBuffer {

    pub(crate) handle: vk::CommandBuffer,
    pub(crate) usage : CmdBufferUsage,
}

impl GsCommandBuffer {

    pub(crate) fn new(handle: vk::CommandBuffer, usage: CmdBufferUsage) -> GsCommandBuffer {
        GsCommandBuffer { handle, usage }
    }

    pub(crate) fn setup_record(self, device: &GsDevice) -> GsCommandRecorder {

        GsCommandRecorder::new(device, self)
    }
}
