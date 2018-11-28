
use ash::vk;

use buffer::target::BufferDescInfo;
use buffer::entity::BufferBlock;
use buffer::instance::enums::BufferInstanceType;
use buffer::traits::{ BufferInstance, BufferBlockInfo };
use buffer::traits::{ BufferCopiable, BufferCopyInfo };

use types::vkbytes;

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

pub struct HaImgsrcBlock {

    block: BufferBlock,
    repository_index: usize,
}

impl HaImgsrcBlock {

    pub(crate) fn new(block: BufferBlock, repository_index: usize) -> HaImgsrcBlock {
        HaImgsrcBlock {
            block,
            repository_index,
        }
    }
}

impl BufferInstance for HaImgsrcBlock {

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

impl BufferCopiable for HaImgsrcBlock {

    fn copy_info(&self) -> BufferCopyInfo {
        BufferCopyInfo::new(&self.block, 0, self.block.size)
    }
}
