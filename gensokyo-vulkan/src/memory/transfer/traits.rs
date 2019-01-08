
use crate::core::device::GsDevice;
use crate::buffer::BufferBlock;
use crate::memory::utils::MemoryWritePtr;
use crate::memory::error::{ MemoryError, AllocatorError };

pub trait MemoryDataDelegate {

    fn prepare(&mut self, device: &GsDevice) -> Result<(), MemoryError>;

    fn acquire_write_ptr(&mut self, block: &BufferBlock, repository_index: usize) -> Result<MemoryWritePtr, MemoryError>;

    fn finish(&mut self, device: &GsDevice) -> Result<(), AllocatorError>;
}
