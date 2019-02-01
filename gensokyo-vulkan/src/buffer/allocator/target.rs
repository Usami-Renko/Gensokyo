
use crate::core::GsDevice;

use crate::buffer::target::GsBuffer;
use crate::buffer::instance::types::BufferCIApi;
use crate::memory::{ MemoryFilter, MemoryDstEntity };
use crate::error::{ VkResult, VkError };

use crate::buffer::allocator::types::BufferMemoryTypeAbs;
use crate::buffer::allocator::memory::{ BufferAllocateInfos, BufMemAllocator };
use crate::buffer::allocator::distributor::GsBufferDistributor;
use crate::utils::allot::{ GsAssignIndex, GsAllocatorApi, GsAllotIntoDistributor };
use crate::types::vkbytes;

use std::marker::PhantomData;


pub struct GsBufferAllocator<M>
    where
        M: BufferMemoryTypeAbs {

    phantom_type: PhantomData<M>,
    storage_type: M,

    device  : GsDevice,

    buffers : Vec<GsBuffer>,
    /// The size of each buffer occupy.
    spaces  : Vec<vkbytes>,

    allot_infos: BufferAllocateInfos,
    memory_filter: MemoryFilter,
}

impl<M, I> GsAllocatorApi<I, GsBufferDistributor<M>> for GsBufferAllocator<M>
    where
        I: BufferCIApi,
        M: BufferMemoryTypeAbs {

    type AssignResult = VkResult<GsAssignIndex<I::IConveyor>>;

    fn assign(&mut self, info: I) -> Self::AssignResult {

        // check if the usage of buffer valid.
        if I::check_storage_validity(self.storage_type.memory_type()) == false {
            return Err(VkError::device("The type of buffer is not support on this allocator."))
        }

        let mut info = info; // make it mutable.
        info.check_limits(&self.device);

        let buffer_ci = GsBuffer::new(info.estimate_size(), I::VK_FLAG);
        let buffer = buffer_ci.build(&self.device, self.storage_type)?;
        self.memory_filter.filter(&buffer)?;

        let dst_index = GsAssignIndex {
            convey_info: info.into_index(),
            assign_index: self.buffers.len(),
        };

        // get buffer alignment.
        let alignment_space = buffer.alignment_size();

        self.spaces.push(alignment_space);
        self.buffers.push(buffer);
        self.allot_infos.push(alignment_space, buffer_ci);

        Ok(dst_index)
    }
}

impl<M> GsAllotIntoDistributor<GsBufferDistributor<M>> for GsBufferAllocator<M>
    where
        M: BufferMemoryTypeAbs {

    fn allocate(self) -> VkResult<GsBufferDistributor<M>> {

        if self.buffers.is_empty() {
            return Err(VkError::device("Failed to get attachment content to the buffer"))
        }

        // allocate memory.
        let mut memory_allocator = BufMemAllocator::allot_memory(
            self.storage_type, &self.device, self.allot_infos, self.spaces.iter().sum(), &self.memory_filter
        )?;

        let mut buffers_to_distribute = Vec::with_capacity(self.buffers.len());

        // bind buffers to memory.
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
            self.device, memory, buffers_to_distribute, self.spaces, allot_infos
        );

        Ok(distributor)
    }

    fn reset(&mut self) {

        self.buffers.iter().for_each(|b| b.destroy(&self.device));
        self.buffers.clear();
        self.spaces.clear();
        self.memory_filter.reset();
    }
}

impl<M> GsBufferAllocator<M>
    where
        M: BufferMemoryTypeAbs {

    pub fn create(device: &GsDevice, storage_type: M) -> GsBufferAllocator<M> {

        GsBufferAllocator {
            phantom_type: PhantomData,
            storage_type,

            device  : device.clone(),

            buffers: vec![],
            spaces : vec![],

            allot_infos: BufferAllocateInfos::new(),
            memory_filter: MemoryFilter::new(device, storage_type.memory_type()),
        }
    }

    pub fn assign_v2<R>(&mut self, delegate: &impl GsBufferAllocatable<M, R>) -> VkResult<R> {

        let allot_func = delegate.allot_func();
        allot_func(delegate, self)
    }
}

pub trait GsBufferAllocatable<M, R>
    where
        Self: Sized,
        M: BufferMemoryTypeAbs {

    fn allot_func(&self) -> Box<dyn Fn(&Self, &mut GsBufferAllocator<M>) -> VkResult<R>>;
}
