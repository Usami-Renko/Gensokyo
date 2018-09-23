
pub(crate) use self::flag::MemoryPropertyFlag;
pub(crate) use self::device::HaDeviceMemory;
pub(crate) use self::host::HaHostMemory;
pub(crate) use self::traits::{ HaMemoryAbstract, MemoryDataTransferable };

pub(crate) use self::traits::{ HaMemoryType, MemoryRange };
pub(crate) type MemPtr = *mut ::ash::vk::c_void;

mod device;
mod host;
mod flag;
mod traits;
