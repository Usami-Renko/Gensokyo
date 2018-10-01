
use ash::vk;

use core::device::HaDevice;

use resources::allocator::BufferAllocateInfos;
use resources::buffer::BufferConfigAbstract;
use resources::buffer::{ HostBufferConfig, CachedBufferConfig, DeviceBufferConfig, StagingBufferConfig };
use resources::memory::HaMemoryAbstract;
use resources::error::MemoryError;


/// Represent an trait object as a Buffer Memory Allocator.
pub(crate) trait BufMemAlloAbstract {

    fn add_allocate(&mut self, space: vk::DeviceSize, config: Box<BufferConfigAbstract>);
    fn allocate(&mut self, device: &HaDevice, size: vk::DeviceSize, mem_type_index: usize, mem_type: Option<vk::MemoryType>) -> Result<(), MemoryError>;
    fn borrow_memory(&self) -> Result<&HaMemoryAbstract, MemoryError>;
    fn memory_map_if_need(&mut self, device: &HaDevice) -> Result<(), MemoryError>;
    fn take_memory(&mut self) -> Result<Box<HaMemoryAbstract>, MemoryError>;
    fn take_info(&mut self) -> BufferAllocateInfos;
}



pub trait BufferConfigsAllocatable {

    fn to_staging_config(&self) -> Option<StagingBufferConfig> { None }
}

impl BufferConfigsAllocatable for CachedBufferConfig {

    fn to_staging_config(&self) -> Option<StagingBufferConfig> {

        let config = StagingBufferConfig {
            usage: self.usage.clone(),
            flags: self.flags.clone(),

            total_size: self.total_size,
            items_size: self.items_size.clone(),
        };

        Some(config)
    }
}

impl BufferConfigsAllocatable for DeviceBufferConfig {

    fn to_staging_config(&self) -> Option<StagingBufferConfig> {

        let config = StagingBufferConfig {
            usage: self.usage.clone(),
            flags: self.flags.clone(),

            total_size: self.total_size,
            items_size: self.items_size.clone(),
        };

        Some(config)
    }
}

impl BufferConfigsAllocatable for HostBufferConfig    {}
impl BufferConfigsAllocatable for StagingBufferConfig {}
