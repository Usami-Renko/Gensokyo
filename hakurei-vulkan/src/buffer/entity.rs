
use ash::vk;

use buffer::target::HaBuffer;
use buffer::traits::BufferHandleEntity;

use types::vkbytes;

pub enum BufferEntity {

    Block(BufferBlock),
    Slices(Vec<BufferSlice>),
}

impl BufferHandleEntity for BufferEntity {

    fn handle(&self) -> vk::Buffer {
        match self {
            | BufferEntity::Block(block)   => block.handle,
            | BufferEntity::Slices(slices) => slices[0].handle,
        }
    }
}

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

    pub fn new(buffer: &HaBuffer, size: vkbytes, memory_offset: vkbytes) -> BufferBlock {

        BufferBlock {
            handle: buffer.handle,
            size,
            memory_offset,
        }
    }
}

impl BufferSlice {

    pub fn new(buffer: &HaBuffer, size: vkbytes, memory_offset: vkbytes) -> BufferSlice {

        BufferSlice {
            handle: buffer.handle,
            size,
            memory_offset,
        }
    }
}
