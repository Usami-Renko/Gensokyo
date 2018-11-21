
use memory::HaMemoryType;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageStorageType {
    Device,
    Cached,
}

impl ImageStorageType {

    pub fn memory_type(&self) -> HaMemoryType {
        match self {
            | ImageStorageType::Cached  => HaMemoryType::CachedMemory,
            | ImageStorageType::Device  => HaMemoryType::DeviceMemory,
        }
    }
}
