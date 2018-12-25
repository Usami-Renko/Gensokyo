
use ash::vk;

use ash::version::DeviceV1_0;

use crate::core::device::GsDevice;
use crate::memory::filter::MemoryFilter;

use crate::memory::error::MemoryError;
use crate::types::vkbytes;

use std::ptr;

pub struct GsMemory {

    pub handle: vk::DeviceMemory,

    pub typ : vk::MemoryType,
    pub size: vkbytes,
}

impl GsMemory {

    pub fn allocate(device: &GsDevice, size: vkbytes, filter: &MemoryFilter) -> Result<GsMemory, MemoryError> {

        let optimal_memory_index = filter.optimal_memory()?;
        let typ = filter.optimal_mem_type()?;

        let allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MEMORY_ALLOCATE_INFO,
            p_next: ptr::null(),
            allocation_size: size,
            // an index identifying a memory type from the memoryTypes array of the vkPhysicalDeviceMemoryProperties structure.
            memory_type_index: optimal_memory_index as _,
        };

        let handle = unsafe {
            device.handle.allocate_memory(&allocate_info, None)
                .or(Err(MemoryError::AllocateMemoryError))?
        };

        let target = GsMemory {
            handle, typ, size,
        };

        Ok(target)
    }

    pub fn is_coherent_memroy(&self) -> bool {
        self.typ.property_flags.contains(vk::MemoryPropertyFlags::HOST_COHERENT)
    }

    pub fn cleanup(&self, device: &GsDevice) {

        unsafe {
            device.handle.free_memory(self.handle, None);
        }
    }
}
