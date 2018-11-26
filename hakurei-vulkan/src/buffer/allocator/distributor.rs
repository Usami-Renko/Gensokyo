
use core::device::HaDevice;
use core::physical::HaPhyDevice;

use buffer::{ HaBuffer, BufferBlock };
use buffer::allocator::BufferBlockIndex;
use buffer::allocator::memory::BufferAllocateInfos;
use buffer::instance::{ HaVertexBlock, HaIndexBlock, HaUniformBlock, HaImgsrcBlock };
use buffer::repository::HaBufferRepository;
use memory::instance::HaBufferMemory;

use types::vkbytes;
use std::marker::PhantomData;

pub struct HaBufferDistributor<M> {

    phantom_type: PhantomData<M>,

    device   : HaDevice,
    physical : HaPhyDevice,
    memory   : HaBufferMemory,

    buffers : Vec<HaBuffer>,
    spaces  : Vec<vkbytes>,
    offsets : Vec<vkbytes>,

    allot_infos: BufferAllocateInfos,
}

impl<M> HaBufferDistributor<M> {

    pub(super) fn new(phantom_type: PhantomData<M>, device: HaDevice, physical: HaPhyDevice, memory: HaBufferMemory, buffers: Vec<HaBuffer>, spaces: Vec<vkbytes>, allot_infos: BufferAllocateInfos) -> HaBufferDistributor<M> {

        use utils::memory::spaces_to_offsets;
        let offsets = spaces_to_offsets(&spaces);

        HaBufferDistributor {
            phantom_type, device, physical, memory, buffers, spaces, offsets, allot_infos,
        }
    }

    pub fn acquire_vertex(&self, index: BufferBlockIndex) -> HaVertexBlock {

        HaVertexBlock::new(self.gen_buffer_item(&index), index.0)
    }

    pub fn acquire_index(&self, index: BufferBlockIndex) -> HaIndexBlock {

        HaIndexBlock::new(self.gen_buffer_item(&index), index.0)
    }

    pub fn acquire_uniform(&self, index: BufferBlockIndex) -> HaUniformBlock {

        unimplemented!()
    }

    pub fn acquire_imgsrc(&self, index: BufferBlockIndex) -> HaImgsrcBlock {

        HaImgsrcBlock::new(self.gen_buffer_item(&index), index.0)
    }

    pub fn into_repository(self) -> HaBufferRepository<M> {

        HaBufferRepository::store(self.phantom_type, self.device, self.physical, self.buffers, self.memory, self.allot_infos)
    }

    fn gen_buffer_item(&self, index: &BufferBlockIndex) -> BufferBlock {

        BufferBlock::new(
            &self.buffers[index.0],
            self.spaces[index.0],
            self.offsets[index.0],
        )
    }
}
