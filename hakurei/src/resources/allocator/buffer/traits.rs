
use resources::repository::HaBufferRepository;
use resources::buffer::BufferSubItem;
use resources::buffer::{ CachedBufferConfig, DeviceBufferConfig, StagingBufferConfig };
use resources::error::AllocatorError;

pub trait HaBufferAllocatorAbstract {
    type BufferConfigType;

    fn attach_buffer(&mut self, config: Self::BufferConfigType) -> Result<Vec<BufferSubItem>, AllocatorError>;

    /// Allocate memory for buffers, and bind those buffer to the memory. All resource store in BufferRepository Object.
    ///
    /// Must not call attach_buffer method after calling this method.
    fn allocate(&mut self) -> Result<HaBufferRepository, AllocatorError>;
    fn reset(&mut self);
}

pub trait BufferConfigsAllocatable {

    fn to_staging_config(&self) -> StagingBufferConfig;
}

impl BufferConfigsAllocatable for CachedBufferConfig {

    fn to_staging_config(&self) -> StagingBufferConfig {

        StagingBufferConfig {
            usage: self.usage.clone(),
            flags: self.flags.clone(),

            total_size: self.total_size,
            items_size: self.items_size.clone(),
        }
    }
}
impl BufferConfigsAllocatable for DeviceBufferConfig {

    fn to_staging_config(&self) -> StagingBufferConfig {
        StagingBufferConfig {
            usage: self.usage.clone(),
            flags: self.flags.clone(),

            total_size: self.total_size,
            items_size: self.items_size.clone(),
        }
    }
}
