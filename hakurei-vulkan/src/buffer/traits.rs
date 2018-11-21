
use ash::vk;

use types::vkbytes;

pub trait BufferInstance: BufferCopiable {}

pub trait BufferCopiable {

    fn copy_info(&self) -> BufferCopyInfo;
}

pub trait BufferHandleEntity {

    fn handle(&self) -> vk::Buffer;
}

pub struct BufferCopyInfo {

    /// `handle` the handle of buffer whose data is copied from or copy to.
    pub(crate) handle: vk::Buffer,
    /// `offset` the starting offset in bytes from the start of source or destination buffer.
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
