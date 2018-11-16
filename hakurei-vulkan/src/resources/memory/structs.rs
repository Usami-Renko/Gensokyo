
use ash::vk;

use resources::memory::flag::MemoryPropertyFlag;

use utils::types::MemPtr;
use utils::marker::VulkanFlags;

use std::ptr;

pub struct MemoryRange {

    pub offset: vk::DeviceSize,
    pub size  : vk::DeviceSize,
}

pub enum HaMemoryType {

    HostMemory,
    CachedMemory,
    DeviceMemory,
    StagingMemory,
}

impl HaMemoryType {

    pub fn property_flags(&self) -> vk::MemoryPropertyFlags {
        match self {
            | HaMemoryType::HostMemory => {
                [
                    MemoryPropertyFlag::HostVisibleBit,
                    MemoryPropertyFlag::HostCoherentBit,
                ].flags()
            },
            | HaMemoryType::CachedMemory => {
                [
                    MemoryPropertyFlag::HostCachedBit,
                ].flags()
            },
            | HaMemoryType::DeviceMemory => {
                [
                    MemoryPropertyFlag::DeviceLocalBit,
                ].flags()
            },
            | HaMemoryType::StagingMemory => {
                [
                    MemoryPropertyFlag::HostVisibleBit,
                    MemoryPropertyFlag::HostCoherentBit,
                ].flags()
            },
        }
    }
}


pub struct MemoryMapStatus {

    pub data_ptr: MemPtr,
    pub is_map  : bool,
}

impl MemoryMapStatus {

    pub fn from_unmap() -> MemoryMapStatus {
        MemoryMapStatus {
            data_ptr: ptr::null_mut(),
            is_map  : false,
        }
    }

    pub fn set_map(&mut self, ptr: MemPtr) {
        self.is_map = true;
        self.data_ptr = ptr;
    }

    pub fn invaild_map(&mut self) {
        self.data_ptr = ptr::null_mut();
        self.is_map = false;
    }
}
