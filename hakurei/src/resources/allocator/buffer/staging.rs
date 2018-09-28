
use ash::vk;

use core::device::HaDevice;
use core::physical::{ HaPhyDevice, MemorySelector };

use resources::allocator::{ HaBufferAllocatorAbstract, BufferAllocateInfos };
use resources::buffer::HaBuffer;
use resources::buffer::{ StagingBufferConfig, BufferSubItem };
use resources::buffer::BufferGenerator;
use resources::memory::{ HaStagingMemory, HaMemoryAbstract };
use resources::repository::HaBufferRepository;
use resources::error::{ BufferError, AllocatorError };

use utility::memory::bind_to_alignment;

pub struct HaStagingBufferAllocator {

    physical: HaPhyDevice,
    device  : HaDevice,

    buffers : Vec<HaBuffer>,
    /// The size of each buffer occupy.
    spaces  : Vec<vk::DeviceSize>,

    require_mem_flag: vk::MemoryPropertyFlags,
    memory_selector : MemorySelector,
}

impl HaStagingBufferAllocator {

    pub(crate) fn new(physical: &HaPhyDevice, device: &HaDevice) -> HaStagingBufferAllocator {

        HaStagingBufferAllocator {
            physical: physical.clone(),
            device  : device.clone(),

            buffers: vec![],
            spaces : vec![],
            require_mem_flag: HaStagingMemory::default_flag(),
            memory_selector : MemorySelector::init(physical),
        }
    }
}

impl HaBufferAllocatorAbstract for HaStagingBufferAllocator {
    type BufferConfigType = StagingBufferConfig;

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

    fn allocate(&mut self) -> Result<HaBufferRepository, AllocatorError> {

        if self.buffers.is_empty() {
            return Err(AllocatorError::Buffer(BufferError::NoBufferAttachError))
        }

        // allocate memory
        let optimal_memory_index = self.memory_selector.optimal_memory()?;
        let mem_type = self.physical.memory.memory_type(optimal_memory_index);
        let memory = HaStagingMemory::allocate(
            &self.device, self.spaces.iter().sum(), optimal_memory_index, Some(mem_type)
        )?;

        // bind buffers to memory
        let mut offset = 0;
        let mut repository_buffer = vec![];
        for (i, buffer) in self.buffers.drain(..).enumerate() {
            memory.bind_to_buffer(&self.device, &buffer, offset)?;
            offset += self.spaces[i];
            repository_buffer.push(buffer);
        }

        let allocate_infos = BufferAllocateInfos::from_spaces(&self.spaces);

        let repository = HaBufferRepository::store(&self.device, &self.physical, repository_buffer, Box::new(memory), allocate_infos);

        self.reset();
        Ok(repository)
    }

    fn reset(&mut self) {

        self.buffers.iter().for_each(|buffer| buffer.cleanup(&self.device));
        self.buffers.clear();

        self.spaces.clear();
        self.memory_selector.reset();
    }
}
