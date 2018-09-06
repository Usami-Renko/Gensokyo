
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;
use core::physical::{ HaPhysicalDevice, MemorySelector };

use resources::buffer::HaBuffer;
use resources::buffer::{BufferConfig, BufferSubItem};
use resources::memory::device::HaDeviceMemory;
use resources::memory::traits::HaMemoryAbstract;
use resources::repository::HaBufferRepository;
use resources::error::{ BufferError, AllocatorError };

use utility::memory::bind_to_alignment;

// TODO: Currently HaBufferAllocator only support operation in single queue family.

pub struct HaBufferAllocator<'re> {

    physical: &'re HaPhysicalDevice,
    device  : &'re HaLogicalDevice,

    buffers : Vec<HaBuffer>,
    /// The size of each buffer occupy.
    spaces  : Vec<vk::DeviceSize>,

    memory_selector: MemorySelector<'re>,
    /// record the final memory property flags, use to make some judgements in latter operation
    mem_flag: vk::MemoryPropertyFlags,
}

impl<'re> HaBufferAllocator<'re> {

    pub(super) fn new(physical: &'re HaPhysicalDevice, device: &'re HaLogicalDevice)
        -> HaBufferAllocator<'re> {

        HaBufferAllocator {
            physical,
            device,

            buffers: vec![],
            spaces : vec![],
            memory_selector: MemorySelector::init(physical),
            mem_flag: vk::MemoryPropertyFlags::empty(),
        }
    }

    pub fn attach_buffer(&mut self, config: BufferConfig) -> Result<Vec<BufferSubItem>, AllocatorError> {

        let buffer = HaBuffer::generate(self.device, &config, None)?;
        let required_memory_flag = config.memory_flags;

        self.memory_selector.try(buffer.requirement.memory_type_bits, required_memory_flag)?;

        let buffer_index = self.buffers.len();
        self.mem_flag = self.mem_flag | required_memory_flag;
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
    pub fn allocate(&mut self) -> Result<HaBufferRepository, AllocatorError> {

        if self.buffers.is_empty() {
            return Err(AllocatorError::Buffer(BufferError::NoBufferAttachError))
        }

        let optimal_memory_index = self.memory_selector.optimal_memory()?;
        let allocate_size = self.spaces.iter().sum();

        // allocate memory
        let memory = HaDeviceMemory::allocate(
            self.physical, self.device,
            allocate_size,
            optimal_memory_index,
            self.mem_flag
        )?;

        // bind buffers to memory
        let mut offset = 0;
        for (i, buffer) in self.buffers.iter().enumerate() {

            memory.bind_to_buffer(self.device, buffer, offset)?;
            offset += self.spaces[i];
        }

        let mut repository_buffer = vec![];
        repository_buffer.append(&mut self.buffers);

        let repository = HaBufferRepository::store(repository_buffer, memory, self.spaces.clone());

        self.reset();
        Ok(repository)
    }

    pub fn reset(&mut self) {

        for buffer in self.buffers.iter() {
            unsafe {
                self.device.handle.destroy_buffer(buffer.handle, None);
            }
        }
        self.buffers.clear();

        self.spaces.clear();
        self.memory_selector.reset();
        self.mem_flag = vk::MemoryPropertyFlags::empty();
    }
}
