
use ash::vk;
use utility::marker::VulkanFlags;

use core::device::HaLogicalDevice;
use core::physical::HaPhysicalDevice;

use resources::buffer::{ HaBuffer, BufferConfig };
use resources::memory::{ HaMemoryAbstract, HaDeviceMemory };
use resources::repository::HaBufferRepository;
use resources::error::MemoryError;
use resources::error::AllocatorError;

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

    pub fn new(physical: &'re HaPhysicalDevice, device: &'re HaLogicalDevice)
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

    pub fn attach_buffer<D: Copy>(&mut self, config: BufferConfig<D>) -> Result<(), AllocatorError> {

        let buffer = HaBuffer::generate(self.device, config.data, config.usage, config.buffer_flags, None)
            .map_err(|e| AllocatorError::Buffer(e))?;
        let required_size = buffer.require_memory_size();
        let required_memory_flag = config.memory_flags.flags();

        self.candidate_memories = self.physical.memory.find_memory_type(
            buffer.require_memory_type_bits(),
            required_memory_flag,
            if self.candidate_memories.is_empty() { None } else { Some(&self.candidate_memories) }
        ).map_err(|e| AllocatorError::Memory(e))?;

        if self.mem_flag.subset(required_memory_flag) {

            self.mem_flag = self.mem_flag & required_memory_flag;
            self.buffers.push(buffer);
            self.spaces.push(required_size);

            Ok(())
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
        Ok(repository)
    }
}
