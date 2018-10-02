
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;
use core::physical::HaPhyDevice;

use resources::buffer::{ HaBuffer, BufferSubItem };
use resources::memory::{ HaMemoryType, MemPtr, MemoryRange, UploadStagingResource };
use resources::allocator::BufferAllocateInfos;
use resources::image::HaImage;
use resources::error::MemoryError;

use utility::memory::MemoryWritePtr;

use std::ptr;

pub(crate) trait HaMemoryAbstract: MemoryDataUploadable {

    fn handle(&self) -> vk::DeviceMemory;
    fn flag(&self) -> vk::MemoryPropertyFlags;
    fn memory_type(&self) -> HaMemoryType;

    fn allocate(device: &HaDevice, size: vk::DeviceSize, mem_type_index: usize, mem_type: Option<vk::MemoryType>) -> Result<Self, MemoryError> where Self: Sized;

    // bindings
    fn bind_to_buffer(&self, device: &HaDevice, buffer: &HaBuffer, memory_offset: vk::DeviceSize) -> Result<(), MemoryError> {
        unsafe {
            device.handle.bind_buffer_memory(buffer.handle, self.handle(), memory_offset)
                .or(Err(MemoryError::BindMemoryError))
        }
    }
    fn bind_to_image(&self, device: &HaDevice, image: &HaImage, memory_offset: vk::DeviceSize) -> Result<(), MemoryError> {
        unsafe {
            device.handle.bind_image_memory(image.handle, self.handle(), memory_offset)
                .or(Err(MemoryError::BindMemoryError))
        }
    }

    // cleaning
    fn cleanup(&self, device: &HaDevice) {
        unsafe {
            device.handle.free_memory(self.handle(), None);
        }
    }

    fn is_coherent_memroy(&self) -> bool {
        self.flag().subset(vk::MEMORY_PROPERTY_HOST_COHERENT_BIT)
    }
}

/// A trait indicate a Memory is able to map.
pub(crate) trait MemoryMapable: HaMemoryAbstract {

    /// Map specific range of the memory.
    ///
    /// If range is None, the function will map the whole memory.
    fn map_range(&self, device: &HaDevice, range: Option<MemoryRange>) -> Result<MemPtr, MemoryError> {

        let data_ptr = unsafe {
            if let Some(range) = range {
                device.handle.map_memory(
                    self.handle(),
                    // zero-based byte offset from the beginning of the memory object.
                    range.offset,
                    // the size of the memory range to map, or VK_WHOLE_SIZE to map from offset to the end of the allocation.
                    range.size,
                    // flags is reserved for future use in API version 1.1.82.
                    vk::MemoryMapFlags::empty(),
                ).or(Err(MemoryError::MapMemoryError))?
            } else {
                device.handle.map_memory(self.handle(), 0, vk::VK_WHOLE_SIZE, vk::MemoryMapFlags::empty())
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
                    memory: self.handle(),
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
            device.handle.unmap_memory(self.handle())
        }
    }
}

pub(crate) trait MemoryDataUploadable {

    fn prepare_data_transfer(&mut self, physical: &HaPhyDevice, device: &HaDevice, allocate_infos: &Option<BufferAllocateInfos>) -> Result<Option<UploadStagingResource>, MemoryError> {

        let staging = UploadStagingResource::new(physical, device, allocate_infos)?;

        Ok(Some(staging))
    }

    fn map_memory_ptr(&mut self, staging: &mut Option<UploadStagingResource>, item: &BufferSubItem, _offset: vk::DeviceSize) -> Result<(MemoryWritePtr, MemoryRange), MemoryError> {

        if let Some(ref mut staging) = staging {

            let result = staging.append_dst_item(item)?;
            Ok(result)
        } else {
            Err(MemoryError::AllocateInfoMissing)
        }
    }

    fn terminate_transfer(&mut self, device: &HaDevice, staging: &Option<UploadStagingResource>, _ranges_to_flush: &Vec<MemoryRange>) -> Result<(), MemoryError> {

        if let Some(staging) = staging {
            staging.transfer(device)
                .or(Err(MemoryError::BufferToBufferCopyError))?
        } else {
            return Err(MemoryError::AllocateInfoMissing)
        }

        Ok(())
    }
}


// TODO: Implement MemoryDataUpdatable.
//
//pub(crate) trait MemoryDataUpdatable {
//
//
//}
