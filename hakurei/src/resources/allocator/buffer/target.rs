
use vk::core::device::HaDevice;
use vk::core::physical::HaPhyDevice;

use vk::resources::buffer::{ HaBuffer, BufferBlockInfo, BufferItem, BufferStorageType };
use vk::resources::memory::{ HaMemoryType, MemorySelector };
use vk::resources::memory::MemoryDstEntity;
use vk::resources::error::{ BufferError, AllocatorError };
use vk::utils::types::vkMemorySize;

use resources::allocator::buffer::BufferBlockIndex;
use resources::allocator::buffer::{
    distributor::HaBufferDistributor,
    traits::{ BufMemAlloAbstract, BufferInfosAllocatable },
    host::HostBufMemAllocator,
    cached::CachedBufMemAllocator,
    device::DeviceBufMemAllocator,
    staging::StagingBufMemAllocator,
};

use resources::buffer::{ VertexBlockInfo, IndexBlockInfo, UniformBlockInfo, ImgsrcBlockInfo, BufferBranch };

pub struct HaBufferAllocator {

    physical: HaPhyDevice,
    device  : HaDevice,

    buffers : Vec<HaBuffer>,
    /// The size of each buffer occupy.
    spaces  : Vec<vkMemorySize>,

    storage_type: BufferStorageType,
    allocator: Box<BufMemAlloAbstract>,
    memory_selector : MemorySelector,
}

impl HaBufferAllocator {

    pub(crate) fn new(physical: &HaPhyDevice, device: &HaDevice, storage_type: BufferStorageType) -> HaBufferAllocator {

        HaBufferAllocator {
            physical: physical.clone(),
            device  : device.clone(),

            buffers: vec![],
            spaces : vec![],

            storage_type,
            allocator: gen_allocator(storage_type),
            memory_selector: MemorySelector::init(physical, storage_type.memory_type()),
        }
    }

    pub fn append_buffer(&mut self, info: impl BufferBlockInfo + BufferInfosAllocatable + 'static) -> Result<BufferBlockIndex, AllocatorError> {

        let buffer_info = self.gen_buffer(&info, info.branch_type())?;
        let index = BufferBlockIndex(self.buffers.len());

        self.spaces.push(buffer_info.aligment_space);
        self.buffers.push(buffer_info.buffer);

        self.allocator.add_allocate(buffer_info.aligment_space, Box::new(info));

        Ok(index)
    }

    fn gen_buffer(&mut self, info: &impl BufferBlockInfo, branch: BufferBranch) -> Result<BufferGenInfo, AllocatorError> {

        if branch.check_storage_validity(self.storage_type) == false {
            return Err(AllocatorError::UnsupportBufferUsage)
        }

        let buffer = info.build(&self.device, None, self.storage_type)?;
        self.memory_selector.try(&buffer)?;

        let aligment_space = buffer.aligment_size();

        let info = BufferGenInfo {
            buffer, aligment_space,
        };
        Ok(info)
    }

    pub fn allocate(mut self) -> Result<HaBufferDistributor, AllocatorError> {

        if self.buffers.is_empty() {
            return Err(AllocatorError::Buffer(BufferError::NoBufferAttachError))
        }

        // allocate memory

        self.allocator.allocate(
            &self.device, self.spaces.iter().sum(), &self.memory_selector
        )?;

        let mut buffers_to_distribute = vec![];
        {
            let memory_allocated = self.allocator.borrow_memory()?;

            // bind buffers to memory
            let mut offset = 0;
            for (i, buffer) in self.buffers.drain(..).enumerate() {
                memory_allocated.bind_to_buffer(&self.device, &buffer, offset)?;
                offset += self.spaces[i];
                buffers_to_distribute.push(buffer);
            }
        }

        self.allocator.memory_map_if_need(&self.device)?;

        let distributor = HaBufferDistributor::new(
            self.device, self.physical,
            self.allocator.take_memory()?,
            buffers_to_distribute,
            self.spaces,
            self.allocator.take_info()
        );

        Ok(distributor)
    }

    pub fn storage_type(&self) -> BufferStorageType {
        self.storage_type
    }

    pub fn reset(&mut self) {

        self.buffers.iter()
            .for_each(|buffer| buffer.cleanup(&self.device));
        self.buffers.clear();

        self.spaces.clear();
        self.memory_selector.reset();
    }
}

struct BufferGenInfo {

    buffer: HaBuffer,
    aligment_space: vkMemorySize,
}

fn gen_allocator(storage: BufferStorageType) -> Box<BufMemAlloAbstract> {

    match storage {
        | BufferStorageType::Host    => Box::new(HostBufMemAllocator::new()),
        | BufferStorageType::Cached  => Box::new(CachedBufMemAllocator::new()),
        | BufferStorageType::Device  => Box::new(DeviceBufMemAllocator::new()),
        | BufferStorageType::Staging => Box::new(StagingBufMemAllocator::new()),
    }
}
