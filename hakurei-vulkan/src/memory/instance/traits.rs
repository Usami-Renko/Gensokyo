
use core::physical::HaPhyDevice;
use core::device::HaDevice;

use memory::HaMemoryAbstract;
use memory::transfer::MemoryDataDelegate;
use memory::error::MemoryError;

use buffer::allocator::BufferAllocateInfos;

pub type HaBufferMemory = Box<dyn HaBufferMemoryAbs>;
pub type HaImageMemory  = Box<dyn HaImageMemoryAbs>;

pub trait HaBufferMemoryAbs: HaMemoryAbstract {

    fn to_agency(&self, device: &HaDevice, physical: &HaPhyDevice, allot_infos: &BufferAllocateInfos) -> Result<Box<dyn MemoryDataDelegate>, MemoryError>;
}

pub trait HaImageMemoryAbs: HaMemoryAbstract {}
