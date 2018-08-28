
use ash::vk;
use utility::marker::VulkanFlags;

use core::device::HaLogicalDevice;
use core::physical::HaPhysicalDevice;

use resources::buffer::{ HaBuffer, BufferConfig };
use resources::memory::{ HaMemoryAbstract, HaHostMemory };
use resources::error::MemoryError;
use utility::allocator::error::AllocatorError;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MemoryLocation {
    HostMemory,
    DeviceMemory,
}

// TODO: Currently HaBufferAllocator only support operation in single queue family.

pub struct HaBufferAllocator<'re> {

    device  : &'re HaLogicalDevice,
    physical: &'re HaPhysicalDevice,

    buffers : Vec<HaBuffer>,
    memory  : Option<HaHostMemory>,
    /// The size of each buffer occupy.
    spaces  : Vec<vk::DeviceSize>,
    /// The offset of each buffer in meomory.
    offsets : Vec<vk::DeviceSize>,
    /// The index of memory type that available to use.
    candidate_memories: Vec<usize>,
}

impl<'re> HaBufferAllocator<'re> {

    pub fn new(device: &'re HaLogicalDevice, physical: &'re HaPhysicalDevice)
        -> HaBufferAllocator<'re> {

        HaBufferAllocator {
            device,
            physical,

            buffers: vec![],
            memory : None,
            spaces : vec![],
            offsets: vec![],
            candidate_memories: vec![],
        }
    }

    pub fn attach_buffer<D: Copy>(&mut self, config: BufferConfig<D>) -> Result<(), AllocatorError> {

        if self.memory.is_some() {
            return Err(AllocatorError::MemoryAlreadyAllocated)
        }

        let buffer = HaBuffer::generate(self.device, config.data, config.usage, config.buffer_flags, None)
            .map_err(|e| AllocatorError::Buffer(e))?;
        let required_size = buffer.require_memory_size();

        self.candidate_memories = self.physical.memory.find_memory_type(
            buffer.require_memory_type_bits(),
            config.memory_flags.flags(),
            if self.candidate_memories.is_empty() { None } else { Some(&self.candidate_memories) }
        ).map_err(|e| AllocatorError::Memory(e))?;

        self.buffers.push(buffer);
        self.spaces.push(required_size);

        Ok(())
    }

    /// Allocate memory for buffers, and bind those buffer to the memory.
    ///
    /// Must not call attach_buffer method after calling this method.
    pub fn allocate(&mut self) -> Result<(), AllocatorError> {

        if self.buffers.is_empty() {
            return Err(AllocatorError::NoAvailableBufferAttach)
        }

        let optimal_memory_index = self.physical.memory.optimal_memory(&self.candidate_memories)
            .map_err(|e| AllocatorError::Memory(e))?;
        let allocate_size = self.spaces.iter()
            .fold(0 as vk::DeviceSize, |total, buffer_size| total + buffer_size);

        // allocate memory
        // TODO: Support HaDeviceMemory
        let memory = HaHostMemory::allocate(
            self.physical, self.device,
            allocate_size,
            optimal_memory_index
        ).map_err(|e| AllocatorError::Memory(e))?;

        // bind buffers to memory
        let mut offset = 0;
        for (i, buffer) in self.buffers.iter().enumerate() {
            self.offsets.push(offset);

            memory.bind(self.device, buffer.handle, offset)
                .map_err(|e| AllocatorError::Memory(e))?;
            offset += self.spaces[i];
        }

        self.memory = Some(memory);

        Ok(())
    }

    pub fn tranfer_data<D: Copy>(&self, data: &Vec<D>) -> Result<(), AllocatorError> {

        let memory = self.memory.as_ref()
            .ok_or(AllocatorError::Memory(MemoryError::MemoryNotYetAllocateError))?;

        // FIXME: use VK_WHOLE_SIZE map the memory once, and copy the data multiple time.
        for (i, buffer) in self.buffers.iter().enumerate() {
            let data_size = self.spaces[i];

            let data_ptr = memory.map(self.device, self.offsets[i], data_size)
                .map_err(|e| AllocatorError::Memory(e))?;
            buffer.copy_data(data_ptr, data_size, data);
            memory.unmap(self.device);
        }

        Ok(())
    }

    pub fn binding_infos(&self) -> BufferBindingInfos {

        let handles: Vec<vk::Buffer> = self.buffers.iter().map(|b| b.handle).collect();
        let offsets = self.offsets.clone();

        BufferBindingInfos {
            handles,
            offsets,
        }
    }

    pub fn cleanup(&self) {
        for buffer in self.buffers.iter() {
            buffer.cleanup(self.device);
        }
        if let Some(ref memory) = self.memory {
            memory.cleanup(self.device);
        }
    }
}

pub struct BufferBindingInfos {

    pub(crate) handles: Vec<vk::Buffer>,
    pub(crate) offsets: Vec<vk::DeviceSize>,
}
