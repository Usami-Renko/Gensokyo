
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::buffer::BufferSubItem;
use resources::memory::{ HaMemoryAbstract, HaMemoryType, MemoryDataTransferable, MemoryPropertyFlag };
use resources::memory::{ MemPtr, MemoryRange };
use resources::error::MemoryError;

use utility::memory::MemoryWritePtr;
use utility::marker::VulkanFlags;

use std::ptr;

pub struct HaHostMemory  {

    handle     : vk::DeviceMemory,
    _size      : vk::DeviceSize,
    mem_type   : Option<vk::MemoryType>,

    map_status : MemoryMapStatus,
}

impl HaMemoryAbstract for HaHostMemory {

    fn handle(&self) -> vk::DeviceMemory {
        self.handle
    }

    fn flag(&self) -> vk::MemoryPropertyFlags {
        self.mem_type.as_ref().and_then(|m| Some(m.property_flags))
            .unwrap_or(vk::MemoryPropertyFlags::empty())
    }

    fn memory_type(&self) -> HaMemoryType {
        HaMemoryType::HostMemory
    }

    fn default_flag() -> vk::MemoryPropertyFlags {
        [
            MemoryPropertyFlag::HostVisibleBit,
            MemoryPropertyFlag::HostCoherentBit,
        ].flags()
    }

    fn allocate(device: &HaDevice, size: vk::DeviceSize, mem_type_index: usize, mem_type: Option<vk::MemoryType>) -> Result<HaHostMemory, MemoryError> {

        let allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MemoryAllocateInfo,
            p_next: ptr::null(),
            allocation_size: size,
            // an index identifying a memory type from the memoryTypes array of the vkPhysicalDeviceMemoryProperties structure.
            memory_type_index: mem_type_index as uint32_t,
        };

        let handle = unsafe {
            device.handle.allocate_memory(&allocate_info, None)
                .or(Err(MemoryError::AllocateMemoryError))?
        };

        let memory = HaHostMemory {
            handle,
            _size: size,
            mem_type,
            map_status: MemoryMapStatus::from_unmap(),
        };
        Ok(memory)
    }

    fn enable_map(&mut self, device: &HaDevice, is_enable: bool) -> Result<(), MemoryError> {

        if is_enable {
            if !self.map_status.is_map {
                let ptr = self.map_range(device, None)?;
                self.map_status.set_map(ptr);
            }
        } else {
            if self.map_status.is_map {
                self.unmap(device);
                self.map_status.invaild_map();
            }
        }

        Ok(())
    }
}

// Memory mapping Operation
impl HaHostMemory {

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

impl MemoryDataTransferable for HaHostMemory {

    fn prepare_data_transfer(&mut self, device: &HaDevice) -> Result<(), MemoryError> {

        self.enable_map(device, true)?;
        Ok(())
    }

    fn map_memory_ptr(&mut self, item: &BufferSubItem, offset: vk::DeviceSize) -> Result<(MemoryWritePtr, MemoryRange), MemoryError> {

        let ptr = unsafe {
            self.map_status.data_ptr.offset(offset as isize)
        };
        // let ptr = self.map_range(device, offset, item.size)?;

        let writer = MemoryWritePtr::new(ptr, item.size);
        let range = MemoryRange { offset, size: item.size };
        Ok((writer, range))
    }

    fn terminate_transfer(&mut self, device: &HaDevice, ranges_to_flush: &Vec<MemoryRange>) -> Result<(), MemoryError> {

        if !self.is_coherent_memroy() {
            // FIXME: the VkPhysicalDeviceLimits::nonCoherentAtomSize is not satified for flushing range.
            self.flush_ranges(device, ranges_to_flush)?;
        }

        Ok(())
    }
}


struct MemoryMapStatus {

    data_ptr: MemPtr,
    is_map  : bool,
}

impl MemoryMapStatus {

    fn from_unmap() -> MemoryMapStatus {
        MemoryMapStatus {
            data_ptr: ptr::null_mut(),
            is_map  : false,
        }
    }

    fn set_map(&mut self, ptr: MemPtr) {
        self.is_map = true;
        self.data_ptr = ptr;
    }

    fn invaild_map(&mut self) {
        self.data_ptr = ptr::null_mut();
        self.is_map = false;
    }
}
