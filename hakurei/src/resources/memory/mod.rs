
pub(crate) use self::flag::MemoryPropertyFlag;
pub(crate) use self::host::HaHostMemory;
pub(crate) use self::cached::HaCachedMemory;
pub(crate) use self::device::HaDeviceMemory;
pub(crate) use self::staging::{ HaStagingMemory, StagingUploader };
pub(crate) use self::traits::{ HaMemoryAbstract, MemoryDataUploadable, MemoryMapable };
pub(crate) use self::structs::{ HaMemoryType, MemoryRange, MemoryMapStatus, UploadStagingResource };

pub(crate) type MemPtr = *mut ::ash::vk::c_void;

mod host;
mod cached;
mod device;
mod staging;
mod flag;
mod traits;
mod structs;
