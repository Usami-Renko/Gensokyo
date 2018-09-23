
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;
use core::physical::{ HaPhysicalDevice, MemorySelector };

use resources::allocator::HaBufferAllocatorAbstract;
use resources::buffer::HaBuffer;
use resources::buffer::{ DeviceBufferConfig, HostBufferConfig, BufferSubItem };
use resources::buffer::BufferGenerator;
use resources::memory::{ HaDeviceMemory, HaMemoryAbstract, MemoryPropertyFlag };
use resources::repository::HaBufferRepository;
use resources::error::{ BufferError, AllocatorError };

use utility::marker::VulkanEnum;
use utility::memory::bind_to_alignment;

pub struct HaDeviceBufferAllocator<'re> {

    physical: &'re HaPhysicalDevice,
    device  : &'re HaLogicalDevice,

    buffers : Vec<HaBuffer>,
    /// The size of each buffer occupy.
    spaces  : Vec<vk::DeviceSize>,

    memory_selector: MemorySelector<'re>,
    require_mem_flag: vk::MemoryPropertyFlags,

    allocate_infos: Option<DeviceBufferAllocateInfos>,
}

impl<'re> HaDeviceBufferAllocator<'re> {

    pub(crate) fn new(physical: &'re HaPhysicalDevice, device: &'re HaLogicalDevice) -> HaDeviceBufferAllocator<'re> {
        HaDeviceBufferAllocator {
            physical,
            device,

            buffers: vec![],
            spaces : vec![],
            require_mem_flag: HaDeviceMemory::default_flag(),
            memory_selector: MemorySelector::init(physical),

            allocate_infos: Some(DeviceBufferAllocateInfos::new()),
        }
    }

    pub fn set_lazily_allocate(&mut self, is_enable: bool) {

        self.require_mem_flag = if is_enable {
            HaDeviceMemory::default_flag() | MemoryPropertyFlag::LazilyAllocatedBit.value()
        } else {
            HaDeviceMemory::default_flag()
        }
    }
}

impl<'re> HaBufferAllocatorAbstract for HaDeviceBufferAllocator<'re> {
    type BufferConfigType = DeviceBufferConfig;

    fn attach_buffer(&mut self, config: Self::BufferConfigType) -> Result<Vec<BufferSubItem>, AllocatorError> {

        // TODO: Currently HaBuffer only support operation in single queue family.

        let buffer = config.generate(self.device, None)?;
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

        if let Some(ref mut infos) = self.allocate_infos {
            infos.configs.push(config.to_host());
            infos.spaces.push(aligment_space);
        }

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
        let mut memory = HaDeviceMemory::allocate(self.device, self.spaces.iter().sum(), optimal_memory_index, Some(mem_type))?;

        memory.set_allocate_infos(self.allocate_infos.take().unwrap());

        // bind buffers to memory
        let mut offset = 0;
        let mut repository_buffer = vec![];
        for (i, buffer) in self.buffers.drain(..).enumerate() {
            memory.bind_to_buffer(self.device, &buffer, offset)?;
            offset += self.spaces[i];
            repository_buffer.push(buffer);
        }

        let repository = HaBufferRepository::store(repository_buffer, Box::new(memory), self.spaces.clone());

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
        self.allocate_infos = Some(DeviceBufferAllocateInfos::new());
    }
}


pub struct DeviceBufferAllocateInfos {

    pub configs: Vec<HostBufferConfig>,
    pub spaces : Vec<vk::DeviceSize>,
}

impl DeviceBufferAllocateInfos {

    pub fn new() -> DeviceBufferAllocateInfos {
        DeviceBufferAllocateInfos { configs: vec![], spaces: vec![], }
    }
}
