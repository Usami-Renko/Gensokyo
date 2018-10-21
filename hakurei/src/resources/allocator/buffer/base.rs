
use ash::vk;

use core::device::HaDevice;
use core::physical::{ HaPhyDevice, MemorySelector };

use resources::allocator::{ BufMemAlloAbstract, BufferInfosAllocatable };
use resources::allocator::{ HostBufMemAllocator, CachedBufMemAllocator, DeviceBufMemAllocator, StagingBufMemAllocator };
use resources::buffer::{ HaBuffer, BufferBlockInfo, BufferSubItem };
use resources::buffer::{
    HaVertexBlock, VertexBlockInfo,
    HaIndexBlock, IndexBlockInfo,
    HaUniformBlock, UniformBlockInfo,
    HaImgsrcBlock, ImgsrcBlockInfo,
};
use resources::memory::{ HaMemoryType, MemoryPropertyFlag };
use resources::repository::HaBufferRepository;
use resources::error::{ BufferError, AllocatorError };

use utility::marker::VulkanEnum;


pub struct HaBufferAllocator {

    physical: HaPhyDevice,
    device  : HaDevice,

    buffers : Vec<HaBuffer>,
    /// The size of each buffer occupy.
    spaces  : Vec<vk::DeviceSize>,

    ty: BufferStorageType,
    allocator: Box<BufMemAlloAbstract>,
    require_mem_flag: vk::MemoryPropertyFlags,
    memory_selector : MemorySelector,
}

impl  HaBufferAllocator {

    pub(crate) fn new(physical: &HaPhyDevice, device: &HaDevice, ty: BufferStorageType) -> HaBufferAllocator {

        HaBufferAllocator {
            physical: physical.clone(),
            device: device.clone(),

            buffers: vec![],
            spaces : vec![],

            ty,
            allocator: ty.allocator(),
            require_mem_flag: ty.memory_type().property_flags(),
            memory_selector: MemorySelector::init(physical),
        }
    }

    pub fn append_vertex(&mut self, info: VertexBlockInfo) -> Result<HaVertexBlock, AllocatorError> {

        let buffer_info = self.gen_buffer(&info, BufferBranch::Vertex)?;
        let block = HaVertexBlock::from(&info, buffer_info.item);
        self.append_buffer(buffer_info.buffer, buffer_info.aligment_space, info);

        Ok(block)
    }

    pub fn append_index(&mut self, info: IndexBlockInfo) -> Result<HaIndexBlock, AllocatorError> {

        let buffer_info = self.gen_buffer(&info, BufferBranch::Index)?;
        let block = HaIndexBlock::from(&info, buffer_info.item);
        self.append_buffer(buffer_info.buffer, buffer_info.aligment_space, info);

        Ok(block)
    }

    pub fn append_uniform(&mut self, info: UniformBlockInfo) -> Result<HaUniformBlock, AllocatorError> {

        let buffer_info = self.gen_buffer(&info, BufferBranch::Uniform)?;
        let block = HaUniformBlock::from(&info, buffer_info.item);
        self.append_buffer(buffer_info.buffer, buffer_info.aligment_space, info);

        Ok(block)
    }

    pub(crate) fn append_imgsrc(&mut self, info: ImgsrcBlockInfo) -> Result<HaImgsrcBlock, AllocatorError> {

        let buffer_info = self.gen_buffer(&info, BufferBranch::ImageSrc)?;
        let block = HaImgsrcBlock::from(buffer_info.item);
        self.append_buffer(buffer_info.buffer, buffer_info.aligment_space, info);

        Ok(block)
    }

    fn gen_buffer(&mut self, info: &impl BufferBlockInfo, branch: BufferBranch) -> Result<BufferGenInfo, AllocatorError> {

        if self.ty.check_usage(branch) == false {
            return Err(AllocatorError::UnsupportBufferUsage)
        }

        let buffer = info.build(&self.device, None, self.ty)?;
        self.memory_selector.try(buffer.requirement.memory_type_bits, self.require_mem_flag)?;

        use utility::memory::bind_to_alignment;
        let aligment_space = bind_to_alignment(buffer.requirement.size, buffer.requirement.alignment);

        let item = BufferSubItem {
            handle: buffer.handle,
            buffer_index: self.buffers.len(),
            offset: 0,
            size: info.total_size(),
        };

        let info = BufferGenInfo {
            buffer, aligment_space, item,
        };
        Ok(info)
    }

    fn append_buffer(&mut self, buffer: HaBuffer, space: vk::DeviceSize, config: impl BufferInfosAllocatable + 'static) {

        self.spaces.push(space);
        self.buffers.push(buffer);

        self.allocator.add_allocate(space, Box::new(config));
    }

    pub fn allocate(&mut self) -> Result<HaBufferRepository, AllocatorError> {

        if self.buffers.is_empty() {
            return Err(AllocatorError::Buffer(BufferError::NoBufferAttachError))
        }

        // allocate memory
        let optimal_memory_index = self.memory_selector.optimal_memory()?;
        let mem_type = self.physical.memory.memory_type(optimal_memory_index);

        self.allocator.allocate(
            &self.device, self.spaces.iter().sum(), optimal_memory_index, Some(mem_type)
        )?;

        let mut repository_buffer = vec![];
        {
            let memory_allocated = self.allocator.borrow_memory()?;

            // bind buffers to memory
            let mut offset = 0;
            for (i, buffer) in self.buffers.drain(..).enumerate() {
                memory_allocated.bind_to_buffer(&self.device, &buffer, offset)?;
                offset += self.spaces[i];
                repository_buffer.push(buffer);
            }
        }

        self.allocator.memory_map_if_need(&self.device)?;

        let repository = HaBufferRepository::store(
            &self.device, &self.physical,
            repository_buffer,
            self.allocator.take_memory()?,
            self.allocator.take_info()
        );

        self.reset();
        Ok(repository)
    }

    /// Only call this function for device buffer allocator.
    pub fn set_device_lazily_allocate(&mut self) {
        self.require_mem_flag = self.require_mem_flag | MemoryPropertyFlag::LazilyAllocatedBit.value();
    }

    pub fn storage_type(&self) -> BufferStorageType {
        self.ty
    }

    pub fn reset(&mut self) {

        self.buffers.iter().for_each(|buffer| buffer.cleanup(&self.device));
        self.buffers.clear();

        self.spaces.clear();
        self.memory_selector.reset();
        self.require_mem_flag = self.ty.memory_type().property_flags();
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BufferStorageType {
    Host,
    Cached,
    Device,
    Staging,
}

impl BufferStorageType {

    fn allocator(&self) -> Box<BufMemAlloAbstract> {
        match self {
            | BufferStorageType::Host    => Box::new(HostBufMemAllocator::new()),
            | BufferStorageType::Cached  => Box::new(CachedBufMemAllocator::new()),
            | BufferStorageType::Device  => Box::new(DeviceBufMemAllocator::new()),
            | BufferStorageType::Staging => Box::new(StagingBufMemAllocator::new()),
        }
    }

    fn memory_type(&self) -> HaMemoryType {
        match self {
            | BufferStorageType::Host    => HaMemoryType::HostMemory,
            | BufferStorageType::Cached  => HaMemoryType::CachedMemory,
            | BufferStorageType::Device  => HaMemoryType::DeviceMemory,
            | BufferStorageType::Staging => HaMemoryType::StagingMemory,
        }
    }

    fn check_usage(&self, branch: BufferBranch) -> bool {
        match self {
            | BufferStorageType::Host => {
                [
                    BufferBranch::Vertex,
                    BufferBranch::Index,
                    BufferBranch::Uniform,
                ].iter().find(|&b| *b == branch).is_some()
            },
            | BufferStorageType::Cached  => {
                [
                    BufferBranch::Vertex,
                    BufferBranch::Index,
                ].iter().find(|&b| *b == branch).is_some()
            },
            | BufferStorageType::Device  => {
                [
                    BufferBranch::Vertex,
                    BufferBranch::Index,
                ].iter().find(|&b| *b == branch).is_some()
            },
            | BufferStorageType::Staging => {
                [
                    BufferBranch::Vertex,
                    BufferBranch::Index,
                    BufferBranch::Uniform,
                    BufferBranch::ImageSrc,
                ].iter().find(|&b| *b == branch).is_some()
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum BufferBranch {
    Vertex,
    Index,
    Uniform,
    ImageSrc,
}

struct BufferGenInfo {

    buffer: HaBuffer,
    aligment_space: vk::DeviceSize,
    item: BufferSubItem,
}
