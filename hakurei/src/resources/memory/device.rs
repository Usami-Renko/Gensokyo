
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use resources::buffer::{ HaBuffer, BufferSubItem, BufferUsageFlag };
use resources::buffer::BufferGenerator;
use resources::memory::{ HaMemoryAbstract, HaMemoryType, MemoryDataTransferable, MemoryPropertyFlag, HaHostMemory };
use resources::memory::MemoryRange;
use resources::allocator::DeviceBufferAllocateInfos;
use resources::repository::HaBufferRepository;
use resources::error::MemoryError;

use utility::memory::{ MemoryWritePtr, spaces_to_offsets };
use utility::marker::{ VulkanFlags, VulkanEnum };

use std::ptr;

pub struct HaDeviceMemory {

    handle     : vk::DeviceMemory,
    _size      : vk::DeviceSize,
    mem_type   : Option<vk::MemoryType>,

    staging    : StagingRepository,
}

impl HaMemoryAbstract for HaDeviceMemory {

    fn handle(&self) -> vk::DeviceMemory {
        self.handle
    }

    fn flag(&self) -> vk::MemoryPropertyFlags {
        self.mem_type.as_ref().and_then(|m| Some(m.property_flags))
            .unwrap_or(vk::MemoryPropertyFlags::empty())
    }

    fn memory_type(&self) -> HaMemoryType {
        HaMemoryType::DeviceMemory
    }

    fn default_flag() -> vk::MemoryPropertyFlags {
        [MemoryPropertyFlag::DeviceLocalBit].flags()
    }

    fn allocate(device: &HaLogicalDevice, size: vk::DeviceSize, mem_type_index: usize, mem_type: Option<vk::MemoryType>) -> Result<Self, MemoryError> {

        let allocate_info = vk::MemoryAllocateInfo {
            s_type: vk::StructureType::MemoryAllocateInfo,
            p_next: ptr::null(),
            allocation_size: size,
            // an index identifying a memory type from the memoryTypes array of the vkPhysicalDeviceMemoryProperties structure.
            memory_type_index: mem_type_index as uint32_t,
        };

        let handle = unsafe {
            device.handle.allocate_memory(&allocate_info, None)
                .or(Err(MemoryError::AllocateMemoryError))?
        };

        let memory = HaDeviceMemory {
            handle,
            _size: size,
            mem_type,
            staging: StagingRepository::new(),
        };
        Ok(memory)
    }

    fn enable_map(&mut self, _: &HaLogicalDevice, _: bool) -> Result<(), MemoryError> {
        // leave it empty
        Ok(())
    }
}

impl MemoryDataTransferable for HaDeviceMemory {

    fn prepare_data_transfer(&mut self, device: &HaLogicalDevice) -> Result<(), MemoryError> {

        self.staging.generate_repository(device)?;
        if let Some(ref mut memory) = self.staging.memory {
            memory.prepare_data_transfer(device)?;
            Ok(())
        } else {
            Err(MemoryError::MemoryNotYetAllocateError)
        }
    }

    fn map_memory_ptr(&mut self, item: &BufferSubItem, _: vk::DeviceSize) -> Result<(MemoryWritePtr, MemoryRange), MemoryError> {

        let dst_item = item.clone();
        let mut src_item = item.clone();
        src_item.handle = self.staging.buffers[item.buffer_index].handle;

        // transfer data to staging buffer.
        let offset = self.staging.offsets[item.buffer_index] + item.offset;
        if let Some(ref mut memory) = self.staging.memory {
            let (writer, range) = memory.map_memory_ptr(&src_item, offset)?;

            self.staging.src_items.push(src_item);
            self.staging.dst_items.push(dst_item);

            Ok((writer, range))
        } else {
            Err(MemoryError::MemoryNotYetAllocateError)
        }
    }

    fn terminate_transfer(&mut self, device: &HaLogicalDevice, ranges_to_flush: &Vec<MemoryRange>) -> Result<(), MemoryError> {

        if let Some(ref mut memory) = self.staging.memory {
            memory.terminate_transfer(device, ranges_to_flush)?;
        }

        HaBufferRepository::copy_buffers_to_buffers(device, &self.staging.src_items, &self.staging.dst_items)
            .or(Err(MemoryError::BufferToBufferCopyError))?;

        self.staging.cleanup(device);
        Ok(())

    }
}

impl HaDeviceMemory {

    pub fn set_allocate_infos(&mut self, infos: DeviceBufferAllocateInfos) {
        self.staging.allo_infos = infos;
    }
}



struct StagingRepository {

    allo_infos: DeviceBufferAllocateInfos,
    buffers   : Vec<HaBuffer>,
    memory    : Option<Box<HaMemoryAbstract>>,
    offsets   : Vec<vk::DeviceSize>,

    src_items: Vec<BufferSubItem>,
    dst_items: Vec<BufferSubItem>,
}

impl StagingRepository {

    fn new() -> StagingRepository {
        StagingRepository {
            allo_infos: DeviceBufferAllocateInfos::new(),
            buffers: vec![],
            memory: None,
            offsets: vec![],
            src_items: vec![],
            dst_items: vec![],
        }
    }

    fn generate_repository(&mut self, device: &HaLogicalDevice) -> Result<(), MemoryError> {

        // set buffer usage to transfer source.
        self.allo_infos.configs.iter_mut().for_each(|info| {
            info.usage = BufferUsageFlag::TransferSrcBit.value();
        });

        // generate buffers
        let mut buffers = vec![];
        for info in self.allo_infos.configs.iter() {
            let buffer = info.generate(device, None)
                .or(Err(MemoryError::AllocateMemoryError))?;
            buffers.push(buffer);
        }

        // allocate memory
        let _mem_flag = HaHostMemory::default_flag();
        // FIXME: mem_type_index and mem_type
        let mem_type = vk::MemoryType { property_flags: vk::MEMORY_PROPERTY_HOST_VISIBLE_BIT | vk::MEMORY_PROPERTY_HOST_COHERENT_BIT | vk::MEMORY_PROPERTY_HOST_CACHED_BIT, heap_index: 0 };
        let memory = HaHostMemory::allocate(device, self.allo_infos.spaces.iter().sum(), 1, Some(mem_type))?;

        // bind buffers to memory
        let mut offset = 0;
        let mut repository_buffer = vec![];
        for (i, buffer) in buffers.drain(..).enumerate() {
            memory.bind_to_buffer(device, &buffer, offset)?;
            offset += self.allo_infos.spaces[i];
            repository_buffer.push(buffer);
        }

        self.offsets = spaces_to_offsets(&self.allo_infos.spaces);
        self.buffers = repository_buffer;
        self.memory = Some(Box::new(memory));

        Ok(())
    }

    fn cleanup(&mut self, device: &HaLogicalDevice) {

        for buffer in self.buffers.iter() {
            buffer.cleanup(device);
        }
        if let Some(ref memory) = self.memory {
            memory.cleanup(device);
        }

        self.memory = None;
        self.buffers.clear();
        self.offsets.clear();
        self.src_items.clear();
        self.dst_items.clear();
    }
}
