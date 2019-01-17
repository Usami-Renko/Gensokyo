
use ash::vk;

use crate::core::device::GsDevice;

use crate::memory::MemoryMappable;
use crate::memory::types::GsMemoryType;

use crate::error::VkResult;
use crate::utils::phantom::{ Host, Device, Cached, Staging };

pub trait BufferMemoryTypeAbs: Copy + Sized {
    const MEMORY_TYPE: GsMemoryType;

    fn memory_type(&self) -> GsMemoryType {
        Self::MEMORY_TYPE
    }

    fn complement_usage(&self, origin: vk::BufferUsageFlags) -> vk::BufferUsageFlags;

    fn map_memory_if_need(&self, _device: &GsDevice, _mappable_memory: &mut MemoryMappable) -> VkResult<()> { Ok(()) }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BufferStorageType {}

impl BufferStorageType {

    pub const HOST   : Host    = Host;
    pub const DEVICE : Device  = Device;
    pub const CACHED : Cached  = Cached;
    pub const STAGING: Staging = Staging;
}

impl BufferMemoryTypeAbs for Host {
    const MEMORY_TYPE: GsMemoryType = GsMemoryType::HostMemory;

    fn complement_usage(&self, origin: vk::BufferUsageFlags) -> vk::BufferUsageFlags {
        origin
    }

    fn map_memory_if_need(&self, device: &GsDevice, mappable_memory: &mut MemoryMappable) -> VkResult<()> {
        mappable_memory.map_range(device, None)
    }
}

impl BufferMemoryTypeAbs for Cached {
    const MEMORY_TYPE: GsMemoryType = GsMemoryType::CachedMemory;

    fn complement_usage(&self, origin: vk::BufferUsageFlags) -> vk::BufferUsageFlags {
        origin | vk::BufferUsageFlags::TRANSFER_DST
    }
}

impl BufferMemoryTypeAbs for Device {
    const MEMORY_TYPE: GsMemoryType = GsMemoryType::DeviceMemory;

    fn complement_usage(&self, origin: vk::BufferUsageFlags) -> vk::BufferUsageFlags {
        origin | vk::BufferUsageFlags::TRANSFER_DST
    }
}

impl BufferMemoryTypeAbs for Staging {
    const MEMORY_TYPE: GsMemoryType = GsMemoryType::StagingMemory;

    fn complement_usage(&self, origin: vk::BufferUsageFlags) -> vk::BufferUsageFlags {
        origin| vk::BufferUsageFlags::TRANSFER_SRC
    }
}
