
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;
use core::physical::{ HaPhyDevice, MemorySelector };

use resources::allocator::HaBufferAllocatorAbstract;
use resources::buffer::HaBuffer;
use resources::buffer::{ HostBufferConfig, BufferSubItem };
use resources::buffer::BufferGenerator;
use resources::memory::{ HaHostMemory, HaMemoryAbstract, MemoryPropertyFlag };
use resources::repository::HaBufferRepository;
use resources::error::{ BufferError, AllocatorError };

use utility::marker::VulkanEnum;
use utility::memory::bind_to_alignment;

pub struct HaHostBufferAllocator {

    physical: HaPhyDevice,
    device  : HaDevice,

    buffers : Vec<HaBuffer>,
    /// The size of each buffer occupy.
    spaces  : Vec<vk::DeviceSize>,

    require_mem_flag: vk::MemoryPropertyFlags,

    memory_selector: MemorySelector,
}

impl HaHostBufferAllocator {

    pub(crate) fn new(physical: &HaPhyDevice, device: &HaDevice) -> HaHostBufferAllocator {

        HaHostBufferAllocator {
            physical: physical.clone(),
            device  : device.clone(),

            buffers: vec![],
            spaces : vec![],
            require_mem_flag: HaHostMemory::default_flag(),
            memory_selector: MemorySelector::init(physical),
        }
    }

    pub fn set_cache(&mut self, is_enable: bool) {

        self.require_mem_flag = if is_enable {
            HaHostMemory::default_flag() | MemoryPropertyFlag::HostCachedBit.value()
        } else {
            HaHostMemory::default_flag()
        }
    }
}

impl HaBufferAllocatorAbstract for HaHostBufferAllocator {
    type BufferConfigType = HostBufferConfig;

    fn attach_buffer(&mut self, config: Self::BufferConfigType) -> Result<Vec<BufferSubItem>, AllocatorError> {

        // TODO: Currently HaBuffer only support operation in single queue family.
        let buffer = config.generate(&self.device, None)?;
        self.memory_selector.try(buffer.requirement.memory_type_bits, self.require_mem_flag)?;

        let buffer_index = self.buffers.len();
        let aligment_space = bind_to_alignment(buffer.requirement.size, buffer.requirement.alignment);

        let mut items = vec![];
        let mut offset: vk::DeviceSize = 0;

        for &item_size in config.items_size.iter() {
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

        Ok(items)
    }

    /// Allocate memory for buffers, and bind those buffer to the memory. All resource store in BufferRepository Object.
    ///
    /// Must not call attach_buffer method after calling this method.
    fn allocate(&mut self) -> Result<HaBufferRepository, AllocatorError> {

        if self.buffers.is_empty() {
            return Err(AllocatorError::Buffer(BufferError::NoBufferAttachError))
        }

        // allocate memory
        let optimal_memory_index = self.memory_selector.optimal_memory()?;
        let mem_type = self.physical.memory.memory_type(optimal_memory_index);
        let memory = HaHostMemory::allocate(&self.device, self.spaces.iter().sum(), optimal_memory_index, Some(mem_type))?;

        // bind buffers to memory
        let mut offset = 0;
        let mut repository_buffer = vec![];
        for (i, buffer) in self.buffers.drain(..).enumerate() {
            memory.bind_to_buffer(&self.device, &buffer, offset)?;
            offset += self.spaces[i];
            repository_buffer.push(buffer);
        }

        let repository = HaBufferRepository::store(&self.device, repository_buffer, Box::new(memory), self.spaces.clone());

        self.reset();
        Ok(repository)
    }

    fn reset(&mut self) {

        for buffer in self.buffers.iter() {
            unsafe {
                self.device.handle.destroy_buffer(buffer.handle, None);
            }
        }
        self.buffers.clear();

        self.spaces.clear();
        self.memory_selector.reset();
    }
}
