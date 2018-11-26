
use core::physical::HaPhyDevice;
use core::device::HaDevice;

use buffer::BufferBlock;
use memory::{ HaMemoryAbstract, MemoryRange };
use memory::error::MemoryError;

use types::vkbytes;
use utils::memory::MemoryWritePtr;

use buffer::allocator::BufferAllocateInfos;
use memory::instance::staging::UploadStagingResource;

pub type HaBufferMemory = Box<dyn HaBufferMemoryAbs>;
pub type HaImageMemory  = Box<dyn HaImageMemoryAbs>;

pub trait HaBufferMemoryAbs: HaMemoryAbstract + MemoryDataUploadable {}
pub trait HaImageMemoryAbs : HaMemoryAbstract {}

pub trait MemoryDataUploadable {

    fn prepare_data_transfer(&mut self, physical: &HaPhyDevice, device: &HaDevice, allocate_infos: &Option<BufferAllocateInfos>)
        -> Result<Option<UploadStagingResource>, MemoryError>;

    fn map_memory_ptr(&mut self, staging: &mut Option<UploadStagingResource>, block: &BufferBlock, offset: vkbytes)
        -> Result<(MemoryWritePtr, MemoryRange), MemoryError>;

    fn terminate_transfer(&mut self, device: &HaDevice, staging: &Option<UploadStagingResource>, ranges_to_flush: &Vec<MemoryRange>)
        -> Result<(), MemoryError>;
}

// TODO: Implement MemoryDataUpdatable.
//
//pub trait MemoryDataUpdatable {
//
//
//}
