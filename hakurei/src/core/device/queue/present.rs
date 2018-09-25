
use ash::vk;

use config::core::CoreConfig;
use core::DeviceV1;
use core::device::{ HaLogicalDevice, HaQueue };
use core::device::queue::HaQueueAbstract;
use core::error::LogicalDeviceError;

pub struct HaPresentQueue {

    pub queue: HaQueue,
}

impl HaQueueAbstract for HaPresentQueue {

    fn new(_device: &DeviceV1, queue: HaQueue, _config: &CoreConfig) -> Result<Self, LogicalDeviceError> {

        let present_queue = HaPresentQueue {
            queue,
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
