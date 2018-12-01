
use core::device::GsDevice;
use core::physical::GsPhyDevice;

use buffer::target::GsBuffer;
use buffer::allocator::BufferAllocateInfos;
use buffer::allocator::types::BufferMemoryTypeAbs;

use memory::types::GsMemoryType;
use memory::instance::GsBufferMemory;
use memory::transfer::BufferDataUploader;
use memory::{ AllocatorError, MemoryError };

use types::vkbytes;
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

        use utils::memory::spaces_to_offsets;
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
            buffer.cleanup(&self.device));

        self.memory.cleanup(&self.device);

        self.buffers.clear();
        self.offsets.clear();
    }
}
