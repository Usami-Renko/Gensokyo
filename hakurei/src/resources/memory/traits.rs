
use vk::core::physical::HaPhyDevice;
use vk::core::device::HaDevice;

use vk::resources::buffer::BufferItem;
use vk::resources::memory::{ HaMemoryAbstract, MemoryRange };
use vk::resources::error::MemoryError;
use vk::utils::types::vkMemorySize;
use vk::utils::memory::MemoryWritePtr;

use resources::allocator::buffer::BufferAllocateInfos;
use resources::memory::staging::UploadStagingResource;

pub type HaMemoryEntity = Box<HaMemoryEntityAbs>;

pub trait HaMemoryEntityAbs: HaMemoryAbstract + MemoryDataUploadable {}

pub trait MemoryDataUploadable {

    fn prepare_data_transfer(&mut self, physical: &HaPhyDevice, device: &HaDevice, allocate_infos: &Option<BufferAllocateInfos>)
        -> Result<Option<UploadStagingResource>, MemoryError>;

    fn map_memory_ptr(&mut self, staging: &mut Option<UploadStagingResource>, item: &BufferItem, offset: vkMemorySize)
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
