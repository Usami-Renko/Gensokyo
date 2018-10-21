
use ash::vk;

use resources::buffer::{ BufferSubItem, BufferUsageFlag };
use resources::buffer::{ BufferBlockInfo, BufferBlockEntity };
use resources::allocator::BufferInfosAllocatable;

use utility::marker::VulkanEnum;

pub(crate) struct ImgsrcBlockInfo {

    flags: vk::BufferCreateFlags,
    estimate_size: vk::DeviceSize,
}

impl ImgsrcBlockInfo {

    pub fn new(estimate_size: vk::DeviceSize) -> ImgsrcBlockInfo {
        ImgsrcBlockInfo {
            flags: vk::BufferCreateFlags::empty(),
            estimate_size,
        }
    }
}

impl BufferBlockInfo for ImgsrcBlockInfo {

    fn flags(&self) -> vk::BufferCreateFlags {
        self.flags
    }

    fn usage(&self) -> vk::BufferUsageFlags {
        BufferUsageFlag::TransferSrcBit.value()
    }

    fn total_size(&self) -> vk::DeviceSize {
        self.estimate_size
    }
}

impl BufferInfosAllocatable for ImgsrcBlockInfo {}


pub(crate) struct HaImgsrcBlock {

    item: BufferSubItem,
}

impl HaImgsrcBlock {

    pub(crate) fn from(item: BufferSubItem) -> HaImgsrcBlock {
        HaImgsrcBlock {
            item,
        }
    }
}

impl BufferBlockEntity for HaImgsrcBlock {

    fn get_buffer_item(&self) -> &BufferSubItem {
        &self.item
    }
}
