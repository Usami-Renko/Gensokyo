
use crate::core::GsDevice;

use crate::buffer::target::GsBuffer;
use crate::buffer::allocator::BufferAllocateInfos;
use crate::buffer::allocator::types::BufferMemoryTypeAbs;

use crate::memory::types::GsMemoryType;
use crate::memory::instance::GsBufferMemory;
use crate::memory::transfer::{ GsBufferDataUploader, GsBufferDataUpdater };

use crate::error::{ VkResult, VkError };

use std::marker::PhantomData;

pub struct GsBufferRepository<M>
    where
        M: BufferMemoryTypeAbs {

    phantom_type: PhantomData<M>,

    device  : GsDevice,
    buffers : Vec<GsBuffer>,
    memory  : GsBufferMemory,

    allocate_infos: BufferAllocateInfos,
}

impl<M> GsBufferRepository<M>
    where
        M: BufferMemoryTypeAbs {

    pub(crate) fn store(phantom_type: PhantomData<M>, device: GsDevice, buffers: Vec<GsBuffer>, memory: GsBufferMemory, allocate_infos: BufferAllocateInfos) -> GsBufferRepository<M> {

        GsBufferRepository {
            phantom_type,
            device, memory,

            buffers,
            allocate_infos,
        }
    }

    pub fn data_uploader(&mut self) -> VkResult<GsBufferDataUploader> {

        GsBufferDataUploader::new(&self.device, &self.memory, &self.allocate_infos)
    }

    pub fn data_updater(&mut self) -> VkResult<GsBufferDataUpdater> {

        match self.memory.memory_type() {
            | GsMemoryType::HostMemory => {
                GsBufferDataUpdater::new(&self.device, &self.memory)
            },
            | GsMemoryType::StagingMemory
            | GsMemoryType::CachedMemory
            | GsMemoryType::DeviceMemory => {
                return Err(VkError::device("This type of memory is not support to use updater."))
            },
        }
    }
}

impl<M> Drop for GsBufferRepository<M>
    where
        M: BufferMemoryTypeAbs {

    fn drop(&mut self) {

        self.buffers.iter().for_each(|buffer| buffer.destroy(&self.device));

        self.memory.destroy(&self.device);
    }
}
