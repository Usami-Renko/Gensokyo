
use ash;

use core::device::device::{ GsLogicalDevice, DeviceConfig };
use core::device::queue::GsQueue;
use core::error::LogicalDeviceError;

use std::rc::Rc;

pub trait GsQueueAbstract {

    fn new(device: &ash::Device, queue: &Rc<GsQueue>, config: &DeviceConfig) -> Result<Self, LogicalDeviceError> where Self: Sized;

    fn queue(&self) -> &Rc<GsQueue>;
    fn cleanup(&self, device: &GsLogicalDevice);
}
