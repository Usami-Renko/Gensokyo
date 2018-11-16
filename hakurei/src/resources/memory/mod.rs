
pub use self::host::HaHostMemory;
pub use self::cached::HaCachedMemory;
pub use self::device::HaDeviceMemory;
pub use self::staging::HaStagingMemory;

pub(super) use self::staging::UploadStagingResource;
pub(super) use self::traits::{ HaMemoryEntity, HaMemoryEntityAbs, MemoryDataUploadable };

mod traits;
mod host;
mod cached;
mod device;
mod staging;

