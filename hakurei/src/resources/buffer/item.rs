
use ash::vk;

use resources::buffer::{ BufferUsageFlag, BufferCreateFlag };
use resources::memory::MemoryPropertyFlag;

use utility::marker::VulkanFlags;

pub struct BufferConfig {

    pub(crate) usages       : vk::BufferUsageFlags,
    // TODO: Turn the flags into bool options.
    pub(crate) buffer_flags : vk::BufferCreateFlags,
    pub(crate) memory_flags : vk::MemoryPropertyFlags,

    pub(crate) total_size   : vk::DeviceSize,
    pub(crate) items_size   : Vec<vk::DeviceSize>,
}

impl BufferConfig {

    pub fn init(usages: &[BufferUsageFlag], memory_flags: &[MemoryPropertyFlag], buffer_flags: &[BufferCreateFlag]) -> BufferConfig {
        BufferConfig {
            usages: usages.flags(),
            buffer_flags: buffer_flags.flags(),
            memory_flags: memory_flags.flags(),

            total_size: 0,
            items_size: vec![],
        }
    }

    /// estimate_size is the size in bytes of the buffer to be created. size must be greater than 0.
    pub fn add_item(&mut self, estimate_size: vk::DeviceSize) -> usize {
        let item_index = self.items_size.len();
        self.total_size += estimate_size;
        self.items_size.push(estimate_size);

        item_index
    }
}


#[derive(Debug, Clone)]
pub struct BufferItem {
    /// the handle of the vk::Buffer object.
    pub(crate) handle: vk::Buffer,
    /// the index of buffer in HaBufferRepository/
    pub(crate) buffer_index: usize,
    /// the data offset in the buffer.
    pub(crate) offset: vk::DeviceSize,
    /// the size of this BufferItem represent.
    pub(crate) size  : vk::DeviceSize,
}

impl BufferItem {

    pub fn unset() -> BufferItem {
        BufferItem {
            handle      : vk::Buffer::null(),
            buffer_index: 0,
            offset      : 0,
            size        : 0,
        }
    }
}
