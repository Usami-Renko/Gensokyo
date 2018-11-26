
use ash::vk;

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

pub struct MemoryMapStatus {

    pub data_ptr: Option<vkptr>,

    ranges: Vec<MemoryRange>,
    boundary: vkbytes,
}

impl MemoryMapStatus {

    pub fn from_unmap(boundary: vkbytes) -> MemoryMapStatus {
        MemoryMapStatus {
            data_ptr: None,
            ranges  : vec![],
            boundary,
        }
    }

    pub fn set_map(&mut self, ptr: vkptr, range: Option<MemoryRange>) {

        self.data_ptr = Some(ptr);

        if let Some(range) = range {
            self.ranges.push(range);
        } else {
            let whole_range = MemoryRange {
                offset: 0,
                size  : self.boundary,
            };
            self.ranges.push(whole_range);
        }
    }

    pub fn invaild_map(&mut self) {

        self.data_ptr = None;
        self.ranges.clear();
    }

    // TODO: This function has not implemented yet.
    pub fn is_range_available(&self, _range: Option<MemoryRange>) -> bool {

        self.ranges.is_empty()
    }

    pub fn is_mapping(&self) -> bool {

        self.data_ptr.is_some()
    }
}
