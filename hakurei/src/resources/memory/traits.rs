
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use resources::buffer::{ HaBuffer, BufferSubItem };
use resources::image::HaImage;
use resources::error::MemoryError;

use utility::memory::MemoryWritePtr;

use std::ptr;

// TODO: Split HaMemoryAbstract into small trait.

pub(crate) trait HaMemoryAbstract: MemoryDataTransfer {

    fn handle(&self) -> vk::DeviceMemory;
    fn flag(&self) -> vk::MemoryPropertyFlags;
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

    // memory mapping
    fn map_range(&self, device: &HaLogicalDevice, offset: vk::DeviceSize, size: vk::DeviceSize) -> Result<*mut vk::c_void, MemoryError> {

        let data_ptr = unsafe {
            device.handle.map_memory(
                // zero-based byte offset from the beginning of the memory object.
                self.handle(),
                offset,
                // the size of the memory range to map, or VK_WHOLE_SIZE to map from offset to the end of the allocation.
                size,
                // flags is reserved for future use in API version 1.1.82.
                vk::MemoryMapFlags::empty(),
            ).or(Err(MemoryError::MapMemoryError))?
        };

        Ok(data_ptr)
    }
    fn map_whole(&self, device: &HaLogicalDevice) -> Result<*mut vk::c_void, MemoryError> {

        let data_ptr = unsafe {
            device.handle.map_memory(self.handle(), 0, vk::VK_WHOLE_SIZE, vk::MemoryMapFlags::empty())
                .or(Err(MemoryError::MapMemoryError))?
        };

        Ok(data_ptr)
    }

    fn flush_ranges(&self, device: &HaLogicalDevice, ranges: &Vec<(vk::DeviceSize, vk::DeviceSize)>) -> Result<(), MemoryError> {

        let flush_ranges = ranges.iter()
            .map(|&(size, offset)| {
                vk::MappedMemoryRange {
                    s_type: vk::StructureType::MappedMemoryRange,
                    p_next: ptr::null(),
                    memory: self.handle(),
                    offset,
                    size,
                }
        }).collect::<Vec<_>>();

        unsafe {
            device.handle.flush_mapped_memory_ranges(&flush_ranges)
                .or(Err(MemoryError::FlushMemoryError))
        }
    }

    fn unmap(&self, device: &HaLogicalDevice) {

        unsafe {
            device.handle.unmap_memory(self.handle())
        }
    }

    fn is_coherent_memroy(&self) -> bool {
        self.flag().subset(vk::MEMORY_PROPERTY_HOST_COHERENT_BIT)
    }
}

pub(crate) trait MemoryDataTransfer {

    // data transfer
    // only use the following theree function for first time data transfer.
    fn prepare_data_transfer(&mut self, device: &HaLogicalDevice) -> Result<(), MemoryError>;

    fn map_memory_ptr(&mut self, device: &HaLogicalDevice, item: &BufferSubItem, offset: vk::DeviceSize) -> Result<MemoryWritePtr, MemoryError>;
    fn unmap_memory_ptr(&mut self, item: &BufferSubItem, offset: vk::DeviceSize);

    fn transfer_data(&mut self, device: &HaLogicalDevice) -> Result<(), MemoryError>;
}

// FIXME: Change impl for MemoryDataTransfer
impl HaMemoryAbstract {

    pub fn add_transfer_data<D: Copy>(&mut self, device: &HaLogicalDevice, item: &BufferSubItem, data: &Vec<D>, offset: vk::DeviceSize) -> Result<(), MemoryError> {

        let data_ptr = self.map_memory_ptr(device, item, offset)?;
        data_ptr.write_data(data);

        self.unmap_memory_ptr(item, offset);

        Ok(())
    }
}
