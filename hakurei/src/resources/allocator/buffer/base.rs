
use ash::vk;

use core::device::HaDevice;
use core::physical::{ HaPhyDevice, MemorySelector };

use resources::allocator::BufMemAlloAbstract;
use resources::allocator::{ HostBufMemAllocator, CachedBufMemAllocator, DeviceBufMemAllocator, StagingBufMemAllocator };
use resources::buffer::{ HaBuffer, BufferSubItem, BufferConfigAbstract };
use resources::buffer::{ HostBufferConfig, CachedBufferConfig, DeviceBufferConfig, StagingBufferConfig };
use resources::memory::{ HaMemoryType, MemoryPropertyFlag };
use resources::repository::HaBufferRepository;
use resources::error::{ BufferError, AllocatorError };

use utility::memory::bind_to_alignment;
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
            spaces: vec![],

            ty,
            allocator: ty.allocator(),
            require_mem_flag: ty.memory_type().property_flags(),
            memory_selector: MemorySelector::init(physical),
        }
    }

    pub fn attach_host_buffer(&mut self, config: HostBufferConfig) -> Result<Vec<BufferSubItem>, AllocatorError> {

        if self.ty == BufferStorageType::Host {
            self.attach_buffer(config)
        } else {
            Err(AllocatorError::UnmatchBufferConfig)
        }
    }

    pub fn attach_cached_buffer(&mut self, config: CachedBufferConfig) -> Result<Vec<BufferSubItem>, AllocatorError> {

        if self.ty == BufferStorageType::Cached {
            self.attach_buffer(config)
        } else {
            Err(AllocatorError::UnmatchBufferConfig)
        }
    }

    pub fn attach_device_buffer(&mut self, config: DeviceBufferConfig) -> Result<Vec<BufferSubItem>, AllocatorError> {

        if self.ty == BufferStorageType::Device {
            self.attach_buffer(config)
        } else {
            Err(AllocatorError::UnmatchBufferConfig)
        }
    }

    pub fn attach_staging_buffer(&mut self, config: StagingBufferConfig) -> Result<Vec<BufferSubItem>, AllocatorError> {

        if self.ty == BufferStorageType::Staging {
            self.attach_buffer(config)
        } else {
            Err(AllocatorError::UnmatchBufferConfig)
        }
    }

    fn attach_buffer(&mut self, config: impl BufferConfigAbstract + 'static) -> Result<Vec<BufferSubItem>, AllocatorError> {

        // TODO: Currently HaBuffer only support operation in single queue family.
        let buffer = config.generate(&self.device, None)?;
        self.memory_selector.try(buffer.requirement.memory_type_bits, self.require_mem_flag)?;

        let buffer_index = self.buffers.len();
        let aligment_space = bind_to_alignment(buffer.requirement.size, buffer.requirement.alignment);

        let mut items = vec![];
        let mut offset: vk::DeviceSize = 0;

        for &item_size in config.items_size().iter() {
            let item = BufferSubItem {
                handle: buffer.handle,
                buffer_index,
                offset,
                size: item_size,
            };
            items.push(item);
            offset += item_size;
        }

        self.spaces.push(aligment_space);
        self.buffers.push(buffer);

        self.allocator.add_allocate(aligment_space, Box::new(config));

        Ok(items)
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
}
