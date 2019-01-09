
pub use self::host::GsHostMemory;
pub use self::cached::GsCachedMemory;
pub use self::device::GsDeviceMemory;
pub use self::staging::GsStagingMemory;

pub use self::staging::UploadStagingResource;
pub use self::traits::{ GsBufferMemory, GsImageMemory, GsBufferMemoryAbs };

mod traits;
mod host;
mod cached;
mod device;
mod staging;
