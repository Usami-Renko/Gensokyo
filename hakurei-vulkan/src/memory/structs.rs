
use ash::vk;

use types::{ vkptr, vkbytes };

use std::ptr;

pub struct MemoryRange {

    pub offset: vkbytes,
    pub size  : vkbytes,
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
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
            },
            | HaMemoryType::CachedMemory => {
                vk::MemoryPropertyFlags::HOST_CACHED
            },
            | HaMemoryType::DeviceMemory => {
                vk::MemoryPropertyFlags::DEVICE_LOCAL
            },
            | HaMemoryType::StagingMemory => {
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
            },
        }
    }
}


pub struct MemoryMapStatus {

    pub data_ptr: vkptr,
    pub is_map  : bool,
}

impl MemoryMapStatus {

    pub fn from_unmap() -> MemoryMapStatus {
        MemoryMapStatus {
            data_ptr: ptr::null_mut(),
            is_map  : false,
        }
    }

    pub fn set_map(&mut self, ptr: vkptr) {
        self.is_map = true;
        self.data_ptr = ptr;
    }

    pub fn invaild_map(&mut self) {
        self.data_ptr = ptr::null_mut();
        self.is_map = false;
    }
}
