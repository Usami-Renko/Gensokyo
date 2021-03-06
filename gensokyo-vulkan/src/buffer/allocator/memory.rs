
use crate::core::GsDevice;

use crate::buffer::target::BufferCI;
use crate::buffer::allocator::types::BufferMemoryTypeAbs;

use crate::memory::types::GsMemoryType;
use crate::memory::{ GsMemoryAbstract, MemoryMappable, MemoryFilter };
use crate::memory::instance::{ GsBufferMemory, GsHostMemory, GsCachedMemory, GsDeviceMemory, GsStagingMemory };

use crate::error::VkResult;
use crate::types::vkbytes;

#[derive(Default)]
pub struct BufferAllocateInfos {

    pub cis   : Vec<BufferCI>,
    pub spaces: Vec<vkbytes>,
}

impl BufferAllocateInfos {

    pub fn new() -> BufferAllocateInfos {
        Default::default()
    }

    pub fn push(&mut self, space: vkbytes, buffer_ci: BufferCI) {

        self.spaces.push(space);
        self.cis.push(buffer_ci);
    }
}


pub struct BufMemAllocator<M>
    where
        M: BufferMemoryTypeAbs {

    phantom_type: M,

    pub infos : BufferAllocateInfos,
    pub memory: GsBufferMemory,
}

impl<M> BufMemAllocator<M>
    where
        M: BufferMemoryTypeAbs {

    pub fn allot_memory(phantom_type: M, device: &GsDevice, infos: BufferAllocateInfos, size: vkbytes, filter: &MemoryFilter) -> VkResult<BufMemAllocator<M>> {

        let allocator = BufMemAllocator {
            phantom_type,
            infos,
            memory: phantom_type.memory_type().allot_buffer_memory(device, size, filter)?,
        };

        Ok(allocator)
    }

    pub fn memory_map_if_need(&mut self, device: &GsDevice) -> VkResult<()> {

        if let Some(mapable_memory) = self.memory.as_mut_mappable() {
            self.phantom_type.map_memory_if_need(device, mapable_memory as &mut MemoryMappable)
        } else {
            Ok(())
        }
    }

    pub fn take(self) -> (GsBufferMemory, BufferAllocateInfos) {

        (self.memory, self.infos)
    }
}

impl GsMemoryType {

    fn allot_buffer_memory(&self, device: &GsDevice, size: vkbytes, filter: &MemoryFilter) -> VkResult<GsBufferMemory> {

        let memory = match self {
            | GsMemoryType::HostMemory => {
                Box::new(GsHostMemory::allocate(device, size, filter)?) as GsBufferMemory
            },
            | GsMemoryType::CachedMemory => {
                Box::new(GsCachedMemory::allocate(device, size, filter)?) as GsBufferMemory
            },
            | GsMemoryType::DeviceMemory => {
                Box::new(GsDeviceMemory::allocate(device, size, filter)?) as GsBufferMemory
            },
            | GsMemoryType::StagingMemory => {
                Box::new(GsStagingMemory::allocate(device, size, filter)?) as GsBufferMemory
            },
        };

        Ok(memory)
    }
}
