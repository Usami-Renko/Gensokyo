
use vk::resources::buffer::BufferItem;
use vk::resources::buffer::{ BufferCreateFlag, BufferUsageFlag };
use vk::resources::buffer::{ BufferBlockInfo, BufferBlockEntity };
use vk::resources::buffer::{ BufferCopiable, BufferCopyInfo };
use vk::utils::types::vkMemorySize;

use resources::buffer::BufferBranch;
use resources::allocator::buffer::BufferInfosAllocatable;

pub struct ImgsrcBlockInfo {

    flags: Vec<BufferCreateFlag>,
    estimate_size: vkMemorySize,
}

impl ImgsrcBlockInfo {

    pub fn new(estimate_size: vkMemorySize) -> ImgsrcBlockInfo {

        ImgsrcBlockInfo {
            flags: vec![],
            estimate_size,
        }
    }

    pub fn add_flag(&mut self, flag: BufferCreateFlag) {
        self.flags.push(flag);
    }
}

impl BufferBlockInfo for ImgsrcBlockInfo {

    fn create_flags(&self) -> &[BufferCreateFlag] {
        &self.flags
    }

    fn usage_flags(&self) -> &[BufferUsageFlag] {
        &[BufferUsageFlag::TransferSrcBit]
    }

    fn estimate_size(&self) -> vkMemorySize {
        self.estimate_size
    }
}

impl BufferInfosAllocatable for ImgsrcBlockInfo {

    fn branch_type(&self) -> BufferBranch {
        BufferBranch::ImageSrc
    }
}


pub struct HaImgsrcBlock {

    item: BufferItem,
    repository_index: usize,
}

impl HaImgsrcBlock {

    pub(crate) fn new(item: BufferItem, repository_index: usize) -> HaImgsrcBlock {
        HaImgsrcBlock {
            item,
            repository_index,
        }
    }
}

impl BufferBlockEntity for HaImgsrcBlock {

    fn item(&self) -> &BufferItem {
        &self.item
    }

    fn offset(&self, _sub_index: usize) -> vkMemorySize {
        unimplemented!()
    }
}

impl BufferCopiable for HaImgsrcBlock {

    fn copy_info(&self) -> BufferCopyInfo {
        BufferCopyInfo::new(&self.item, 0, self.item.size)
    }
}
