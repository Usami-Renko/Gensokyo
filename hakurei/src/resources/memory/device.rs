
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaDevice;
use core::physical::HaPhyDevice;

use resources::allocator::BufferAllocateInfos;
use resources::buffer::BufferItem;
use resources::memory::{ HaMemoryAbstract, HaMemoryType, MemoryRange };
use resources::memory::{ MemoryDataUploadable, UploadStagingResource, StagingUploader };
use resources::error::MemoryError;
use utility::memory::MemoryWritePtr;

use std::ptr;

pub struct HaDeviceMemory {

    handle     : vk::DeviceMemory,
    _size      : vk::DeviceSize,
    mem_type   : Option<vk::MemoryType>,
}

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

impl MemoryDataUploadable for HaDeviceMemory {

    fn prepare_data_transfer(&mut self, physical: &HaPhyDevice, device: &HaDevice, allocate_infos: &Option<BufferAllocateInfos>)
        -> Result<Option<UploadStagingResource>, MemoryError> {

        StagingUploader::prepare_data_transfer(physical, device, allocate_infos)
    }

    fn map_memory_ptr(&mut self, staging: &mut Option<UploadStagingResource>, item: &BufferItem, offset: vk::DeviceSize)
        -> Result<(MemoryWritePtr, MemoryRange), MemoryError> {

        StagingUploader::map_memory_ptr(staging, item, offset)
    }

    fn terminate_transfer(&mut self, device: &HaDevice, staging: &Option<UploadStagingResource>, ranges_to_flush: &Vec<MemoryRange>)
        -> Result<(), MemoryError> {

        StagingUploader::terminate_transfer(device, staging, ranges_to_flush)
    }
}
