
use core::device::HaDevice;
use core::physical::HaPhyDevice;

use buffer::{ HaBuffer, BufferBlock };
use buffer::allocator::BufferBlockIndex;
use buffer::allocator::infos::BufferAllocateInfos;
use buffer::instance::{ HaVertexBlock, HaIndexBlock, HaUniformBlock, HaImgsrcBlock };
use buffer::repository::HaBufferRepository;
use memory::instance::HaMemoryEntity;

use types::vkbytes;

pub struct HaBufferDistributor {

    device  : HaDevice,
    physical: HaPhyDevice,
    memory  : HaMemoryEntity,

    buffers: Vec<HaBuffer>,
    spaces : Vec<vkbytes>,
    offsets: Vec<vkbytes>,
    allocate_infos: BufferAllocateInfos,
}

impl HaBufferDistributor {

    pub(super) fn new(device: HaDevice, physical: HaPhyDevice, memory: HaMemoryEntity, buffers: Vec<HaBuffer>, spaces: Vec<vkbytes>, allo_infos: BufferAllocateInfos) -> HaBufferDistributor {

        use utils::memory::spaces_to_offsets;
        let offsets = spaces_to_offsets(&spaces);

        HaBufferDistributor {
            device, physical, memory, buffers, spaces, offsets,
            allocate_infos: allo_infos,
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

    pub fn into_repository(self) -> HaBufferRepository {

        HaBufferRepository::store(self.device, self.physical, self.buffers, self.memory, self.allocate_infos)
    }

    fn gen_buffer_item(&self, index: &BufferBlockIndex) -> BufferBlock {

        BufferBlock::new(
            &self.buffers[index.0],
            self.spaces[index.0],
            self.offsets[index.0],
        )
    }
}
