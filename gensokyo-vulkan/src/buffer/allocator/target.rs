
use crate::core::device::GsDevice;
use crate::core::physical::GsPhyDevice;

use crate::buffer::target::GsBuffer;
use crate::buffer::traits::BufferBlockInfo;
use crate::buffer::instance::BufferInstanceType;
use crate::buffer::error::BufferError;
use crate::memory::{ MemoryFilter, MemoryDstEntity };
use crate::memory::AllocatorError;

use crate::buffer::allocator::index::BufferBlockIndex;
use crate::buffer::allocator::types::BufferMemoryTypeAbs;
use crate::buffer::allocator::memory::{ BufferAllocateInfos, BufMemAllocator };
use crate::buffer::allocator::distributor::GsBufferDistributor;

use crate::types::vkbytes;

use std::marker::PhantomData;


pub struct GsBufferAllocator<M> where M: BufferMemoryTypeAbs {

    phantom_type: PhantomData<M>,
    storage_type: M,

    physical: GsPhyDevice,
    device  : GsDevice,

    buffers : Vec<GsBuffer>,
    /// The size of each buffer occupy.
    spaces  : Vec<vkbytes>,

    allot_infos: BufferAllocateInfos,
    memory_filter: MemoryFilter,
}

impl<M> GsBufferAllocator<M> where M: BufferMemoryTypeAbs {

    pub fn new(physical: &GsPhyDevice, device: &GsDevice, storage_type: M) -> GsBufferAllocator<M> {

        GsBufferAllocator {
            phantom_type: PhantomData,
            storage_type,

            physical: physical.clone(),
            device  : device.clone(),

            buffers: vec![],
            spaces : vec![],

            allot_infos: BufferAllocateInfos::new(),
            memory_filter: MemoryFilter::new(physical, storage_type.memory_type()),
        }
    }

    pub fn append_buffer(&mut self, info: impl BufferBlockInfo) -> Result<BufferBlockIndex, AllocatorError> {

        let buffer = self.gen_buffer(&info, info.typ())?;
        let aligment_space = buffer.aligment_size();

        let index = info.to_block_index(self.buffers.len());

        self.spaces.push(aligment_space);
        self.buffers.push(buffer);
        self.allot_infos.push(aligment_space, info.into_desc());

        Ok(index)
    }

    fn gen_buffer(&mut self, info: &impl BufferBlockInfo, typ: BufferInstanceType) -> Result<GsBuffer, AllocatorError> {

        if typ.check_storage_validity(self.storage_type.memory_type()) == false {
            return Err(AllocatorError::UnsupportBufferUsage)
        }

        let buffer = info.as_desc_ref().build(&self.device, self.storage_type, None)?;
        self.memory_filter.filter(&buffer)?;

        Ok(buffer)
    }

    pub fn allocate(self) -> Result<GsBufferDistributor<M>, AllocatorError> {

        if self.buffers.is_empty() {
            return Err(AllocatorError::Buffer(BufferError::NoBufferAttachError))
        }

        // allocate memory
        let mut memory_allocator = BufMemAllocator::allot_memory(
            self.storage_type, &self.device, self.allot_infos, self.spaces.iter().sum(), &self.memory_filter
        )?;

        let mut buffers_to_distribute = vec![];

        // bind buffers to memory
        let mut offset = 0;
        for (i, buffer) in self.buffers.into_iter().enumerate() {

            memory_allocator.memory.bind_to_buffer(&self.device, &buffer, offset)?;
            offset += self.spaces[i];
            buffers_to_distribute.push(buffer);
        }

        memory_allocator.memory_map_if_need(&self.device)?;

        let (memory, allot_infos) = memory_allocator.take();

        let distributor = GsBufferDistributor::new(
            self.phantom_type,
            self.device, self.physical, memory, buffers_to_distribute, self.spaces, allot_infos
        );

        Ok(distributor)
    }

    pub fn reset(&mut self) {

        self.buffers.iter()
            .for_each(|buffer| buffer.destroy(&self.device));
        self.buffers.clear();

        self.spaces.clear();
        self.memory_filter.reset();
    }
}