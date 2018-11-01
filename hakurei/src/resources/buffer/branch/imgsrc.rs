
use ash::vk;

use resources::buffer::{ BufferItem, BufferUsageFlag, BufferCreateFlag };
use resources::buffer::{ BufferBlockInfo, BufferBlockEntity };
use resources::buffer::{ BufferCopiable, BufferCopyInfo };
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

    #[allow(dead_code)]
    pub fn add_flag(&mut self, flag: BufferCreateFlag) {
        self.flags |= flag.value()
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

    item: BufferItem,
}

impl HaImgsrcBlock {

    pub(crate) fn from(item: BufferItem) -> HaImgsrcBlock {
        HaImgsrcBlock {
            item,
        }
    }
}

impl BufferBlockEntity for HaImgsrcBlock {

    fn get_buffer_item(&self) -> &BufferItem {
        &self.item
    }

    fn offset(&self, _sub_index: usize) -> vk::DeviceSize {
        unimplemented!()
    }
}

impl BufferCopiable for HaImgsrcBlock {

    fn copy_info(&self) -> BufferCopyInfo {
        BufferCopyInfo {
            handle: self.item.handle,
            offset: 0,
            size  : self.item.size,
        }
    }
}
