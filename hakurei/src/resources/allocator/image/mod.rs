
pub use self::base::{ HaImageAllocator, ImageStorageType };

pub(crate) use self::base::ImageAllocateInfo;
pub(crate) use self::traits::ImgMemAlloAbstract;
pub(crate) use self::device::DeviceImgMemAllocator;
pub(crate) use self::cached::CachedImgMemAllocator;

mod base;
mod device;
mod cached;
mod traits;
