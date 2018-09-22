
use resources::repository::HaBufferRepository;
use resources::buffer::BufferSubItem;
use resources::error::AllocatorError;

pub trait HaBufferAllocatorAbstract {
    type BufferConfigType;

    fn attach_buffer(&mut self, config: Self::BufferConfigType) -> Result<Vec<BufferSubItem>, AllocatorError>;

    fn allocate(&mut self) -> Result<HaBufferRepository, AllocatorError>;
    fn reset(&mut self);
}
