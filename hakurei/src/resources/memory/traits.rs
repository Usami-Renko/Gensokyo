
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use resources::buffer::{ HaBuffer, BufferSubItem };
use resources::image::HaImage;
use resources::error::MemoryError;

use utility::memory::MemoryWritePtr;

pub(crate) enum HaMemoryType {
    HostMemory,
    DeviceMemory,
}

// TODO: Split HaMemoryAbstract into small trait.

pub(crate) trait HaMemoryAbstract: MemoryDataTransferable {

    fn handle(&self) -> vk::DeviceMemory;
    fn flag(&self) -> vk::MemoryPropertyFlags;
    fn memory_type(&self) -> HaMemoryType;
    fn default_flag() -> vk::MemoryPropertyFlags where Self: Sized;

    fn allocate(device: &HaLogicalDevice, size: vk::DeviceSize, mem_type_index: usize, mem_type: Option<vk::MemoryType>) -> Result<Self, MemoryError> where Self: Sized;

    // bindings
    fn bind_to_buffer(&self, device: &HaLogicalDevice, buffer: &HaBuffer, memory_offset: vk::DeviceSize) -> Result<(), MemoryError> {
        unsafe {
            device.handle.bind_buffer_memory(buffer.handle, self.handle(), memory_offset)
                .or(Err(MemoryError::BindMemoryError))
        }
    }
    fn bind_to_image(&self, device: &HaLogicalDevice, image: &HaImage, memory_offset: vk::DeviceSize) -> Result<(), MemoryError> {
        unsafe {
            device.handle.bind_image_memory(image.handle, self.handle(), memory_offset)
                .or(Err(MemoryError::BindMemoryError))
        }
    }

    // cleaning
    fn cleanup(&self, device: &HaLogicalDevice) {
        unsafe {
            device.handle.free_memory(self.handle(), None);
        }
    }

    fn enable_map(&mut self, device: &HaLogicalDevice, is_enable: bool) -> Result<(), MemoryError>;

    fn is_coherent_memroy(&self) -> bool {
        self.flag().subset(vk::MEMORY_PROPERTY_HOST_COHERENT_BIT)
    }
}

pub(crate) trait MemoryDataTransferable {

    // data transfer
    // only use the following theree function for first time data transfer.
    fn prepare_data_transfer(&mut self, device: &HaLogicalDevice) -> Result<(), MemoryError>;

    fn map_memory_ptr(&mut self, item: &BufferSubItem, offset: vk::DeviceSize) -> Result<(MemoryWritePtr, MemoryRange), MemoryError>;

    fn terminate_transfer(&mut self, device: &HaLogicalDevice, ranges_to_flush: &Vec<MemoryRange>) -> Result<(), MemoryError>;
}

pub struct MemoryRange {

    pub offset: vk::DeviceSize,
    pub size  : vk::DeviceSize,
}
