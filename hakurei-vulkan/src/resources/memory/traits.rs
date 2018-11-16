
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::buffer::HaBuffer;
use resources::memory::target::HaMemory;
use resources::memory::selector::MemorySelector;
use resources::memory::structs::{ HaMemoryType, MemoryRange };
use resources::image::HaImage;
use resources::error::MemoryError;

use utils::types::{ vkint, vkMemorySize, MemPtr };

use std::ptr;

pub trait HaMemoryAbstract {

    fn target(&self) -> &HaMemory;

    // TODO: Make this to const variable in trait.
    fn memory_type(&self) -> HaMemoryType;

    fn allocate(device: &HaDevice, size: vkMemorySize, selector: &MemorySelector) -> Result<Self, MemoryError> where Self: Sized;

    // bindings
    fn bind_to_buffer(&self, device: &HaDevice, buffer: &HaBuffer, memory_offset: vkMemorySize) -> Result<(), MemoryError> {
        unsafe {
            device.handle.bind_buffer_memory(buffer.handle, self.target().handle, memory_offset)
                .or(Err(MemoryError::BindMemoryError))
        }
    }

    fn bind_to_image(&self, device: &HaDevice, image: &HaImage, memory_offset: vkMemorySize) -> Result<(), MemoryError> {
        unsafe {
            device.handle.bind_image_memory(image.handle, self.target().handle, memory_offset)
                .or(Err(MemoryError::BindMemoryError))
        }
    }

    fn cleanup(&self, device: &HaDevice) {
        unsafe {
            device.handle.free_memory(self.target().handle, None);
        }
    }
}

/// A trait indicate a Memory is able to map.
pub trait MemoryMapable: HaMemoryAbstract {

    /// Map specific range of the memory.
    ///
    /// If range is None, the function will map the whole memory.
    fn map_range(&self, device: &HaDevice, range: Option<MemoryRange>) -> Result<MemPtr, MemoryError> {

        let memory = self.target();

        let data_ptr = unsafe {
            if let Some(range) = range {
                device.handle.map_memory(
                    memory.handle,
                    // zero-based byte offset from the beginning of the memory object.
                    range.offset,
                    // the size of the memory range to map, or VK_WHOLE_SIZE to map from offset to the end of the allocation.
                    range.size,
                    // flags is reserved for future use in API version 1.1.82.
                    vk::MemoryMapFlags::empty(),
                ).or(Err(MemoryError::MapMemoryError))?
            } else {
                device.handle.map_memory(memory.handle, 0, vk::VK_WHOLE_SIZE, vk::MemoryMapFlags::empty())
                    .or(Err(MemoryError::MapMemoryError))?
            }
        };

        Ok(data_ptr)
    }

    fn flush_ranges(&self, device: &HaDevice, ranges: &Vec<MemoryRange>) -> Result<(), MemoryError> {

        let flush_ranges = ranges.iter()
            .map(|range| {
                vk::MappedMemoryRange {
                    s_type: vk::StructureType::MappedMemoryRange,
                    p_next: ptr::null(),
                    memory: self.target().handle,
                    offset: range.offset,
                    size  : range.size,
                }
            }).collect::<Vec<_>>();

        unsafe {
            device.handle.flush_mapped_memory_ranges(&flush_ranges)
                .or(Err(MemoryError::FlushMemoryError))
        }
    }

    fn unmap(&self, device: &HaDevice) {

        unsafe {
            device.handle.unmap_memory(self.target().handle)
        }
    }
}

pub trait MemoryDstEntity {

    fn type_bytes(&self) -> vkint;
    fn aligment_size(&self) -> vkMemorySize;
}
