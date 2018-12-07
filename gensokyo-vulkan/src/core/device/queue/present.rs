
use crate::core::device::device::{ GsLogicalDevice, DeviceConfig };
use crate::core::device::queue::{ GsQueue, GsQueueAbstract };
use crate::core::error::LogicalDeviceError;

use std::rc::Rc;

pub struct GsPresentQueue {

    queue: Rc<GsQueue>,
}

impl GsQueueAbstract for GsPresentQueue {

    fn new(_device: &ash::Device, queue: &Rc<GsQueue>, _config: &DeviceConfig) -> Result<Self, LogicalDeviceError> {

        let present_queue = GsPresentQueue {
            queue: queue.clone(),
        };

        Ok(present_queue)
    }

    fn queue(&self) -> &Rc<GsQueue> {
        &self.queue
    }

    fn cleanup(&self, _device: &GsLogicalDevice) {
        // nothing to clean
    }
}
