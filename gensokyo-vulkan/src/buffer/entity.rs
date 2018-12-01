
use ash::vk;

use buffer::target::GsBuffer;
use buffer::traits::{ BufferHandleEntity, BufferCopiable, BufferCopyInfo };

use types::vkbytes;

#[derive(Debug, Clone, Default)]
pub struct BufferBlock {

    /// the handle of the vk::Buffer object.
    pub(crate) handle: vk::Buffer,
    /// the size of this BufferBlock represent.
    pub size: vkbytes,
    /// the starting offset of this BufferBlock in memory.
    pub memory_offset: vkbytes,
}

#[derive(Debug, Clone, Default)]
pub struct BufferSlice {

    /// the handle of the vk::Buffer object.
    pub(crate) handle: vk::Buffer,
    /// the size of this BufferSlice represent.
    pub size: vkbytes,
    /// the starting offset of this BufferSlice in memory.
    pub memory_offset: vkbytes,
}

impl BufferBlock {

    pub fn new(buffer: &GsBuffer, size: vkbytes, memory_offset: vkbytes) -> BufferBlock {

        BufferBlock {
            handle: buffer.handle,
            size,
            memory_offset,
        }
    }
}

impl BufferHandleEntity for BufferBlock {

    fn handle(&self) -> vk::Buffer {
        self.handle
    }
}

impl BufferCopiable for BufferBlock {

    fn copy_info(&self) -> BufferCopyInfo {

        BufferCopyInfo::new(self, self.memory_offset, self.size)
    }
}

impl BufferSlice {

    pub fn new(buffer: &GsBuffer, size: vkbytes, memory_offset: vkbytes) -> BufferSlice {

        BufferSlice {
            handle: buffer.handle,
            size,
            memory_offset,
        }
    }
}
