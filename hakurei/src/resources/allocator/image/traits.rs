
use ash::vk;

use core::device::HaDevice;

use resources::memory::HaMemoryAbstract;
use resources::error::MemoryError;

/// Represent an trait object as a Image Memory Allocator.
pub(crate) trait ImgMemAlloAbstract {

    fn allocate(&mut self, device: &HaDevice, size: vk::DeviceSize, mem_type_index: usize, mem_type: Option<vk::MemoryType>) -> Result<(), MemoryError>;

    fn borrow_memory(&mut self) -> Result<&HaMemoryAbstract, MemoryError>;
    fn take_memory(&mut self) -> Result<Box<HaMemoryAbstract>, MemoryError>;
}
