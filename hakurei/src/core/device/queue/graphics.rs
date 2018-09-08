
use ash::vk;

use config::core::CoreConfig;
use core::DeviceV1;
use core::device::{ HaLogicalDevice, HaQueue };
use core::device::queue::HaQueueAbstract;
use core::error::LogicalDeviceError;

pub struct HaGraphicsQueue {

    pub queue: HaQueue,
}

impl HaQueueAbstract for HaGraphicsQueue {

    fn new(_device: &DeviceV1, queue: HaQueue, _config: &CoreConfig) -> Result<Self, LogicalDeviceError> {

        let graphics_queue = HaGraphicsQueue {
            queue,
        };
        Ok(graphics_queue)
    }

    fn handle(&self) -> vk::Queue {
        self.queue.handle
    }

    fn clean(&self, _device: &HaLogicalDevice) {
        // nothing to clean
    }
}
