
use vk::core::device::HaDevice;

use vk::resources::memory::{ HaMemoryAbstract, MemorySelector };
use vk::resources::error::MemoryError;
use vk::utils::types::vkMemorySize;

/// Represent an trait object as a Image Memory Allocator.
pub trait ImgMemAlloAbstract {

    fn allocate(&mut self, device: &HaDevice, size: vkMemorySize, selector: &MemorySelector) -> Result<(), MemoryError>;

    fn borrow_memory(&mut self) -> Result<&HaMemoryAbstract, MemoryError>;
    fn take_memory(&mut self) -> Result<Box<HaMemoryAbstract>, MemoryError>;
}
