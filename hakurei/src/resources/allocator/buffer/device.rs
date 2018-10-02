
use ash::vk;

use core::device::HaDevice;

use resources::allocator::{ BufMemAlloAbstract, BufferAllocateInfos };
use resources::buffer::BufferConfigAbstract;
use resources::memory::{ HaDeviceMemory, HaMemoryAbstract };
use resources::error::MemoryError;


pub(crate) struct DeviceBufMemAllocator {

    infos : Option<BufferAllocateInfos>,
    memory: Option<HaDeviceMemory>,
}

impl DeviceBufMemAllocator {

    pub fn new() -> DeviceBufMemAllocator {
        DeviceBufMemAllocator {
            infos : Some(BufferAllocateInfos::new()),
            memory: None,
        }
    }
}

impl BufMemAlloAbstract for DeviceBufMemAllocator {

    fn add_allocate(&mut self, space: vk::DeviceSize, config: Box<BufferConfigAbstract>) {

        if let Some(ref mut infos) = self.infos {
            infos.spaces.push(space);
            infos.configs.push(config);
        }
    }

    fn allocate(&mut self, device: &HaDevice, size: vk::DeviceSize, mem_type_index: usize, mem_type: Option<vk::MemoryType>) -> Result<(), MemoryError> {

        let memory = HaDeviceMemory::allocate(device, size, mem_type_index, mem_type)?;
        self.memory = Some(memory);
        Ok(())
    }

    fn borrow_memory(&self) -> Result<&HaMemoryAbstract, MemoryError> {

        self.memory.as_ref()
            .and_then(|mem| Some(mem as &HaMemoryAbstract))
            .ok_or(MemoryError::MemoryNotYetAllocateError)
    }

    fn memory_map_if_need(&mut self, _: &HaDevice) -> Result<(), MemoryError> {
        // ignore it.
        Ok(())
    }

    fn take_memory(&mut self) -> Result<Box<HaMemoryAbstract>, MemoryError> {

        self.memory.take()
            .and_then(|mem| Some(Box::new(mem) as Box<HaMemoryAbstract>))
            .ok_or(MemoryError::MemoryNotYetAllocateError)
    }

    fn take_info(&mut self) -> BufferAllocateInfos {

        self.infos.take().unwrap()
    }
}
