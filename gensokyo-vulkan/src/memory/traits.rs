
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::GsDevice;

use memory::target::GsMemory;
use memory::selector::MemorySelector;
use memory::types::GsMemoryType;
use memory::utils::{ MemoryRange, MemoryMapStatus };
use memory::error::MemoryError;

use buffer::GsBuffer;
use image::GsImage;

use types::{ vkuint, vkbytes };

use std::ptr;

pub trait GsMemoryAbstract {

    fn memory_type(&self) -> GsMemoryType;

    fn target(&self) -> &GsMemory;

    fn allocate(device: &GsDevice, size: vkbytes, selector: &MemorySelector) -> Result<Self, MemoryError> where Self: Sized;

    fn as_mut_mapable(&mut self) -> Option<&mut MemoryMappable>;

    fn bind_to_buffer(&self, device: &GsDevice, buffer: &GsBuffer, memory_offset: vkbytes) -> Result<(), MemoryError> {

        unsafe {
            device.handle.bind_buffer_memory(buffer.handle, self.target().handle, memory_offset)
                .or(Err(MemoryError::BindMemoryError))
        }
    }

    fn bind_to_image(&self, device: &GsDevice, image: &GsImage, memory_offset: vkbytes) -> Result<(), MemoryError> {

        unsafe {
            device.handle.bind_image_memory(image.handle, self.target().handle, memory_offset)
                .or(Err(MemoryError::BindMemoryError))
        }
    }

    fn cleanup(&mut self, device: &GsDevice) {
        unsafe {
            device.handle.free_memory(self.target().handle, None);
        }
    }
}

/// A trait indicate a Memory is able to map.
pub trait MemoryMappable {

    fn map_handle(&self) -> vk::DeviceMemory;

    fn mut_status(&mut self) -> &mut MemoryMapStatus;

    /// Map specific range of the memory.
    ///
    /// If range is None, the function will map the whole memory.
    fn map_range(&mut self, device: &GsDevice, range: Option<MemoryRange>) -> Result<(), MemoryError> {

        let data_ptr = unsafe {

            if let Some(range) = range {

                device.handle.map_memory(
                    self.map_handle(),
                    // zero-based byte offset from the beginning of the memory object.
                    range.offset,
                    // the size of the memory range to map, or VK_WHOLE_SIZE to map from offset to the end of the allocation.
                    range.size,
                    // flags is reserved for future use in API version 1.1.82.
                    vk::MemoryMapFlags::empty(),
                ).or(Err(MemoryError::MapMemoryError))?

            } else {
                device.handle.map_memory(self.map_handle(), 0, vk::WHOLE_SIZE, vk::MemoryMapFlags::empty())
                    .or(Err(MemoryError::MapMemoryError))?
            }
        };

        let map_status = self.mut_status();
        map_status.set_map(data_ptr);

        Ok(())
    }

    fn flush_ranges(&self, device: &GsDevice, ranges: &Vec<MemoryRange>) -> Result<(), MemoryError> {

        let flush_ranges: Vec<vk::MappedMemoryRange> = ranges.iter()
            .map(|range| {
                vk::MappedMemoryRange {
                    s_type: vk::StructureType::MAPPED_MEMORY_RANGE,
                    p_next: ptr::null(),
                    memory: self.map_handle(),
                    offset: range.offset,
                    size  : range.size,
                }
            }).collect();

        unsafe {
            device.handle.flush_mapped_memory_ranges(&flush_ranges)
                .or(Err(MemoryError::FlushMemoryError))
        }
    }

    fn unmap(&mut self, device: &GsDevice) {

        unsafe {
            device.handle.unmap_memory(self.map_handle())
        }

        let map_status = self.mut_status();
        map_status.invaild_map();
    }
}

pub trait MemoryDstEntity {

    fn type_bytes(&self) -> vkuint;
    fn aligment_size(&self) -> vkbytes;
}
