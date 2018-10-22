
use ash::vk;

use resources::buffer::{ BufferItem, BufferUsageFlag };
use resources::buffer::{ BufferBlockInfo, BufferBlockEntity };
use resources::allocator::BufferInfosAllocatable;

use utility::marker::VulkanEnum;

#[derive(Debug, Clone)]
pub struct VertexBlockInfo {

    flags: vk::BufferCreateFlags,

    offsets: Vec<vk::DeviceSize>,
    estimate_size: vk::DeviceSize,
}

impl VertexBlockInfo {

    pub fn new(estimate_size: vk::DeviceSize) -> VertexBlockInfo {
        VertexBlockInfo {
            flags: vk::BufferCreateFlags::empty(),
            offsets: vec![],
            estimate_size,
        }
    }

    pub fn split_block(&mut self, offsets: Vec<vk::DeviceSize>) {
        self.offsets = offsets;
    }
}

impl BufferBlockInfo for VertexBlockInfo {

    fn flags(&self) -> vk::BufferCreateFlags {
        self.flags
    }

    fn usage(&self) -> vk::BufferUsageFlags {
        BufferUsageFlag::VertexBufferBit.value()
    }

    fn total_size(&self) -> vk::DeviceSize {
        self.estimate_size
    }
}

impl BufferInfosAllocatable for VertexBlockInfo {

    fn to_staging_info(&self) -> Option<Box<BufferBlockInfo>> {
        Some(Box::new(self.clone()))
    }
}


pub struct HaVertexBlock {

    offsets: Vec<vk::DeviceSize>,
    item: BufferItem,
}

impl HaVertexBlock {

    pub fn uninitialize() -> HaVertexBlock {
        HaVertexBlock {
            offsets: vec![],
            item: BufferItem::unset(),
        }
    }

    pub(crate) fn from(info: &VertexBlockInfo, item: BufferItem) -> HaVertexBlock {

        HaVertexBlock {
            offsets: info.offsets.clone(),
            item,
        }
    }
}

impl BufferBlockEntity for HaVertexBlock {

    fn get_buffer_item(&self) -> &BufferItem {
        &self.item
    }

    fn offset(&self, sub_index: usize) -> vk::DeviceSize {
        self.offsets[sub_index]
    }
}
