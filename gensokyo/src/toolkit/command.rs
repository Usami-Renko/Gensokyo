
use ash::vk;

use gsvk::core::device::GsDevice;
use gsvk::core::device::DeviceQueueIdentifier;

use gsvk::pipeline::target::{ GsPipeline, GsVkPipelineType };

use gsvk::command::{ GsCommandBuffer, GsCommandPool };
use gsvk::command::{ GsVkCommandType, GsCmdRecorder };
use gsvk::command::CommandError;

use gsvk::utils::phantom::Copy;

pub struct CommandKit {

    device: GsDevice,
}

impl CommandKit {

    pub(crate) fn init(device: &GsDevice) -> CommandKit {

        CommandKit {
            device: device.clone(),
        }
    }

    // FIXME: Currently not support any commmand pool flag.
    pub fn pool(&self, queue: DeviceQueueIdentifier) -> Result<GsCommandPool, CommandError> {

        GsCommandPool::setup(&self.device, queue, vk::CommandPoolCreateFlags::empty())
    }

    pub fn copy_recorder(&self, command: GsCommandBuffer) -> GsCmdRecorder<r#Copy> {
        GsCmdRecorder::new_copy(&self.device, command)
    }

    pub fn pipeline_recorder<T>(&self, pipeline: &GsPipeline<T>, command: GsCommandBuffer) -> GsCmdRecorder<T>
        where T: GsVkPipelineType + GsVkCommandType {

        GsCmdRecorder::new(&self.device, command, pipeline)
    }
}
