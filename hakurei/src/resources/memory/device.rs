
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use resources::buffer::{ HaBuffer, BufferSubItem, BufferUsageFlag };
use resources::buffer::BufferGenerator;
use resources::memory::{ HaMemoryAbstract, MemoryDataTransfer, MemoryPropertyFlag, HaHostMemory };
use resources::allocator::DeviceBufferAllocateInfos;
use resources::repository::HaBufferRepository;
use resources::error::MemoryError;

use utility::memory::MemoryWritePtr;
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
}

impl MemoryDataTransfer for HaDeviceMemory {

    fn prepare_data_transfer(&mut self, device: &HaLogicalDevice) -> Result<(), MemoryError> {
        // create staging host buffer
        self.staging.generate_repository(device)?;
        self.staging.repository.prepare_data_transfer(device)
            .or(Err(MemoryError::AllocateMemoryError))
    }

    fn map_memory_ptr(&mut self, device: &HaLogicalDevice, item: &BufferSubItem, offset: vk::DeviceSize) -> Result<MemoryWritePtr, MemoryError> {

        let dst_item = item.clone();
        let mut src_item = item.clone();
        src_item.handle = self.staging.repository.buffer_at(item.buffer_index).handle;

        // transfer data to staging buffer.
        let staging_memory = self.staging.repository.borrow_mut_memory();
        let writer = staging_memory.map_memory_ptr(device, &src_item, offset)?;

        self.staging.src_items.push(src_item);
        self.staging.dst_items.push(dst_item);

        Ok(writer)
    }

    fn unmap_memory_ptr(&mut self, item: &BufferSubItem, offset: vk::DeviceSize) {

        let staging_memory = self.staging.repository.borrow_mut_memory();
        staging_memory.unmap_memory_ptr(item, offset);
    }

    fn transfer_data(&mut self, device: &HaLogicalDevice) -> Result<(), MemoryError> {

        self.staging.repository.execute_data_transfer(device)
            .or(Err(MemoryError::AllocateMemoryError))?;

        {
            let mut src_items = vec![];
            let mut dst_items = vec![];

            for item in self.staging.src_items.iter() { src_items.push(item.as_ref()) }
            for item in self.staging.dst_items.iter() { dst_items.push(item.as_ref()) }

            HaBufferRepository::copy_buffers_to_buffers(device, &src_items, &dst_items)
                .or(Err(MemoryError::BufferToBufferCopyError))?;
        }

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
    repository: HaBufferRepository,

    src_items: Vec<BufferSubItem>,
    dst_items: Vec<BufferSubItem>,
}

impl StagingRepository {

    fn new() -> StagingRepository {
        StagingRepository {
            allo_infos: DeviceBufferAllocateInfos::new(),
            repository: HaBufferRepository::empty(),
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
        let mem_flag = HaHostMemory::default_flag();
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

        self.repository = HaBufferRepository::store(
            repository_buffer, Box::new(memory), self.allo_infos.spaces.clone());

        Ok(())
    }

    fn cleanup(&mut self, device: &HaLogicalDevice) {

        self.allo_infos.clear();
        self.repository.cleanup(device);

        self.src_items.clear();
        self.dst_items.clear();
    }
}
