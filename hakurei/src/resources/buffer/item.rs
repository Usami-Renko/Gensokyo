
use ash::vk;

#[derive(Debug, Clone)]
pub struct BufferSubItem {
    /// the handle of the vk::Buffer object.
    pub(crate) handle: vk::Buffer,
    /// the index of buffer in HaBufferRepository.
    pub(crate) buffer_index: usize,
    /// the data offset in the buffer.
    ///
    /// This is not the offset in memory.
    pub(crate) offset: vk::DeviceSize,
    /// the size of this BufferSubItem represent.
    pub(crate) size  : vk::DeviceSize,
}

impl BufferSubItem {

    pub fn unset() -> BufferSubItem {
        BufferSubItem {
            handle      : vk::Buffer::null(),
            buffer_index: 0,
            offset      : 0,
            size        : 0,
        }
    }
}

impl AsRef<BufferSubItem> for BufferSubItem {

    fn as_ref(&self) -> &BufferSubItem {
        &self
    }
}

#[derive(Debug, Clone)]
pub struct BufferItem {
    /// the handle of the vk::Buffer object.
    pub(crate) handle: vk::Buffer,
    /// the index of buffer in HaBufferRepository.
    pub(crate) buffer_index: usize,
    /// the size of this BufferItem represent.
    pub(crate) size: vk::DeviceSize,
}

impl BufferItem {

    pub fn unset() -> BufferItem {
        BufferItem {
            handle      : vk::Buffer::null(),
            buffer_index: 0,
            size        : 0,
        }
    }
}
