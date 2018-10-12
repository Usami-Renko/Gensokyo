
use ash::vk;

use config::core::DeviceConfig;
use core::DeviceV1;
use core::device::{ HaLogicalDevice, HaQueue };
use core::device::queue::HaQueueAbstract;
use core::error::LogicalDeviceError;

use std::rc::Rc;

pub struct HaGraphicsQueue {

    pub queue: Rc<HaQueue>,
}

impl HaQueueAbstract for HaGraphicsQueue {

    fn new(_device: &DeviceV1, queue: &Rc<HaQueue>, _config: &DeviceConfig) -> Result<Self, LogicalDeviceError> {

        let graphics_queue = HaGraphicsQueue {
            queue: queue.clone(),
        };
        Ok(graphics_queue)
    }

    fn handle(&self) -> vk::Queue {
        self.queue.handle
    }

    fn cleanup(&self, _device: &HaLogicalDevice) {
        // nothing to clean
    }
}
