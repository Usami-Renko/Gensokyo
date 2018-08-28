
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;
use core::physical::HaPhysicalDevice;

use resources::memory::traits::HaMemoryAbstract;
use resources::error::MemoryError;

use std::ptr;

pub struct HaDeviceMemory {

    handle     : vk::DeviceMemory,
    _size      : vk::DeviceSize,
    _type_index: uint32_t,
    _mem_type  : vk::MemoryType,
}

impl HaMemoryAbstract for HaDeviceMemory {

    fn allocate(physical: &HaPhysicalDevice, device: &HaLogicalDevice, size: vk::DeviceSize, type_index: usize)
                -> Result<HaDeviceMemory, MemoryError> {

        let allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MemoryAllocateInfo,
            p_next: ptr::null(),
            allocation_size: size,
            // an index identifying a memory type from the memoryTypes array of the vkPhysicalDeviceMemoryProperties structure.
            memory_type_index: type_index as uint32_t,
        };

        let handle = unsafe {
            device.handle.allocate_memory(&allocate_info, None)
                .or(Err(MemoryError::AllocateMemoryError))?
        };
        let mem_type = physical.memory.memory_type(type_index);

        let memory = HaDeviceMemory {
            handle,
            _size: size,
            _type_index: type_index as uint32_t,
            _mem_type: mem_type,
        };
        Ok(memory)
    }

    fn bind(&self, device: &HaLogicalDevice, buffer_handle: vk::Buffer, offset: vk::DeviceSize) -> Result<(), MemoryError> {

        unsafe {
            device.handle.bind_buffer_memory(buffer_handle, self.handle, offset)
                .or(Err(MemoryError::BindMemoryError))?;
        }

        Ok(())
    }

    fn map(&self, device: &HaLogicalDevice, offset: vk::DeviceSize, size: vk::DeviceSize)
           -> Result<*mut vk::c_void, MemoryError> {

        let data_ptr = unsafe {
            device.handle.map_memory(
                // zero-based byte offset from the beginning of the memory object.
                self.handle,
                offset,
                // the size of the memory range to map, or VK_WHOLE_SIZE to map from offset to the end of the allocation.
                size,
                // flags is reserved for future use in API version 1.0.82.
                vk::MemoryMapFlags::empty(),
            ).or(Err(MemoryError::MapMemoryError))?
        };

        Ok(data_ptr)
    }

    fn unmap(&self, device: &HaLogicalDevice) {

        unsafe {
            device.handle.unmap_memory(self.handle)
        }
    }
}

impl HaDeviceMemory {

    pub fn cleanup(&self, device: &HaLogicalDevice) {
        unsafe {
            device.handle.free_memory(self.handle, None);
        }
    }
}

