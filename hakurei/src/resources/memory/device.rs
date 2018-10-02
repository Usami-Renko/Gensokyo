
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::memory::{ HaMemoryAbstract, HaMemoryType, MemoryDataUploadable };
use resources::error::MemoryError;

use std::ptr;

pub struct HaDeviceMemory {

    handle     : vk::DeviceMemory,
    _size      : vk::DeviceSize,
    mem_type   : Option<vk::MemoryType>,
}

impl MemoryDataUploadable for HaDeviceMemory {}

impl HaMemoryAbstract for HaDeviceMemory {

    fn handle(&self) -> vk::DeviceMemory {
        self.handle
    }

    fn flag(&self) -> vk::MemoryPropertyFlags {
        self.mem_type.as_ref()
            .and_then(|m| Some(m.property_flags))
            .unwrap_or(vk::MemoryPropertyFlags::empty())
    }

    fn memory_type(&self) -> HaMemoryType {
        HaMemoryType::DeviceMemory
    }

    fn allocate(device: &HaDevice, size: vk::DeviceSize, mem_type_index: usize, mem_type: Option<vk::MemoryType>) -> Result<Self, MemoryError> {

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

        let memory = HaDeviceMemory {
            handle,
            _size: size,
            mem_type,
        };
        Ok(memory)
    }
}

