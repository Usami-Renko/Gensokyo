
use resources::allocator::ImgMemAlloAbstract;
use resources::allocator::{ DeviceImgMemAllocator, CachedImgMemAllocator };
use resources::memory::HaMemoryType;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageStorageType {
    Device,
    Cached,
}

impl ImageStorageType {

    pub(crate) fn allocator(&self) -> Box<ImgMemAlloAbstract> {
        match self {
            | ImageStorageType::Device => Box::new(DeviceImgMemAllocator::new()),
            | ImageStorageType::Cached => Box::new(CachedImgMemAllocator::new()),
        }
    }

    pub(crate) fn memory_type(&self) -> HaMemoryType {
        match self {
            | ImageStorageType::Cached  => HaMemoryType::CachedMemory,
            | ImageStorageType::Device  => HaMemoryType::DeviceMemory,
        }
    }
}
