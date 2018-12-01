
use core::device::GsDevice;
use core::physical::GsPhyDevice;

use buffer::{ GsBuffer, BufferBlock };
use buffer::allocator::BufferBlockIndex;
use buffer::allocator::memory::BufferAllocateInfos;
use buffer::allocator::types::BufferMemoryTypeAbs;
use buffer::instance::{ GsVertexBlock, GsIndexBlock, GsUniformBlock, GsImgsrcBlock };

use buffer::repository::GsBufferRepository;
use memory::instance::GsBufferMemory;

use types::vkbytes;
use std::marker::PhantomData;

pub struct GsBufferDistributor<M> where M: BufferMemoryTypeAbs {

    phantom_type: PhantomData<M>,

    device   : GsDevice,
    physical : GsPhyDevice,
    memory   : GsBufferMemory,

    buffers : Vec<GsBuffer>,
    spaces  : Vec<vkbytes>,
    offsets : Vec<vkbytes>,

    allot_infos: BufferAllocateInfos,
}

impl<M> GsBufferDistributor<M> where M: BufferMemoryTypeAbs {

    pub(super) fn new(phantom_type: PhantomData<M>, device: GsDevice, physical: GsPhyDevice, memory: GsBufferMemory, buffers: Vec<GsBuffer>, spaces: Vec<vkbytes>, allot_infos: BufferAllocateInfos) -> GsBufferDistributor<M> {

        use utils::memory::spaces_to_offsets;
        let offsets = spaces_to_offsets(&spaces);

        GsBufferDistributor {
            phantom_type, device, physical, memory, buffers, spaces, offsets, allot_infos,
        }
    }

    pub fn acquire_vertex(&self, index: BufferBlockIndex) -> GsVertexBlock {

        GsVertexBlock::new(self.gen_buffer_item(&index), index.0)
    }

    pub fn acquire_index(&self, index: BufferBlockIndex) -> GsIndexBlock {

        GsIndexBlock::new(self.gen_buffer_item(&index), index.0)
    }

    pub fn acquire_uniform(&self, index: BufferBlockIndex) -> GsUniformBlock {

        unimplemented!()
    }

    pub fn acquire_imgsrc(&self, index: BufferBlockIndex) -> GsImgsrcBlock {

        GsImgsrcBlock::new(self.gen_buffer_item(&index), index.0)
    }

    pub fn into_repository(self) -> GsBufferRepository<M> {

        GsBufferRepository::store(self.phantom_type, self.device, self.physical, self.buffers, self.memory, self.allot_infos)
    }

    fn gen_buffer_item(&self, index: &BufferBlockIndex) -> BufferBlock {

        BufferBlock::new(
            &self.buffers[index.0],
            self.spaces[index.0],
            self.offsets[index.0],
        )
    }
}
