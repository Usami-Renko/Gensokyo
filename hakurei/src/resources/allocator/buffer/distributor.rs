
use vk::core::device::HaDevice;
use vk::core::physical::HaPhyDevice;

use vk::resources::buffer::{ HaBuffer, BufferItem };
use vk::resources::error::AllocatorError;
use vk::utils::types::vkMemorySize;

use resources::memory::HaMemoryEntity;
use resources::allocator::buffer::BufferBlockIndex;
use resources::allocator::buffer::infos::BufferAllocateInfos;
use resources::buffer::{ HaVertexBlock, HaIndexBlock, HaUniformBlock, HaImgsrcBlock };
use resources::repository::HaBufferRepository;

pub struct HaBufferDistributor {

    device  : HaDevice,
    physical: HaPhyDevice,
    memory  : HaMemoryEntity,

    buffers: Vec<HaBuffer>,
    spaces : Vec<vkMemorySize>,
    offsets: Vec<vkMemorySize>,
    allocate_infos: BufferAllocateInfos,
}

impl HaBufferDistributor {

    pub(super) fn new(device: HaDevice, physical: HaPhyDevice, memory: HaMemoryEntity, buffers: Vec<HaBuffer>, spaces: Vec<vkMemorySize>, allo_infos: BufferAllocateInfos) -> HaBufferDistributor {

        use utils::shortcuts::spaces_to_offsets;
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

    fn gen_buffer_item(&self, index: &BufferBlockIndex) -> BufferItem {

        BufferItem::new(
            &self.buffers[index.0],
            self.spaces[index.0],
            self.offsets[index.0]
        )
    }
}
