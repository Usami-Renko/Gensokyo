
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

    fn copy_info(&self) -> BufferCopyInfo;
}

pub trait BufferHandleEntity: Sized {

    fn handle(&self) -> vk::Buffer;
}

pub struct BufferCopyInfo {

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

impl BufferCopyInfo {

    pub fn new(buffer: &impl BufferHandleEntity, offset: vkbytes, size: vkbytes) -> BufferCopyInfo {

        BufferCopyInfo {
            handle: buffer.handle(),
            offset,
            size,
        }
    }
}
