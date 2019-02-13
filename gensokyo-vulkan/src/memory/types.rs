
use ash::vk;

pub enum GsMemoryType {

    HostMemory,
    CachedMemory,
    DeviceMemory,
    StagingMemory,
}

impl GsMemoryType {

    pub fn property_flags(&self) -> vk::MemoryPropertyFlags {
        match self {
            | GsMemoryType::HostMemory => {
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
            },
            | GsMemoryType::CachedMemory => {
                vk::MemoryPropertyFlags::HOST_CACHED
            },
            | GsMemoryType::DeviceMemory => {
                vk::MemoryPropertyFlags::DEVICE_LOCAL
            },
            | GsMemoryType::StagingMemory => {
                vk::MemoryPropertyFlags::HOST_VISIBLE | vk::MemoryPropertyFlags::HOST_COHERENT
            },
        }
    }
}
