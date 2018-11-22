
use core::device::HaDevice;

use memory::{ HaMemoryAbstract, MemorySelector };
use memory::MemoryError;

use types::vkbytes;

use memory::instance::HaMemoryEntity;
use buffer::target::BufferDescInfo;
use buffer::allocator::infos::BufferAllocateInfos;

/// Represent an trait object as a Buffer Memory Allocator.
pub trait BufMemAlloAbstract {

    fn add_allocate(&mut self, space: vkbytes, desc_info: BufferDescInfo);

    fn allocate(&mut self, device: &HaDevice, size: vkbytes, selector: &MemorySelector) -> Result<(), MemoryError>;

    fn borrow_memory(&self) -> Result<&HaMemoryAbstract, MemoryError>;

    fn memory_map_if_need(&mut self, device: &HaDevice) -> Result<(), MemoryError>;

    fn take_memory(&mut self) -> Result<HaMemoryEntity, MemoryError>;

    fn take_info(&mut self) -> BufferAllocateInfos;
}
