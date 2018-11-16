
use vk::resources::buffer::BufferItem;
use vk::resources::buffer::{ BufferCreateFlag, BufferUsageFlag };
use vk::resources::buffer::{ BufferBlockInfo, BufferBlockEntity };
use vk::resources::buffer::{ BufferCopiable, BufferCopyInfo };
use vk::utils::types::vkMemorySize;

use resources::buffer::BufferBranch;
use resources::allocator::buffer::BufferInfosAllocatable;

#[derive(Debug, Clone)]
pub struct IndexBlockInfo {

    flags: Vec<BufferCreateFlag>,

    estimate_size: vkMemorySize,
}

impl IndexBlockInfo {

    pub fn new(estimate_size: vkMemorySize) -> IndexBlockInfo {
        IndexBlockInfo {
            flags: vec![],
            estimate_size,
        }
    }

    pub fn add_flag(&mut self, flag: BufferCreateFlag) {
        self.flags.push(flag);
    }
}

impl BufferBlockInfo for IndexBlockInfo {

    fn create_flags(&self) -> &[BufferCreateFlag] {
        &self.flags
    }

    fn usage_flags(&self) -> &[BufferUsageFlag] {
        &[BufferUsageFlag::IndexBufferBit]
    }

    fn estimate_size(&self) -> vkMemorySize {
        self.estimate_size
    }
}

impl BufferInfosAllocatable for IndexBlockInfo {

    fn branch_type(&self) -> BufferBranch {
        BufferBranch::Index
    }

    fn to_staging_info(&self) -> Option<Box<BufferBlockInfo>> {
        Some(Box::new(self.clone()))
    }
}


#[derive(Default)]
pub struct HaIndexBlock {

    item: BufferItem,
    repository_index: usize,
    offsets: Vec<vkMemorySize>,
}

impl HaIndexBlock {

    pub fn uninitialize() -> HaIndexBlock {
        HaIndexBlock::default()
    }

    pub(crate) fn new(item: BufferItem, repository_index: usize) -> HaIndexBlock {

        HaIndexBlock {
            item,
            repository_index,
            offsets: vec![],
        }
    }

    pub fn split_block(&mut self, offsets: Vec<vkMemorySize>) {
        self.offsets = offsets;
    }
}

impl BufferBlockEntity for HaIndexBlock {

    fn item(&self) -> &BufferItem {
        &self.item
    }

    fn offset(&self, sub_index: usize) -> vkMemorySize {
        self.offsets[sub_index]
    }
}

impl BufferCopiable for HaIndexBlock {

    fn copy_info(&self) -> BufferCopyInfo {
        BufferCopyInfo::new(&self.item, 0, self.item.size)
    }
}
