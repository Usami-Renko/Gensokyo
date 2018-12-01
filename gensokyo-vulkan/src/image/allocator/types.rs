
use core::device::GsDevice;

use types::vkbytes;

use memory::{ GsMemoryAbstract, MemorySelector };
use memory::types::{ GsMemoryType, Device, Cached };
use memory::instance::{ GsImageMemory, GsCachedMemory, GsDeviceMemory };
use memory::MemoryError;

pub trait ImageMemoryTypeAbs: Copy {
    const MEMORY_TYPE: GsMemoryType;

    fn memory_type(&self) -> GsMemoryType {
        Self::MEMORY_TYPE
    }

    fn allot_memory(&self, device: &GsDevice, size: vkbytes, selector: &MemorySelector) -> Result<GsImageMemory, MemoryError>;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageStorageType {}

impl ImageStorageType {
    pub const DEVICE: Device = Device;
    pub const CACHED: Cached = Cached;
}

impl ImageMemoryTypeAbs for Device {
    const MEMORY_TYPE: GsMemoryType = GsMemoryType::DeviceMemory;

    fn allot_memory(&self, device: &GsDevice, size: vkbytes, selector: &MemorySelector) -> Result<GsImageMemory, MemoryError> {

        let device_memory = GsDeviceMemory::allocate(device, size, selector)?;
        let memory_abs = Box::new(device_memory) as GsImageMemory;

        Ok(memory_abs)
    }
}

impl ImageMemoryTypeAbs for Cached {
    const MEMORY_TYPE: GsMemoryType = GsMemoryType::CachedMemory;

    fn allot_memory(&self, device: &GsDevice, size: vkbytes, selector: &MemorySelector) -> Result<GsImageMemory, MemoryError> {

        let cached_memory = GsCachedMemory::allocate(device, size, selector)?;
        let memory_abs = Box::new(cached_memory) as GsImageMemory;

        Ok(memory_abs)
    }
}
