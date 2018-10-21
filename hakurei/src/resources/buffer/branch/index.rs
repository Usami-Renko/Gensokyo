
use ash::vk;

use resources::buffer::{ BufferSubItem, BufferUsageFlag };
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

    #[allow(dead_code)]
    offsets: Vec<vk::DeviceSize>,
    item: BufferSubItem,
}

impl HaIndexBlock {

    pub fn uninitialize() -> HaIndexBlock {
        HaIndexBlock {
            offsets: vec![],
            item: BufferSubItem::unset(),
        }
    }

    pub(crate) fn from(info: &IndexBlockInfo, item: BufferSubItem) -> HaIndexBlock {

        HaIndexBlock {
            offsets: info.offsets.clone(),
            item,
        }
    }
}

impl BufferBlockEntity for HaIndexBlock {

    fn get_buffer_item(&self) -> &BufferSubItem {
        &self.item
    }
}
