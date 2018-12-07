
use ash::vk;

use crate::buffer::target::BufferDescInfo;
use crate::buffer::entity::BufferBlock;
use crate::buffer::instance::enums::BufferInstanceType;
use crate::buffer::allocator::BufferBlockIndex;
use crate::buffer::traits::{ BufferInstance, BufferBlockInfo };
use crate::buffer::traits::{ BufferCopiable, BufferCopyInfo };

use crate::types::vkbytes;

#[derive(Debug, Clone)]
pub struct IndexBlockInfo {

    info: BufferDescInfo,
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

pub struct GsIndexBlock {

    block: BufferBlock,
    repository_index: usize,
}

impl GsIndexBlock {

    pub(crate) fn new(block: BufferBlock, index: BufferBlockIndex) -> GsIndexBlock {

        GsIndexBlock {
            block,
            repository_index: index.value,
        }
    }
}

impl BufferInstance for GsIndexBlock {

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

impl BufferCopiable for GsIndexBlock {

    fn copy_info(&self) -> BufferCopyInfo {
        BufferCopyInfo::new(&self.block, 0, self.block.size)
    }
}
