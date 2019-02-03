
use ash::vk;

use crate::buffer::entity::BufferBlock;
use crate::memory::transfer::MemoryDataDelegate;
use crate::memory::MemoryWritePtr;

use crate::error::VkResult;
use crate::types::vkbytes;

pub trait BufferInstance: BufferCopiable {
    type InfoType;

    fn build(block: BufferBlock, info: Self::InfoType, repository_index: usize) -> Self;

    fn acquire_write_ptr(&self, agency: &mut Box<dyn MemoryDataDelegate>) -> VkResult<MemoryWritePtr>;
}

pub trait BufferCopiable: Sized {

    fn copy_whole(&self) -> BufferFullCopyInfo;
}

pub struct BufferFullCopyInfo {

    /// `handle` is the handle of buffer whose data is copied from or copy to.
    pub(crate) handle: vk::Buffer,
    /// If this is the buffer for data source, `size` is the number of bytes to copy.
    ///
    /// If this is the buffer for data destination, `size` will be ignored.
    pub(crate) size: vkbytes,
}

pub struct BufferRangeCopyInfo {

    /// `handle` is the handle of buffer whose data is copied from or copy to.
    pub(crate) handle: vk::Buffer,
    /// `offset` is the starting offset in bytes from the start of source or destination buffer.
    ///
    /// `offset` is not the starting offset of memory.
    pub(crate) offset: vkbytes,
    /// If this is the buffer for data source, `size` is the number of bytes to copy.
    ///
    /// If this is the buffer for data destination, `size` will be ignored.
    pub(crate) size: vkbytes,
}
