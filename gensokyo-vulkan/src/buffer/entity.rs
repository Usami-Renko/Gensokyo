
use ash::vk;

use crate::buffer::target::GsBuffer;
use crate::buffer::traits::{ BufferCopiable, BufferFullCopyInfo };

use crate::types::vkbytes;

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

impl BufferCopiable for BufferBlock {

    fn copy_whole(&self) -> BufferFullCopyInfo {

        BufferFullCopyInfo {
            handle: self.handle,
            size  : self.size,
        }
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
