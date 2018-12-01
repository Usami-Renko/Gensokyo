
use ash;

use core::device::device::{ GsLogicalDevice, DeviceConfig };
use core::device::queue::{ GsQueue, GsQueueAbstract };
use core::error::LogicalDeviceError;

use std::rc::Rc;

pub struct GsGraphicsQueue {

    queue: Rc<GsQueue>,
}

impl GsQueueAbstract for GsGraphicsQueue {

    fn new(_device: &ash::Device, queue: &Rc<GsQueue>, _config: &DeviceConfig) -> Result<Self, LogicalDeviceError> {

        let graphics_queue = GsGraphicsQueue {
            queue: queue.clone(),
        };
        Ok(graphics_queue)
    }

    fn queue(&self) -> &Rc<GsQueue> {
        &self.queue
    }

    fn cleanup(&self, _device: &GsLogicalDevice) {
        // nothing to clean
    }
}
