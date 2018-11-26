
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use memory::target::HaMemory;
use memory::selector::MemorySelector;
use memory::structs::{ HaMemoryType, MemoryRange, MemoryMapStatus };
use memory::error::MemoryError;

use buffer::HaBuffer;
use image::HaImage;

use types::{ vkuint, vkbytes };

use std::ptr;

pub trait HaMemoryAbstract {

    fn memory_type(&self) -> HaMemoryType;

    fn target(&self) -> &HaMemory;

    fn allocate(device: &HaDevice, size: vkbytes, selector: &MemorySelector) -> Result<Self, MemoryError> where Self: Sized;

    fn as_mut_mapable(&mut self) -> Option<&mut MemoryMapable>;

    fn bind_to_buffer(&self, device: &HaDevice, buffer: &HaBuffer, memory_offset: vkbytes) -> Result<(), MemoryError> {

        unsafe {
            device.handle.bind_buffer_memory(buffer.handle, self.target().handle, memory_offset)
                .or(Err(MemoryError::BindMemoryError))
        }
    }

    fn bind_to_image(&self, device: &HaDevice, image: &HaImage, memory_offset: vkbytes) -> Result<(), MemoryError> {

        unsafe {
            device.handle.bind_image_memory(image.handle, self.target().handle, memory_offset)
                .or(Err(MemoryError::BindMemoryError))
        }
    }

    fn cleanup(&mut self, device: &HaDevice) {
        unsafe {
            device.handle.free_memory(self.target().handle, None);
        }
    }
}

/// A trait indicate a Memory is able to map.
pub trait MemoryMapable: HaMemoryAbstract {

    fn mut_status(&mut self) -> &mut MemoryMapStatus;

    /// Map specific range of the memory.
    ///
    /// If range is None, the function will map the whole memory.
    fn map_range(&mut self, device: &HaDevice, range: Option<MemoryRange>) -> Result<(), MemoryError> {

        let memory_handle = self.target().handle;
        let map_status = self.mut_status();

        unsafe {

            if let Some(range) = range {
                if map_status.is_range_available(Some(range.clone())) {

                    let data_ptr = device.handle.map_memory(
                        memory_handle,
                        // zero-based byte offset from the beginning of the memory object.
                        range.offset,
                        // the size of the memory range to map, or VK_WHOLE_SIZE to map from offset to the end of the allocation.
                        range.size,
                        // flags is reserved for future use in API version 1.1.82.
                        vk::MemoryMapFlags::empty(),
                    ).or(Err(MemoryError::MapMemoryError))?;

                    map_status.set_map(data_ptr, Some(range));
                } else {
                    return Err(MemoryError::DuplicateMapError)
                }
            } else {
                if map_status.is_range_available(None) {

                    let data_ptr = device.handle.map_memory(memory_handle, 0, vk::WHOLE_SIZE, vk::MemoryMapFlags::empty())
                        .or(Err(MemoryError::MapMemoryError))?;

                    map_status.set_map(data_ptr, None);
                } else {
                    return Err(MemoryError::DuplicateMapError)
                }
            }
        };

        Ok(())
    }

    fn flush_ranges(&self, device: &HaDevice, ranges: &Vec<MemoryRange>) -> Result<(), MemoryError> {

        let flush_ranges: Vec<vk::MappedMemoryRange> = ranges.iter()
            .map(|range| {
                vk::MappedMemoryRange {
                    s_type: vk::StructureType::MAPPED_MEMORY_RANGE,
                    p_next: ptr::null(),
                    memory: self.target().handle,
                    offset: range.offset,
                    size  : range.size,
                }
            }).collect();

        unsafe {
            device.handle.flush_mapped_memory_ranges(&flush_ranges)
                .or(Err(MemoryError::FlushMemoryError))
        }
    }

    fn unmap(&mut self, device: &HaDevice) {

        unsafe {
            device.handle.unmap_memory(self.target().handle)
        }

        let map_status = self.mut_status();
        map_status.invaild_map();
    }
}

pub trait MemoryDstEntity {

    fn type_bytes(&self) -> vkuint;
    fn aligment_size(&self) -> vkbytes;
}
