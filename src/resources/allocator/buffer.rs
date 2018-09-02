
use ash::vk;

use core::device::HaLogicalDevice;
use core::physical::HaPhysicalDevice;

use resources::buffer::HaBuffer;
use resources::buffer::{ BufferConfig, BufferItem };
use resources::memory::device::HaDeviceMemory;
use resources::memory::traits::HaMemoryAbstract;
use resources::repository::HaBufferRepository;
use resources::error::MemoryError;
use resources::error::AllocatorError;

use utility::aligment::bind_to_alignment;

// TODO: Currently HaBufferAllocator only support operation in single queue family.

pub struct HaBufferAllocator<'re> {

    device  : &'re HaLogicalDevice,
    physical: &'re HaPhysicalDevice,

    buffers : Vec<HaBuffer>,
    /// The size of each buffer occupy.
    spaces  : Vec<vk::DeviceSize>,
    /// The index of memory type that available to use.
    candidate_memories: Vec<usize>,

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
            candidate_memories: vec![],
            mem_flag: vk::MemoryPropertyFlags::all(),
        }
    }

    pub fn attach_buffer(&mut self, config: BufferConfig) -> Result<Vec<BufferItem>, AllocatorError> {

        let buffer = HaBuffer::generate(self.device, &config, None)
            .map_err(|e| AllocatorError::Buffer(e))?;
        let required_memory_flag = config.memory_flags;

        self.candidate_memories = self.physical.memory.find_memory_type(
            buffer.requirement.memory_type_bits,
            required_memory_flag,
            if self.candidate_memories.is_empty() { None } else { Some(&self.candidate_memories) }
        ).map_err(|e| AllocatorError::Memory(e))?;

        if self.mem_flag.subset(required_memory_flag) {
            let buffer_index = self.buffers.len();

            self.mem_flag = self.mem_flag & required_memory_flag;
            let aligment_space = bind_to_alignment(buffer.requirement.size, buffer.requirement.alignment);
            self.spaces.push(aligment_space);
            self.buffers.push(buffer);

            let mut items = vec![];
            let mut offset: vk::DeviceSize = 0;
            for &item_size in config.items_size.iter() {
                let item = BufferItem {
                    buffer_index,
                    offset,
                    size: item_size,
                };
                items.push(item);
                offset += item_size;
            }

            Ok(items)
        } else {
            Err(AllocatorError::Memory(MemoryError::NoSuitableMemoryError))
        }
    }

    /// Allocate memory for buffers, and bind those buffer to the memory. All resource store in BufferRepository Object.
    ///
    /// Must not call attach_buffer method after calling this method.
    pub fn allocate(&mut self) -> Result<HaBufferRepository, AllocatorError> {

        if self.buffers.is_empty() {
            return Err(AllocatorError::NoAvailableBufferAttach)
        }

        let optimal_memory_index = self.physical.memory.optimal_memory(&self.candidate_memories)
            .map_err(|e| AllocatorError::Memory(e))?;
        let allocate_size = self.spaces.iter().sum();

        // allocate memory
        let memory = HaDeviceMemory::allocate(
            self.physical, self.device,
            allocate_size,
            optimal_memory_index,
            self.mem_flag
        ).map_err(|e| AllocatorError::Memory(e))?;

        // bind buffers to memory
        let mut offset = 0;
        for (i, buffer) in self.buffers.iter().enumerate() {

            memory.bind(self.device, buffer.handle, offset)
                .map_err(|e| AllocatorError::Memory(e))?;
            offset += self.spaces[i];
        }

        let mut repository_buffer = vec![];
        repository_buffer.append(&mut self.buffers);

        let repository = HaBufferRepository::store(repository_buffer, memory, self.spaces.clone());

        self.reset();
        Ok(repository)
    }

    pub fn reset(&mut self) {

        self.buffers.clear();
        self.spaces.clear();
        self.candidate_memories.clear();
        self.mem_flag = vk::MemoryPropertyFlags::all();
    }
}
