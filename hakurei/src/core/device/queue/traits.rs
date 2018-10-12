
use ash::vk;

use config::core::DeviceConfig;
use core::DeviceV1;
use core::device::{ HaLogicalDevice, HaQueue };
use core::error::LogicalDeviceError;

use std::rc::Rc;

pub trait HaQueueAbstract {

    fn new(device: &DeviceV1, queue: &Rc<HaQueue>, config: &DeviceConfig) -> Result<Self, LogicalDeviceError> where Self: Sized;

    fn handle(&self) -> vk::Queue;
    fn cleanup(&self, device: &HaLogicalDevice);
}
