
pub use self::host::HaHostMemory;
pub use self::cached::HaCachedMemory;
pub use self::device::HaDeviceMemory;
pub use self::staging::HaStagingMemory;

pub use self::staging::UploadStagingResource;
pub use self::traits::{ HaBufferMemory, HaImageMemory, HaBufferMemoryAbs };

mod traits;
mod host;
mod cached;
mod device;
mod staging;

