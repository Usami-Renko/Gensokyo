
use ash::vk;

use resources::buffer::BufferBlockEntity;

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
        BufferItem::default()
    }
}

impl Default for BufferItem {

    fn default() -> BufferItem {
        BufferItem {
            handle      : vk::Buffer::null(),
            buffer_index: 0,
            size        : 0,
        }
    }
}

impl BufferBlockEntity for BufferItem {

    fn get_buffer_item(&self) -> &BufferItem {
        &self
    }

    fn offset(&self, _sub_index: usize) -> vk::DeviceSize {
        panic!("This function should't be called.")
    }
}

impl AsRef<BufferItem> for BufferItem {

    fn as_ref(&self) -> &BufferItem {
        &self
    }
}


//#[derive(Debug, Clone)]
//pub(crate) struct BufferSubItem {
//    /// the handle of the vk::Buffer object.
//    pub(crate) handle: vk::Buffer,
//    /// the size of this BufferSubItem represent.
//    pub(crate) space: vk::DeviceSize,
//    /// the data offset in the buffer.
//    ///
//    /// This is not the offset in memory.
//    pub(crate) start_offset: vk::DeviceSize,
//}
//
//impl BufferSubItem {
//
//    pub fn from(item: &BufferItem, start_offset: vk::DeviceSize, space: vk::DeviceSize) -> BufferSubItem {
//        BufferSubItem {
//            handle: item.handle,
//            space, start_offset,
//        }
//    }
//}
