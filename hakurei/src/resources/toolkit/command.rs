
use core::device::HaDevice;
use core::device::DeviceQueueIdentifier;

use resources::command::HaCommandPool;
use resources::error::CommandError;

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
}