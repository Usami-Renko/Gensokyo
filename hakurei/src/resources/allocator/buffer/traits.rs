
use vk::core::device::HaDevice;
use vk::resources::buffer::BufferBlockInfo;
use vk::resources::memory::{ HaMemoryAbstract, MemorySelector };
use vk::resources::error::MemoryError;
use vk::utils::types::vkMemorySize;

use resources::memory::HaMemoryEntity;
use resources::buffer::BufferBranch;
use resources::allocator::buffer::infos::BufferAllocateInfos;

/// Represent an trait object as a Buffer Memory Allocator.
pub trait BufMemAlloAbstract {

    fn add_allocate(&mut self, space: vkMemorySize, config: Box<BufferInfosAllocatable>);

    fn allocate(&mut self, device: &HaDevice, size: vkMemorySize, selector: &MemorySelector) -> Result<(), MemoryError>;

    fn borrow_memory(&self) -> Result<&HaMemoryAbstract, MemoryError>;

    fn memory_map_if_need(&mut self, device: &HaDevice) -> Result<(), MemoryError>;

    fn take_memory(&mut self) -> Result<HaMemoryEntity, MemoryError>;

    fn take_info(&mut self) -> BufferAllocateInfos;
}

pub trait BufferInfosAllocatable {

    // TODO: Make this to trait const property.
    fn branch_type(&self) -> BufferBranch;

    fn to_staging_info(&self) -> Option<Box<BufferBlockInfo>> { None }
}
