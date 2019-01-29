
use ash::vk;

use crate::error::GsResult;

use gsvk::core::GsDevice;
use gsvk::core::device::DeviceQueueIdentifier;

use gsvk::pipeline::target::GsVkPipelineType;

use gsvk::command::{ GsCommandBuffer, GsCommandPool };
use gsvk::command::{ GsVkCommandType, GsCmdRecorder };
use gsvk::command::CmdPipelineAbs;

use gsvk::utils::phantom::Transfer;

pub struct CommandKit {

    device: GsDevice,
}

impl CommandKit {

    pub(crate) fn init(device: &GsDevice) -> CommandKit {

        CommandKit {
            device: device.clone(),
        }
    }

    // FIXME: Currently not support any command pool flag.
    pub fn pool(&self, queue: DeviceQueueIdentifier) -> GsResult<GsCommandPool> {

        let pool = GsCommandPool::setup(&self.device, queue, vk::CommandPoolCreateFlags::empty())?;
        Ok(pool)
    }

    pub fn copy_recorder(&self, command: GsCommandBuffer) -> GsCmdRecorder<Transfer> {
        GsCmdRecorder::new_copy(&self.device, command)
    }

    pub fn pipeline_recorder<T>(&self, pipeline: &impl CmdPipelineAbs, command: GsCommandBuffer) -> GsCmdRecorder<T>
        where
            T: GsVkPipelineType + GsVkCommandType {

        GsCmdRecorder::new(&self.device, command, pipeline)
    }
}
