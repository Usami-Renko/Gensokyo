
use ash::vk;

use gsvk::core::device::GsDevice;
use gsvk::core::device::DeviceQueueIdentifier;

use gsvk::command::GsCommandPool;
use gsvk::command::{ GsCommandBuffer, GsCommandRecorder };
use gsvk::command::CommandError;

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

    pub fn recorder(&self, command: GsCommandBuffer) -> GsCommandRecorder {

        GsCommandRecorder::new(&self.device, command)
    }
}
