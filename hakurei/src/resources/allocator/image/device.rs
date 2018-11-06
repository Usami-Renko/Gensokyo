
use ash::vk;

use core::device::HaDevice;

use resources::allocator::ImgMemAlloAbstract;
use resources::memory::{ HaDeviceMemory, HaMemoryAbstract };
use resources::error::MemoryError;

pub struct DeviceImgMemAllocator {

    memory: Option<HaDeviceMemory>,
}

impl DeviceImgMemAllocator {

    pub fn new() -> DeviceImgMemAllocator {
        DeviceImgMemAllocator {

            memory: None,
        }
    }
}

impl ImgMemAlloAbstract for DeviceImgMemAllocator {

    fn allocate(&mut self, device: &HaDevice, size: vk::DeviceSize, mem_type_index: usize, mem_type: vk::MemoryType) -> Result<(), MemoryError> {

        let memory = HaDeviceMemory::allocate(device, size, mem_type_index, mem_type)?;
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
