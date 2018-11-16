
use ash::vk;

use core::device::HaDevice;
use ash::version::DeviceV1_0;

use resources::memory::MemorySelector;
use resources::error::MemoryError;
use utils::types::{ vkint, vkMemorySize };

use std::ptr;

pub struct HaMemory {

    pub handle: vk::DeviceMemory,

    typ : vk::MemoryType,
    size: vkMemorySize,
}

impl HaMemory {

    pub fn allocate(device: &HaDevice, size: vkMemorySize, selector: &MemorySelector) -> Result<HaMemory, MemoryError> {

        let optimal_memory_index = selector.optimal_memory()?;
        let typ = selector.optimal_mem_type()?;

        let allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MemoryAllocateInfo,
            p_next: ptr::null(),
            allocation_size: size,
            // an index identifying a memory type from the memoryTypes array of the vkPhysicalDeviceMemoryProperties structure.
            memory_type_index: optimal_memory_index as vkint,
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
        self.typ.property_flags.subset(vk::MEMORY_PROPERTY_HOST_COHERENT_BIT)
    }

    pub fn cleanup(&self, device: &HaDevice) {

        unsafe {
            device.handle.free_memory(self.handle, None);
        }
    }
}
