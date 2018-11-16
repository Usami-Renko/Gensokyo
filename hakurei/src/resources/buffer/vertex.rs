
use vk::resources::buffer::BufferItem;
use vk::resources::buffer::{ BufferCreateFlag, BufferUsageFlag };
use vk::resources::buffer::{ BufferBlockInfo, BufferBlockEntity };
use vk::resources::buffer::{ BufferCopiable, BufferCopyInfo };
use vk::utils::types::vkMemorySize;

use resources::buffer::BufferBranch;
use resources::allocator::buffer::BufferInfosAllocatable;

#[derive(Debug, Clone)]
pub struct VertexBlockInfo {

    flags: Vec<BufferCreateFlag>,

    estimate_size: vkMemorySize,
}

impl VertexBlockInfo {

    pub fn new(estimate_size: vkMemorySize) -> VertexBlockInfo {
        VertexBlockInfo {
            flags: vec![],
            estimate_size,
        }
    }

    pub fn add_flag(&mut self, flag: BufferCreateFlag) {
        self.flags.push(flag);
    }
}

impl BufferBlockInfo for VertexBlockInfo {

    fn create_flags(&self) -> &[BufferCreateFlag] {
        &self.flags
    }

    fn usage_flags(&self) -> &[BufferUsageFlag] {
        &[BufferUsageFlag::VertexBufferBit]
    }

    fn estimate_size(&self) -> vkMemorySize {
        self.estimate_size
    }
}

impl BufferInfosAllocatable for VertexBlockInfo {

    fn branch_type(&self) -> BufferBranch {
        BufferBranch::Vertex
    }

    fn to_staging_info(&self) -> Option<Box<BufferBlockInfo>> {
        Some(Box::new(self.clone()))
    }
}


#[derive(Default)]
pub struct HaVertexBlock {

    repository_index: usize,
    item: BufferItem,
    offsets: Vec<vkMemorySize>,
}

impl HaVertexBlock {

    pub fn uninitialize() -> HaVertexBlock {
        HaVertexBlock::default()
    }

    pub(crate) fn new(item: BufferItem, repository_index: usize) -> HaVertexBlock {

        HaVertexBlock {
            item,
            repository_index,
            offsets: vec![],
        }
    }

    pub fn split_block(&mut self, offsets: Vec<vkMemorySize>) {
        self.offsets = offsets;
    }
}

impl BufferBlockEntity for HaVertexBlock {

    fn item(&self) -> &BufferItem {
        &self.item
    }

    fn offset(&self, sub_index: usize) -> vkMemorySize {
        self.offsets[sub_index]
    }
}

impl BufferCopiable for HaVertexBlock {

    fn copy_info(&self) -> BufferCopyInfo {
        BufferCopyInfo::new(&self.item, 0, self.item.size)
    }
}
