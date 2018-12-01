
use core::physical::GsPhyDevice;
use core::device::GsDevice;

use memory::GsMemoryAbstract;
use memory::transfer::MemoryDataDelegate;
use memory::error::MemoryError;

use buffer::allocator::BufferAllocateInfos;

pub type GsBufferMemory = Box<dyn GsBufferMemoryAbs>;
pub type GsImageMemory  = Box<dyn GsImageMemoryAbs>;

pub trait GsBufferMemoryAbs: GsMemoryAbstract {

    fn to_agency(&self, device: &GsDevice, physical: &GsPhyDevice, allot_infos: &BufferAllocateInfos) -> Result<Box<dyn MemoryDataDelegate>, MemoryError>;
}

pub trait GsImageMemoryAbs: GsMemoryAbstract {}
