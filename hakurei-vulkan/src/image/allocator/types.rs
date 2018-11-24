
use core::device::HaDevice;

use types::vkbytes;

use image::traits::ImageMemoryTypeAbs;

use memory::{ HaMemoryType, MemorySelector, HaMemoryAbstract };
use memory::instance::{ HaCachedMemory, HaDeviceMemory };
use memory::MemoryError;

pub struct Device;
pub struct Cached;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageStorageType {}

impl ImageStorageType {
    pub const DEVICE: Device = Device;
    pub const CACHED: Cached = Cached;
}

impl ImageMemoryTypeAbs for Device {

    fn memory_type(&self) -> HaMemoryType {
        HaMemoryType::DeviceMemory
    }

    fn allot_memory(&self, device: &HaDevice, size: vkbytes, selector: &MemorySelector) -> Result<Box<dyn HaMemoryAbstract>, MemoryError> {

        let device_memory = HaDeviceMemory::allocate(device, size, selector)?;
        let memory_abs = Box::new(device_memory) as Box<dyn HaMemoryAbstract>;

        Ok(memory_abs)
    }
}

impl ImageMemoryTypeAbs for Cached {

    fn memory_type(&self) -> HaMemoryType {
        HaMemoryType::CachedMemory
    }

    fn allot_memory(&self, device: &HaDevice, size: vkbytes, selector: &MemorySelector) -> Result<Box<dyn HaMemoryAbstract>, MemoryError> {

        let cached_memory = HaCachedMemory::allocate(device, size, selector)?;
        let memory_abs = Box::new(cached_memory) as Box<dyn HaMemoryAbstract>;

        Ok(memory_abs)
    }
}
