
use ash::vk;

use core::device::HaDevice;
use ash::version::DeviceV1_0;

use memory::selector::MemorySelector;

use memory::error::MemoryError;
use types::{ vkuint, vkbytes };

use std::ptr;

pub struct HaMemory {

    pub handle: vk::DeviceMemory,

    pub typ : vk::MemoryType,
    pub size: vkbytes,
}

impl HaMemory {

    pub fn allocate(device: &HaDevice, size: vkbytes, selector: &MemorySelector) -> Result<HaMemory, MemoryError> {

        let optimal_memory_index = selector.optimal_memory()?;
        let typ = selector.optimal_mem_type()?;

        let allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
            p_next: ptr::null(),
            allocation_size: size,
            // an index identifying a memory type from the memoryTypes array of the vkPhysicalDeviceMemoryProperties structure.
            memory_type_index: optimal_memory_index as vkuint,
        };

        let handle = unsafe {
            device.handle.allocate_memory(&allocate_info, None)
                .or(Err(MemoryError::AllocateMemoryError))?
        };

        let target = HaMemory {
            handle, typ, size,
        };

        Ok(target)
    }

    pub fn is_coherent_memroy(&self) -> bool {
        self.typ.property_flags.contains(vk::MemoryPropertyFlags::HOST_COHERENT)
    }

    pub fn cleanup(&self, device: &HaDevice) {

        unsafe {
            device.handle.free_memory(self.handle, None);
        }
    }
}
