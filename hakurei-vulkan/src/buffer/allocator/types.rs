
use ash::vk;

use core::device::HaDevice;

use memory::{ HaMemoryType, MemoryMappable };
use memory::types::{ Host, Device, Cached, Staging };
use memory::MemoryError;

pub trait BufferMemoryTypeAbs: Copy {
    const MEMORY_TYPE: HaMemoryType;

    fn memory_type(&self) -> HaMemoryType {
        Self::MEMORY_TYPE
    }

    fn complement_usage(&self, origin: vk::BufferUsageFlags) -> vk::BufferUsageFlags;

    fn map_memory_if_need(&self, _device: &HaDevice, _mapable_memory: &mut MemoryMappable) -> Result<(), MemoryError> { Ok(()) }
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
    const MEMORY_TYPE: HaMemoryType = HaMemoryType::HostMemory;

    fn complement_usage(&self, origin: vk::BufferUsageFlags) -> vk::BufferUsageFlags {
        origin
    }

    fn map_memory_if_need(&self, device: &HaDevice, mapable_memory: &mut MemoryMappable) -> Result<(), MemoryError> {
        mapable_memory.map_range(device, None)
    }
}

impl BufferMemoryTypeAbs for Cached {
    const MEMORY_TYPE: HaMemoryType = HaMemoryType::CachedMemory;

    fn complement_usage(&self, origin: vk::BufferUsageFlags) -> vk::BufferUsageFlags {
        origin | vk::BufferUsageFlags::TRANSFER_DST
    }
}

impl BufferMemoryTypeAbs for Device {
    const MEMORY_TYPE: HaMemoryType = HaMemoryType::DeviceMemory;

    fn complement_usage(&self, origin: vk::BufferUsageFlags) -> vk::BufferUsageFlags {
        origin | vk::BufferUsageFlags::TRANSFER_DST
    }
}

impl BufferMemoryTypeAbs for Staging {
    const MEMORY_TYPE: HaMemoryType = HaMemoryType::StagingMemory;

    fn complement_usage(&self, origin: vk::BufferUsageFlags) -> vk::BufferUsageFlags {
        origin| vk::BufferUsageFlags::TRANSFER_SRC
    }
}
