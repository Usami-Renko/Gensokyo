
use vk::core::device::HaDevice;
use vk::core::device::DeviceQueueIdentifier;

use vk::resources::command::HaCommandPool;
use vk::resources::command::{ HaCommandBuffer, HaCommandRecorder };
use vk::resources::error::CommandError;

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
    pub fn pool(&self, queue: DeviceQueueIdentifier) -> Result<HaCommandPool, CommandError> {

        HaCommandPool::setup(&self.device, queue, &[])
    }

    pub fn recorder(&self, command: HaCommandBuffer) -> HaCommandRecorder {

        HaCommandRecorder::new(&self.device, command)
    }
}
