
use crate::core::GsDevice;
use crate::buffer::BufferBlock;
use crate::memory::utils::MemoryWritePtr;
use crate::error::VkResult;

pub trait MemoryDataDelegate {

    fn prepare(&mut self, device: &GsDevice) -> VkResult<()>;

    fn acquire_write_ptr(&mut self, block: &BufferBlock, repository_index: usize) -> VkResult<MemoryWritePtr>;

    fn finish(&mut self, device: &GsDevice) -> VkResult<()>;
}
