
use ash;

use core::device::device::{ HaLogicalDevice, DeviceConfig };
use core::device::queue::HaQueue;
use core::error::LogicalDeviceError;

use std::rc::Rc;

pub trait HaQueueAbstract {

    fn new(device: &ash::Device, queue: &Rc<HaQueue>, config: &DeviceConfig) -> Result<Self, LogicalDeviceError> where Self: Sized;

    fn queue(&self) -> &Rc<HaQueue>;
    fn cleanup(&self, device: &HaLogicalDevice);
}
