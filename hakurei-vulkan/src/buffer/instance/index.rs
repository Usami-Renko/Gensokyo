
use ash::vk;

use buffer::target::BufferDescInfo;
use buffer::entity::BufferBlock;
use buffer::instance::enums::BufferInstanceType;
use buffer::traits::{ BufferInstance, BufferBlockInfo };
use buffer::traits::{ BufferCopiable, BufferCopyInfo };

use types::vkbytes;

#[derive(Debug, Clone)]
pub struct IndexBlockInfo {

    info: BufferDescInfo
}

impl IndexBlockInfo {

    pub fn new(estimate_size: vkbytes) -> IndexBlockInfo {

        IndexBlockInfo {
            info: BufferDescInfo::new(estimate_size, vk::BufferUsageFlags::INDEX_BUFFER),
        }
    }
}

impl BufferBlockInfo for IndexBlockInfo {
    const INSTANCE_TYPE: BufferInstanceType = BufferInstanceType::IndexBuffer;

    fn as_desc_ref(&self) -> &BufferDescInfo {
        &self.info
    }

    fn into_desc(self) -> BufferDescInfo {
        self.info
    }
}

pub struct HaIndexBlock {

    block: BufferBlock,
    repository_index: usize,
}

impl HaIndexBlock {

    pub(crate) fn new(block: BufferBlock, repository_index: usize) -> HaIndexBlock {

        HaIndexBlock {
            block,
            repository_index,
        }
    }
}

impl BufferInstance for HaIndexBlock {

    fn typ(&self) -> BufferInstanceType {
        BufferInstanceType::IndexBuffer
    }

    fn as_block_ref(&self) -> &BufferBlock {
        &self.block
    }

    fn repository_index(&self) -> usize {
        self.repository_index
    }
}

impl BufferCopiable for HaIndexBlock {

    fn copy_info(&self) -> BufferCopyInfo {
        BufferCopyInfo::new(&self.block, 0, self.block.size)
    }
}
