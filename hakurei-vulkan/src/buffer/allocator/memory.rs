
use core::device::HaDevice;

use buffer::target::BufferDescInfo;
use buffer::allocator::types::BufferMemoryTypeAbs;

use memory::{ HaMemoryType, HaMemoryAbstract, MemoryMapable, MemorySelector };
use memory::instance::{ HaBufferMemory, HaHostMemory, HaCachedMemory, HaDeviceMemory, HaStagingMemory };
use memory::MemoryError;

use types::vkbytes;

#[derive(Default)]
pub struct BufferAllocateInfos {

    pub infos : Vec<BufferDescInfo>,
    pub spaces: Vec<vkbytes>,
}

impl BufferAllocateInfos {

    pub fn new() -> BufferAllocateInfos {
        Default::default()
    }

    pub fn push(&mut self, space: vkbytes, desc_info: BufferDescInfo) {

        self.spaces.push(space);
        self.infos.push(desc_info);
    }
}


pub struct BufMemAllocator<M> where M: BufferMemoryTypeAbs + Copy {

    phantom_type: M,

    pub infos : BufferAllocateInfos,
    pub memory: HaBufferMemory,
}

impl<M> BufMemAllocator<M> where M: BufferMemoryTypeAbs + Copy {

    pub fn allot_memory(phantom_type: M, device: &HaDevice, infos: BufferAllocateInfos, size: vkbytes, selector: &MemorySelector) -> Result<BufMemAllocator<M>, MemoryError> {

        let allocator = BufMemAllocator {
            phantom_type,
            infos,
            memory: phantom_type.memory_type().allot_buffer_memory(device, size, selector)?
        };

        Ok(allocator)
    }

    pub fn memory_map_if_need(&mut self, device: &HaDevice) -> Result<(), MemoryError> {

        if let Some(mapable_memory) = self.memory.as_mut_mapable() {
            self.phantom_type.map_memory_if_need(device, mapable_memory as &mut MemoryMapable)
        } else {
            Ok(())
        }
    }

    pub fn take(self) -> (HaBufferMemory, BufferAllocateInfos) {

        (self.memory, self.infos)
    }
}

impl HaMemoryType {

    fn allot_buffer_memory(&self, device: &HaDevice, size: vkbytes, selector: &MemorySelector) -> Result<HaBufferMemory, MemoryError> {

        let memory = match self {
            | HaMemoryType::HostMemory => {
                Box::new(HaHostMemory::allocate(device, size, selector)?) as HaBufferMemory
            },
            | HaMemoryType::CachedMemory => {
                Box::new(HaCachedMemory::allocate(device, size, selector)?) as HaBufferMemory
            },
            | HaMemoryType::DeviceMemory => {
                Box::new(HaDeviceMemory::allocate(device, size, selector)?) as HaBufferMemory
            },
            | HaMemoryType::StagingMemory => {
                Box::new(HaStagingMemory::allocate(device, size, selector)?) as HaBufferMemory
            },
        };

        Ok(memory)
    }
}
