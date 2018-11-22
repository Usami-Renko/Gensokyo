
use core::device::HaDevice;
use memory::{ HaMemoryAbstract, MemorySelector };
use memory::MemoryError;

use types::vkbytes;

use memory::instance::HaMemoryEntity;
use buffer::target::BufferDescInfo;
use buffer::allocator::traits::BufMemAlloAbstract;
use buffer::allocator::infos::BufferAllocateInfos;
use memory::instance::HaCachedMemory;

pub struct CachedBufMemAllocator {

    infos : Option<BufferAllocateInfos>,
    memory: Option<HaCachedMemory>,
}

impl CachedBufMemAllocator {

    pub fn new() -> CachedBufMemAllocator {
        CachedBufMemAllocator {
            infos : Some(BufferAllocateInfos::new()),
            memory: None,
        }
    }
}

impl BufMemAlloAbstract for CachedBufMemAllocator {

    fn add_allocate(&mut self, space: vkbytes, desc_info: BufferDescInfo) {
        if let Some(ref mut infos) = self.infos {
            infos.spaces.push(space);
            infos.infos.push(desc_info);
        }
    }

    fn allocate(&mut self, device: &HaDevice, size: vkbytes, selector: &MemorySelector) -> Result<(), MemoryError> {

        let memory = HaCachedMemory::allocate(device, size, selector)?;
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

    fn take_memory(&mut self) -> Result<HaMemoryEntity, MemoryError> {

        self.memory.take()
            .and_then(|mem| Some(Box::new(mem) as HaMemoryEntity))
            .ok_or(MemoryError::MemoryNotYetAllocateError)
    }

    fn take_info(&mut self) -> BufferAllocateInfos {

        self.infos.take().unwrap()
    }
}
