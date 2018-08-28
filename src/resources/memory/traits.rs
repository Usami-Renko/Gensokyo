
use ash::vk;

use core::device::HaLogicalDevice;
use core::physical::HaPhysicalDevice;

use resources::error::MemoryError;

pub trait HaMemoryAbstract where Self: Sized {
    fn allocate(physical: &HaPhysicalDevice, device: &HaLogicalDevice, size: vk::DeviceSize, type_index: usize)
        -> Result<Self, MemoryError>;

    fn bind(&self, device: &HaLogicalDevice, buffer_handle: vk::Buffer, offset: vk::DeviceSize) -> Result<(), MemoryError>;

    fn map(&self, device: &HaLogicalDevice, offset: vk::DeviceSize, size: vk::DeviceSize) -> Result<*mut vk::c_void, MemoryError>;
    fn unmap(&self, device: &HaLogicalDevice);
}
