
use ash::vk;

use resources::buffer::{ BufferItem, BufferUsageFlag, BufferCreateFlag };
use resources::buffer::{ BufferBlockInfo, BufferBlockEntity };
use resources::allocator::BufferInfosAllocatable;

use utility::marker::VulkanEnum;

#[derive(Debug, Clone)]
pub struct IndexBlockInfo {

    flags: vk::BufferCreateFlags,

    offsets: Vec<vk::DeviceSize>,
    estimate_size: vk::DeviceSize,
}

impl IndexBlockInfo {

    pub fn new(estimate_size: vk::DeviceSize) -> IndexBlockInfo {
        IndexBlockInfo {
            flags: vk::BufferCreateFlags::empty(),
            offsets: vec![],
            estimate_size,
        }
    }

    pub fn split_block(&mut self, offsets: Vec<vk::DeviceSize>) {
        self.offsets = offsets;
    }

    pub fn add_flag(&mut self, flag: BufferCreateFlag) {
        self.flags |= flag.value()
    }
}

impl BufferBlockInfo for IndexBlockInfo {

    fn flags(&self) -> vk::BufferCreateFlags {
        self.flags
    }

    fn usage(&self) -> vk::BufferUsageFlags {
        BufferUsageFlag::IndexBufferBit.value()
    }

    fn total_size(&self) -> vk::DeviceSize {
        self.estimate_size
    }
}

impl BufferInfosAllocatable for IndexBlockInfo {

    fn to_staging_info(&self) -> Option<Box<BufferBlockInfo>> {
        Some(Box::new(self.clone()))
    }
}


pub struct HaIndexBlock {

    offsets: Vec<vk::DeviceSize>,
    item: BufferItem,
}

impl HaIndexBlock {

    pub fn uninitialize() -> HaIndexBlock {
        HaIndexBlock {
            offsets: vec![],
            item: BufferItem::unset(),
        }
    }

    pub(crate) fn from(info: &IndexBlockInfo, item: BufferItem) -> HaIndexBlock {

        HaIndexBlock {
            offsets: info.offsets.clone(),
            item,
        }
    }
}

impl BufferBlockEntity for HaIndexBlock {

    fn get_buffer_item(&self) -> &BufferItem {
        &self.item
    }

    fn offset(&self, sub_index: usize) -> vk::DeviceSize {
        self.offsets[sub_index]
    }
}
