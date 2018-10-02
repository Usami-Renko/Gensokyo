
use ash::vk;

use core::device::HaDevice;

use resources::allocator::ImgMemAlloAbstract;
use resources::memory::{ HaCachedMemory, HaMemoryAbstract };
use resources::error::MemoryError;

pub struct CachedImgMemAllocator {

    memory: Option<HaCachedMemory>,
}

impl CachedImgMemAllocator {

    pub fn new() -> CachedImgMemAllocator {
        CachedImgMemAllocator {

            memory: None,
        }
    }
}

impl ImgMemAlloAbstract for CachedImgMemAllocator {

    fn allocate(&mut self, device: &HaDevice, size: vk::DeviceSize, mem_type_index: usize, mem_type: Option<vk::MemoryType>) -> Result<(), MemoryError> {

        let memory = HaCachedMemory::allocate(device, size, mem_type_index, mem_type)?;
        self.memory = Some(memory);
        Ok(())
    }

    fn borrow_memory(&mut self) -> Result<&HaMemoryAbstract, MemoryError> {

        self.memory.as_ref()
            .and_then(|mem| Some(mem as &HaMemoryAbstract))
            .ok_or(MemoryError::MemoryNotYetAllocateError)
    }

    fn take_memory(&mut self) -> Result<Box<HaMemoryAbstract>, MemoryError> {

        self.memory.take()
            .and_then(|mem| Some(Box::new(mem) as Box<HaMemoryAbstract>))
            .ok_or(MemoryError::MemoryNotYetAllocateError)
    }
}
