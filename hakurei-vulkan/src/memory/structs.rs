
use ash::vk;

use memory::traits::MemoryMappable;

use types::{ vkptr, vkbytes };

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
pub struct MemoryMapStatus {

    /// The begining data ptr of the whole memory.
    data_ptr: Option<vkptr>,
}

impl MemoryMapStatus {

    pub fn from_unmap() -> MemoryMapStatus {

        MemoryMapStatus {
            data_ptr: None,
        }
    }

    pub unsafe fn data_ptr(&self, offset: vkbytes) -> Option<vkptr> {

        self.data_ptr.and_then(|ptr| {
            Some(ptr.offset(offset as isize))
        })
    }

    pub fn set_map(&mut self, ptr: vkptr) {

        self.data_ptr = Some(ptr);
    }

    pub fn invaild_map(&mut self) {

        self.data_ptr = None;
    }

    pub fn is_mapping(&self) -> bool {

        self.data_ptr.is_some()
    }
}

pub struct MemoryMapAlias {

    pub handle: vk::DeviceMemory,
    pub status: MemoryMapStatus,
    pub is_coherent: bool,
}

impl MemoryMappable for MemoryMapAlias {

    fn map_handle(&self) -> vk::DeviceMemory {
        self.handle
    }

    fn mut_status(&mut self) -> &mut MemoryMapStatus {
        &mut self.status
    }
}
