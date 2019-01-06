
use ash::vk;

use crate::buffer::instance::BufferInstanceType;
use crate::buffer::entity::BufferBlock;
use crate::buffer::target::BufferDescInfo;
use crate::buffer::allocator::BufferBlockIndex;
use crate::types::vkbytes;

pub trait BufferInstance: BufferCopiable {

    fn typ(&self) -> BufferInstanceType;

    fn as_block_ref(&self) -> &BufferBlock;
    fn repository_index(&self) -> usize;
}

pub trait BufferBlockInfo {
    const INSTANCE_TYPE: BufferInstanceType;

    fn typ(&self) -> BufferInstanceType {
        Self::INSTANCE_TYPE
    }

    fn as_desc_ref(&self) -> &BufferDescInfo;

    fn into_desc(self) -> BufferDescInfo;

    fn to_block_index(&self, index: usize) -> BufferBlockIndex {

        BufferBlockIndex {
            value: index,
            attachment: None,
        }
    }
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
