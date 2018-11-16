
use core::DeviceV1;
use core::device::device::{ HaLogicalDevice, DeviceConfig };
use core::device::queue::{ HaQueue, HaQueueAbstract };
use core::error::LogicalDeviceError;

use std::rc::Rc;

pub struct HaGraphicsQueue {

    queue: Rc<HaQueue>,
}

impl HaQueueAbstract for HaGraphicsQueue {

    fn new(_device: &DeviceV1, queue: &Rc<HaQueue>, _config: &DeviceConfig) -> Result<Self, LogicalDeviceError> {

        let graphics_queue = HaGraphicsQueue {
            queue: queue.clone(),
        };
        Ok(graphics_queue)
    }

    fn queue(&self) -> &Rc<HaQueue> {
        &self.queue
    }

    fn cleanup(&self, _device: &HaLogicalDevice) {
        // nothing to clean
    }
}
