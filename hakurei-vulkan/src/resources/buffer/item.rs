
use ash::vk;

use resources::buffer::target::HaBuffer;
use resources::buffer::traits::BufferBlockEntity;
use resources::buffer::traits::{ BufferCopiable, BufferCopyInfo, BufferHandleEntity };

use utils::types::vkMemorySize;

#[derive(Debug, Clone)]
pub struct BufferItem {

    /// the handle of the vk::Buffer object.
    pub(crate) handle: vk::Buffer,
    /// the size of this BufferItem represent.
    pub size: vkMemorySize,
    /// the offset of this BufferItem in memory.
    pub memory_offset: vkMemorySize,
}

impl BufferItem {

    pub fn unset() -> BufferItem {
        BufferItem {
            handle: vk::Buffer::null(),
            size: 0,
            memory_offset: 0,
        }
    }

    pub fn new(buffer: &HaBuffer, size: vkMemorySize, memory_offset: vkMemorySize) -> BufferItem {

        BufferItem {
            handle: buffer.handle,
            size, memory_offset,
        }
    }
}

impl Default for BufferItem {

    fn default() -> BufferItem {
        BufferItem::unset()
    }
}

impl BufferBlockEntity for BufferItem {

    fn item(&self) -> &BufferItem {
        &self
    }

    fn offset(&self, _sub_index: usize) -> vkMemorySize {
        panic!("This function should't be called.")
    }
}

impl BufferHandleEntity for BufferItem {

    fn handle(&self) -> vk::Buffer {
        self.handle
    }
}

impl BufferCopiable for BufferItem {

    fn copy_info(&self) -> BufferCopyInfo {
        BufferCopyInfo::new(self, 0, self.size)
    }
}

impl AsRef<BufferItem> for BufferItem {

    fn as_ref(&self) -> &BufferItem {
        &self
    }
}
