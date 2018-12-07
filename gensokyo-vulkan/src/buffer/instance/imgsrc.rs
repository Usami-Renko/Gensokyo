
use ash::vk;

use crate::buffer::target::BufferDescInfo;
use crate::buffer::entity::BufferBlock;
use crate::buffer::instance::enums::BufferInstanceType;
use crate::buffer::allocator::BufferBlockIndex;
use crate::buffer::traits::{ BufferInstance, BufferBlockInfo };
use crate::buffer::traits::{ BufferCopiable, BufferCopyInfo };

use crate::types::vkbytes;

pub struct ImgsrcBlockInfo {

    info: BufferDescInfo,
}

impl ImgsrcBlockInfo {

    pub fn new(estimate_size: vkbytes) -> ImgsrcBlockInfo {

        ImgsrcBlockInfo {
            info: BufferDescInfo::new(estimate_size, vk::BufferUsageFlags::TRANSFER_SRC),
        }
    }
}

impl BufferBlockInfo for ImgsrcBlockInfo {
    const INSTANCE_TYPE: BufferInstanceType = BufferInstanceType::ImageSrcBuffer;

    fn as_desc_ref(&self) -> &BufferDescInfo {
        &self.info
    }

    fn into_desc(self) -> BufferDescInfo {
        self.info
    }
}

pub struct GsImgsrcBlock {

    block: BufferBlock,
    repository_index: usize,
}

impl GsImgsrcBlock {

    pub(crate) fn new(block: BufferBlock, index: BufferBlockIndex) -> GsImgsrcBlock {
        GsImgsrcBlock {
            block,
            repository_index: index.value,
        }
    }
}

impl BufferInstance for GsImgsrcBlock {

    fn typ(&self) -> BufferInstanceType {
        BufferInstanceType::ImageSrcBuffer
    }

    fn as_block_ref(&self) -> &BufferBlock {
        &self.block
    }

    fn repository_index(&self) -> usize {
        self.repository_index
    }
}

impl BufferCopiable for GsImgsrcBlock {

    fn copy_info(&self) -> BufferCopyInfo {
        BufferCopyInfo::new(&self.block, 0, self.block.size)
    }
}
