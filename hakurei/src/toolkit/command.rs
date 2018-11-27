
use ash::vk;

use gsvk::core::device::HaDevice;
use gsvk::core::device::DeviceQueueIdentifier;

use gsvk::command::HaCommandPool;
use gsvk::command::{ HaCommandBuffer, HaCommandRecorder };
use gsvk::command::CommandError;

pub struct CommandKit {

    device: HaDevice,
}

impl CommandKit {

    pub(crate) fn init(device: &HaDevice) -> CommandKit {

        CommandKit {
            device: device.clone(),
        }
    }

    // FIXME: Currently not support any commmand pool flag.
    pub fn pool(&self, queue: DeviceQueueIdentifier, flags: vk::CommandPoolCreateFlags) -> Result<HaCommandPool, CommandError> {

        HaCommandPool::setup(&self.device, queue, flags)
    }

    pub fn recorder(&self, command: HaCommandBuffer) -> HaCommandRecorder {

        HaCommandRecorder::new(&self.device, command)
    }
}
