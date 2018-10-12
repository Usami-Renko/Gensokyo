
use ash::vk;

use config::core::DeviceConfig;
use core::DeviceV1;
use core::device::{ HaLogicalDevice, HaQueue };
use core::device::queue::HaQueueAbstract;
use core::error::LogicalDeviceError;

use std::rc::Rc;

pub struct HaPresentQueue {

    pub queue: Rc<HaQueue>,
}

impl HaQueueAbstract for HaPresentQueue {

    fn new(_device: &DeviceV1, queue: &Rc<HaQueue>, _config: &DeviceConfig) -> Result<Self, LogicalDeviceError> {

        let present_queue = HaPresentQueue {
            queue: queue.clone(),
        };
        Ok(present_queue)
    }

    fn handle(&self) -> vk::Queue {
        self.queue.handle
    }

    fn cleanup(&self, _device: &HaLogicalDevice) {
        // nothing to clean
    }
}
