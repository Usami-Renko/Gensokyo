
use crate::core::device::GsDevice;
use crate::core::physical::GsPhyDevice;

use crate::buffer::{ GsBuffer, BufferBlock };
use crate::buffer::allocator::memory::BufferAllocateInfos;
use crate::buffer::allocator::types::BufferMemoryTypeAbs;
use crate::buffer::traits::BufferInstance;
use crate::buffer::repository::GsBufferRepository;
use crate::memory::instance::GsBufferMemory;
use crate::utils::phantom::Host;

use crate::buffer::instance::{
    GsVertexBuffer, IVertex,
    GsIndexBuffer, IIndices,
    GsUniformBuffer, IUniform,
    GsImgsrcBuffer, IImgSrc,
};

use crate::utils::api::{ GsAssignIndex, GsDistributeApi, GsDistIntoRepository };
use crate::types::vkbytes;

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

impl<M> GsDistributeApi<IVertex, GsVertexBuffer, GsBufferRepository<M>> for GsBufferDistributor<M>
    where M: BufferMemoryTypeAbs {

    fn acquire(&self, index: GsAssignIndex<IVertex>) -> GsVertexBuffer {

        let repo_index = index.assign_index;
        let buffer_block = self.gen_buffer_block(index.assign_index);
        GsVertexBuffer::new(buffer_block, index.take_info(), repo_index)
    }
}

impl<M> GsDistributeApi<IIndices, GsIndexBuffer, GsBufferRepository<M>> for GsBufferDistributor<M>
    where M: BufferMemoryTypeAbs {

    fn acquire(&self, index: GsAssignIndex<IIndices>) -> GsIndexBuffer {

        let repo_index = index.assign_index;
        let buffer_block = self.gen_buffer_block(index.assign_index);
        GsIndexBuffer::new(buffer_block, index.take_info(), repo_index)
    }
}

impl GsDistributeApi<IUniform, GsUniformBuffer, GsBufferRepository<Host>> for GsBufferDistributor<Host> {

    fn acquire(&self, index: GsAssignIndex<IUniform>) -> GsUniformBuffer {

        let repo_index = index.assign_index;
        let buffer_block = self.gen_buffer_block(index.assign_index);
        GsUniformBuffer::new(buffer_block, index.take_info(), repo_index)
    }
}

impl<M> GsDistributeApi<IImgSrc, GsImgsrcBuffer, GsBufferRepository<M>> for GsBufferDistributor<M>
    where M: BufferMemoryTypeAbs {

    fn acquire(&self, index: GsAssignIndex<IImgSrc>) -> GsImgsrcBuffer {

        let repo_index = index.assign_index;
        let buffer_block = self.gen_buffer_block(index.assign_index);
        GsImgsrcBuffer::new(buffer_block, index.take_info(), repo_index)
    }
}

impl<M> GsDistIntoRepository<GsBufferRepository<M>> for GsBufferDistributor<M>
    where M: BufferMemoryTypeAbs {

    fn into_repository(self) -> GsBufferRepository<M> {

        GsBufferRepository::store(self.phantom_type, self.device, self.physical, self.buffers, self.memory, self.allot_infos)
    }
}

impl<M> GsBufferDistributor<M> where M: BufferMemoryTypeAbs {

    pub(super) fn new(phantom_type: PhantomData<M>, device: GsDevice, physical: GsPhyDevice, memory: GsBufferMemory, buffers: Vec<GsBuffer>, spaces: Vec<vkbytes>, allot_infos: BufferAllocateInfos) -> GsBufferDistributor<M> {

        use crate::utils::memory::spaces_to_offsets;
        let offsets = spaces_to_offsets(&spaces);

        GsBufferDistributor {
            phantom_type, device, physical, memory, buffers, spaces, offsets, allot_infos,
        }
    }

    fn gen_buffer_block(&self, index: usize) -> BufferBlock {

        BufferBlock::new(&self.buffers[index], self.spaces[index], self.offsets[index])
    }
}
