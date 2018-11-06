
use ash::vk;

use core::device::HaDevice;

use resources::allocator::{ BufferAllocateInfos, BufMemAlloAbstract };
use resources::allocator::BufferInfosAllocatable;
use resources::memory::{ HaHostMemory, HaMemoryAbstract };
use resources::error::MemoryError;


pub(crate) struct HostBufMemAllocator {

    infos : Option<BufferAllocateInfos>,
    memory: Option<HaHostMemory>,
}

impl HostBufMemAllocator {

    pub fn new() -> HostBufMemAllocator {
        HostBufMemAllocator {
            infos : Some(BufferAllocateInfos::new()),
            memory: None,
        }
    }
}

impl BufMemAlloAbstract for HostBufMemAllocator {

    fn add_allocate(&mut self, space: vk::DeviceSize, _: Box<BufferInfosAllocatable>) {
        
        if let Some(ref mut infos) = self.infos {
            infos.spaces.push(space);
        }
    }

    fn allocate(&mut self, device: &HaDevice, size: vk::DeviceSize, mem_type_index: usize, mem_type: vk::MemoryType) -> Result<(), MemoryError> {
        
        let memory = HaHostMemory::allocate(device, size, mem_type_index, mem_type)?;
        self.memory = Some(memory);
        Ok(())
    }

    fn borrow_memory(&self) -> Result<&HaMemoryAbstract, MemoryError> {

        self.memory.as_ref()
            .and_then(|mem| Some(mem as &HaMemoryAbstract))
            .ok_or(MemoryError::MemoryNotYetAllocateError)
    }

    fn memory_map_if_need(&mut self, device: &HaDevice) -> Result<(), MemoryError> {

        if let Some(ref mut mem) = self.memory {
            mem.map_whole(device)?;
        }

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
