
use crate::core::device::GsDevice;
use crate::core::physical::GsPhyDevice;

use crate::buffer::target::GsBuffer;
use crate::buffer::allocator::BufferAllocateInfos;
use crate::buffer::allocator::types::BufferMemoryTypeAbs;

use crate::memory::types::GsMemoryType;
use crate::memory::instance::GsBufferMemory;
use crate::memory::transfer::BufferDataUploader;
use crate::memory::{ AllocatorError, MemoryError };

use crate::types::vkbytes;

use std::marker::PhantomData;

pub struct GsBufferRepository<M> where M: BufferMemoryTypeAbs {

    phantom_type: PhantomData<M>,

    device  : GsDevice,
    physical: GsPhyDevice,
    buffers : Vec<GsBuffer>,
    memory  : GsBufferMemory,

    /// The offset of each buffer in memory.
    offsets: Vec<vkbytes>,

    allocate_infos: BufferAllocateInfos,
}

impl<M> GsBufferRepository<M> where M: BufferMemoryTypeAbs {

    pub(crate) fn store(phantom_type: PhantomData<M>, device: GsDevice, physical: GsPhyDevice, buffers: Vec<GsBuffer>, memory: GsBufferMemory, allocate_infos: BufferAllocateInfos) -> GsBufferRepository<M> {

        use crate::utils::memory::spaces_to_offsets;
        let offsets = spaces_to_offsets(&allocate_infos.spaces);

        GsBufferRepository {
            phantom_type,
            device, physical, memory,

            buffers, offsets,
            allocate_infos,
        }
    }

    pub fn data_uploader(&mut self) -> Result<BufferDataUploader<M>, AllocatorError> {

        BufferDataUploader::new(self.phantom_type, &self.physical, &self.device, &self.memory, &self.allocate_infos)
    }

    // TODO: Implement actual updater.
    pub fn data_updater(&mut self) -> Result<BufferDataUploader<M>, AllocatorError> {

        match self.memory.memory_type() {
            | GsMemoryType::HostMemory => {
                BufferDataUploader::new(self.phantom_type, &self.physical, &self.device, &self.memory, &self.allocate_infos)
            },
            | GsMemoryType::StagingMemory
            | GsMemoryType::CachedMemory
            | GsMemoryType::DeviceMemory => {
                return Err(AllocatorError::Memory(MemoryError::MemoryUnableToUpdate))
            },
        }
    }

    pub fn cleanup(&mut self) {

        self.buffers.iter().for_each(|buffer|
            buffer.destroy(&self.device));

        self.memory.cleanup(&self.device);

        self.buffers.clear();
        self.offsets.clear();
    }
}
