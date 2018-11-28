
use core::device::HaDevice;
use core::physical::HaPhyDevice;

use buffer::target::HaBuffer;
use buffer::traits::BufferBlockInfo;
use buffer::instance::BufferInstanceType;
use buffer::error::BufferError;
use memory::{ MemorySelector, MemoryDstEntity };
use memory::AllocatorError;

use buffer::allocator::index::BufferBlockIndex;
use buffer::allocator::types::BufferMemoryTypeAbs;
use buffer::allocator::memory::{ BufferAllocateInfos, BufMemAllocator };
use buffer::allocator::distributor::HaBufferDistributor;

use types::vkbytes;

use std::marker::PhantomData;

pub struct HaBufferAllocator<M> where M: BufferMemoryTypeAbs {

    phantom_type: PhantomData<M>,
    storage_type: M,

    physical: HaPhyDevice,
    device  : HaDevice,

    buffers : Vec<HaBuffer>,
    /// The size of each buffer occupy.
    spaces  : Vec<vkbytes>,

    allot_infos: BufferAllocateInfos,
    memory_selector: MemorySelector,
}

impl<M> HaBufferAllocator<M> where M: BufferMemoryTypeAbs {

    pub fn new(physical: &HaPhyDevice, device: &HaDevice, storage_type: M) -> HaBufferAllocator<M> {

        HaBufferAllocator {
            phantom_type: PhantomData,
            storage_type,

            physical: physical.clone(),
            device  : device.clone(),

            buffers: vec![],
            spaces : vec![],

            allot_infos: BufferAllocateInfos::new(),
            memory_selector: MemorySelector::init(physical, storage_type.memory_type()),
        }
    }

    pub fn append_buffer(&mut self, info: impl BufferBlockInfo) -> Result<BufferBlockIndex, AllocatorError> {

        let buffer = self.gen_buffer(&info, info.typ())?;
        let aligment_space = buffer.aligment_size();

        let index = BufferBlockIndex(self.buffers.len());

        self.spaces.push(aligment_space);
        self.buffers.push(buffer);
        self.allot_infos.push(aligment_space, info.into_desc());

        Ok(index)
    }

    fn gen_buffer(&mut self, info: &impl BufferBlockInfo, typ: BufferInstanceType) -> Result<HaBuffer, AllocatorError> {

        if typ.check_storage_validity(self.storage_type.memory_type()) == false {
            return Err(AllocatorError::UnsupportBufferUsage)
        }

        let buffer = info.as_desc_ref().build(&self.device, self.storage_type, None)?;
        self.memory_selector.try(&buffer)?;

        Ok(buffer)
    }

    pub fn allocate(mut self) -> Result<HaBufferDistributor<M>, AllocatorError> {

        if self.buffers.is_empty() {
            return Err(AllocatorError::Buffer(BufferError::NoBufferAttachError))
        }

        // allocate memory
        let mut memory_allocator = BufMemAllocator::allot_memory(
            self.storage_type, &self.device, self.allot_infos, self.spaces.iter().sum(), &self.memory_selector
        )?;

        let mut buffers_to_distribute = vec![];

        // bind buffers to memory
        let mut offset = 0;
        for (i, buffer) in self.buffers.drain(..).enumerate() {

            memory_allocator.memory.bind_to_buffer(&self.device, &buffer, offset)?;
            offset += self.spaces[i];
            buffers_to_distribute.push(buffer);
        }

        memory_allocator.memory_map_if_need(&self.device)?;

        let (memory, allot_infos) = memory_allocator.take();

        let distributor = HaBufferDistributor::new(
            self.phantom_type,
            self.device, self.physical, memory, buffers_to_distribute, self.spaces, allot_infos
        );

        Ok(distributor)
    }

    pub fn reset(&mut self) {

        self.buffers.iter()
            .for_each(|buffer| buffer.cleanup(&self.device));
        self.buffers.clear();

        self.spaces.clear();
        self.memory_selector.reset();
    }
}
