
use ash::vk;

use core::device::HaDevice;

use resources::allocator::BufferAllocateInfos;
use resources::buffer::BufferBlockInfo;
use resources::memory::HaMemoryAbstract;
use resources::error::MemoryError;


/// Represent an trait object as a Buffer Memory Allocator.
pub(crate) trait BufMemAlloAbstract {

    fn add_allocate(&mut self, space: vk::DeviceSize, config: Box<BufferInfosAllocatable>);
    fn allocate(&mut self, device: &HaDevice, size: vk::DeviceSize, mem_type_index: usize, mem_type: vk::MemoryType) -> Result<(), MemoryError>;
    fn borrow_memory(&self) -> Result<&HaMemoryAbstract, MemoryError>;
    fn memory_map_if_need(&mut self, device: &HaDevice) -> Result<(), MemoryError>;
    fn take_memory(&mut self) -> Result<Box<HaMemoryAbstract>, MemoryError>;
    fn take_info(&mut self) -> BufferAllocateInfos;
}

pub(crate) trait BufferInfosAllocatable {

    fn to_staging_info(&self) -> Option<Box<BufferBlockInfo>> { None }
}
