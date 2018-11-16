
use vk::core::device::HaDevice;
use vk::resources::memory::{ HaMemoryAbstract, MemorySelector };
use vk::resources::error::MemoryError;
use vk::utils::types::vkMemorySize;

use resources::memory::HaMemoryEntity;
use resources::allocator::buffer::traits::{ BufMemAlloAbstract, BufferInfosAllocatable };
use resources::allocator::buffer::infos::BufferAllocateInfos;
use resources::memory::HaHostMemory;

pub struct HostBufMemAllocator {

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

    fn add_allocate(&mut self, space: vkMemorySize, _: Box<BufferInfosAllocatable>) {
        
        if let Some(ref mut infos) = self.infos {
            infos.spaces.push(space);
        }
    }

    fn allocate(&mut self, device: &HaDevice, size: vkMemorySize, selector: &MemorySelector) -> Result<(), MemoryError> {
        
        let memory = HaHostMemory::allocate(device, size, selector)?;
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

    fn take_memory(&mut self) -> Result<HaMemoryEntity, MemoryError> {

        self.memory.take()
            .and_then(|mem| Some(Box::new(mem) as HaMemoryEntity))
            .ok_or(MemoryError::MemoryNotYetAllocateError)
    }

    fn take_info(&mut self) -> BufferAllocateInfos {

        self.infos.take().unwrap()
    }
}
