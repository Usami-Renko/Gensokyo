
use core::device::HaDevice;

use types::vkbytes;

use memory::{ HaMemoryType, HaMemoryAbstract, MemorySelector };
use memory::types::{ Device, Cached };
use memory::instance::{ HaImageMemory, HaCachedMemory, HaDeviceMemory };
use memory::MemoryError;

pub trait ImageMemoryTypeAbs: Copy {
    const MEMORY_TYPE: HaMemoryType;

    fn memory_type(&self) -> HaMemoryType {
        Self::MEMORY_TYPE
    }

    fn allot_memory(&self, device: &HaDevice, size: vkbytes, selector: &MemorySelector) -> Result<HaImageMemory, MemoryError>;
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageStorageType {}

impl ImageStorageType {
    pub const DEVICE: Device = Device;
    pub const CACHED: Cached = Cached;
}

impl ImageMemoryTypeAbs for Device {
    const MEMORY_TYPE: HaMemoryType = HaMemoryType::DeviceMemory;

    fn allot_memory(&self, device: &HaDevice, size: vkbytes, selector: &MemorySelector) -> Result<HaImageMemory, MemoryError> {

        let device_memory = HaDeviceMemory::allocate(device, size, selector)?;
        let memory_abs = Box::new(device_memory) as HaImageMemory;

        Ok(memory_abs)
    }
}

impl ImageMemoryTypeAbs for Cached {
    const MEMORY_TYPE: HaMemoryType = HaMemoryType::CachedMemory;

    fn allot_memory(&self, device: &HaDevice, size: vkbytes, selector: &MemorySelector) -> Result<HaImageMemory, MemoryError> {

        let cached_memory = HaCachedMemory::allocate(device, size, selector)?;
        let memory_abs = Box::new(cached_memory) as HaImageMemory;

        Ok(memory_abs)
    }
}
