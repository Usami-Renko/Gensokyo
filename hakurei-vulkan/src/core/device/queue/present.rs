
use core::DeviceV1;
use core::device::device::{ HaLogicalDevice, DeviceConfig };
use core::device::queue::{ HaQueue, HaQueueAbstract };
use core::error::LogicalDeviceError;

use std::rc::Rc;

pub struct HaPresentQueue {

    queue: Rc<HaQueue>,
}

impl HaQueueAbstract for HaPresentQueue {

    fn new(_device: &DeviceV1, queue: &Rc<HaQueue>, _config: &DeviceConfig) -> Result<Self, LogicalDeviceError> {

        let present_queue = HaPresentQueue {
            queue: queue.clone(),
        };

        Ok(present_queue)
    }

    fn queue(&self) -> &Rc<HaQueue> {
        &self.queue
    }

    fn cleanup(&self, _device: &HaLogicalDevice) {
        // nothing to clean
    }
}
