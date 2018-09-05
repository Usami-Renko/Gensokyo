
use ash::vk;

use core::device::HaLogicalDevice;
use core::physical::HaPhysicalDevice;

use resources::buffer::HaBuffer;
use resources::image::HaImage;
use resources::error::MemoryError;

pub(crate) trait HaMemoryAbstract where Self: Sized {
    fn allocate(physical: &HaPhysicalDevice, device: &HaLogicalDevice, size: vk::DeviceSize, type_index: usize, flag: vk::MemoryPropertyFlags) -> Result<Self, MemoryError>;

    fn bind_to_buffer(&self, device: &HaLogicalDevice, buffer: &HaBuffer, offset: vk::DeviceSize) -> Result<(), MemoryError>;
    fn bind_to_image(&self, device: &HaLogicalDevice, image: &HaImage, memory_offset: vk::DeviceSize) -> Result<(), MemoryError>;
    fn map(&self, device: &HaLogicalDevice, offset: vk::DeviceSize, size: vk::DeviceSize) -> Result<*mut vk::c_void, MemoryError>;
    fn unmap(&self, device: &HaLogicalDevice, offset: vk::DeviceSize, size: vk::DeviceSize) -> Result<(), MemoryError>;
}
