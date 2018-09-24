
use ash::vk;

use config::core::CoreConfig;
use core::DeviceV1;
use core::device::{ HaLogicalDevice, HaQueue };
use core::error::LogicalDeviceError;

pub trait HaQueueAbstract {

    fn new(device: &DeviceV1, queue: HaQueue, config: &CoreConfig) -> Result<Self, LogicalDeviceError> where Self: Sized;

    fn handle(&self) -> vk::Queue;
    fn cleanup(&self, device: &HaLogicalDevice);
}
