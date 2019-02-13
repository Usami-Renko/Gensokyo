
use crate::core::GsDevice;

use crate::types::vkbytes;

use crate::memory::{ GsMemoryAbstract, MemoryFilter };
use crate::memory::types::GsMemoryType;
use crate::memory::instance::{ GsImageMemory, GsCachedMemory, GsDeviceMemory };

use crate::utils::phantom::{ Device, Cached };
use crate::error::VkResult;

pub trait ImageMemoryTypeAbs: Copy {
    const MEMORY_TYPE: GsMemoryType;

    fn memory_type(&self) -> GsMemoryType {
        Self::MEMORY_TYPE
    }

    fn allot_memory(&self, device: &GsDevice, size: vkbytes, filter: &MemoryFilter) -> VkResult<GsImageMemory>;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageStorageType {}

impl ImageStorageType {
    pub const DEVICE: Device = Device;
    pub const CACHED: Cached = Cached;
}

impl ImageMemoryTypeAbs for Device {
    const MEMORY_TYPE: GsMemoryType = GsMemoryType::DeviceMemory;

    fn allot_memory(&self, device: &GsDevice, size: vkbytes, filter: &MemoryFilter) -> VkResult<GsImageMemory> {

        let device_memory = GsDeviceMemory::allocate(device, size, filter)?;
        let memory_abs = Box::new(device_memory) as GsImageMemory;

        Ok(memory_abs)
    }
}

impl ImageMemoryTypeAbs for Cached {
    const MEMORY_TYPE: GsMemoryType = GsMemoryType::CachedMemory;

    fn allot_memory(&self, device: &GsDevice, size: vkbytes, filter: &MemoryFilter) -> VkResult<GsImageMemory> {

        let cached_memory = GsCachedMemory::allocate(device, size, filter)?;
        let memory_abs = Box::new(cached_memory) as GsImageMemory;

        Ok(memory_abs)
    }
}
