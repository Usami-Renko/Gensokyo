
use vk::core::device::HaDevice;
use vk::resources::memory::{ HaMemoryAbstract, MemorySelector };
use vk::resources::error::MemoryError;
use vk::utils::types::vkMemorySize;

use resources::allocator::image::traits::ImgMemAlloAbstract;
use resources::memory::HaDeviceMemory;

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

    fn allocate(&mut self, device: &HaDevice, size: vkMemorySize, selector: &MemorySelector) -> Result<(), MemoryError> {

        let memory = HaDeviceMemory::allocate(device, size, selector)?;
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
