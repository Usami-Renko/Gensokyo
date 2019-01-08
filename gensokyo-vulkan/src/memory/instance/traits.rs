
use crate::core::physical::GsPhyDevice;
use crate::core::device::GsDevice;

use crate::memory::GsMemoryAbstract;
use crate::memory::transfer::MemoryDataDelegate;
use crate::memory::error::MemoryError;

use crate::buffer::allocator::BufferAllocateInfos;

pub type GsBufferMemory = Box<dyn GsBufferMemoryAbs>;
pub type GsImageMemory  = Box<dyn GsImageMemoryAbs>;

pub trait GsBufferMemoryAbs: GsMemoryAbstract {

    fn to_agency(&self, device: &GsDevice, physical: &GsPhyDevice, allocate_infos: &BufferAllocateInfos) -> Result<Box<dyn MemoryDataDelegate>, MemoryError>;
}

pub trait GsImageMemoryAbs: GsMemoryAbstract {}
