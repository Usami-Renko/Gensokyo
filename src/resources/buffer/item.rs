
use ash::vk;

use resources::buffer::{ BufferUsageFlag, BufferCreateFlag };
use resources::memory::MemoryPropertyFlag;

pub struct BufferConfig<'flag> {

    pub(crate) usages       : &'flag [BufferUsageFlag],
    // TODO: Turn the flags into bool options.
    pub(crate) buffer_flags : &'flag [BufferCreateFlag],
    pub(crate) memory_flags : &'flag [MemoryPropertyFlag],

    pub(crate) total_size   : vk::DeviceSize,
    pub(crate) items_size   : Vec<vk::DeviceSize>,
}

impl<'flag> BufferConfig<'flag> {

    pub fn init(usages: &'flag [BufferUsageFlag], memory_flags: &'flag [MemoryPropertyFlag]) -> BufferConfig<'flag> {
        BufferConfig {
            usages,
            buffer_flags: &[],
            memory_flags,

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
    pub fn with_buffer_flags(&mut self, buffer_flags: &'flag [BufferCreateFlag]) {
        self.buffer_flags = buffer_flags;
    }
}


pub struct BufferItem {
    pub(crate) buffer_index: usize,
    pub(crate) offset: vk::DeviceSize,
    pub(crate) size  : vk::DeviceSize,
}