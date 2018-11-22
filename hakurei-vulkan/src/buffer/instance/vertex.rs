
use ash::vk;

use buffer::target::BufferDescInfo;
use buffer::entity::BufferBlock;
use buffer::instance::enums::BufferInstanceType;
use buffer::traits::{ BufferInstance, BufferBlockInfo };
use buffer::traits::{ BufferCopiable, BufferCopyInfo };

use types::vkbytes;

#[derive(Debug, Clone)]
pub struct VertexBlockInfo {

    info: BufferDescInfo,
}

impl VertexBlockInfo {

    pub fn new(estimate_size: vkbytes) -> VertexBlockInfo {

        VertexBlockInfo {
            info: BufferDescInfo::new(estimate_size, vk::BufferUsageFlags::VERTEX_BUFFER),
        }
    }
}

impl BufferBlockInfo for VertexBlockInfo {
    const INSTANCE_TYPE: BufferInstanceType = BufferInstanceType::VertexBuffer;

    fn as_desc_ref(&self) -> &BufferDescInfo {
        &self.info
    }

    fn into_desc(self) -> BufferDescInfo {
        self.info
    }
}

#[derive(Default)]
pub struct HaVertexBlock {

    repository_index: usize,
    block: BufferBlock,
}

impl HaVertexBlock {

    pub(crate) fn new(block: BufferBlock, repository_index: usize) -> HaVertexBlock {

        HaVertexBlock {
            block,
            repository_index,
        }
    }
}

impl BufferInstance for HaVertexBlock {

    fn typ(&self) -> BufferInstanceType {
        BufferInstanceType::VertexBuffer
    }

    fn as_block_ref(&self) -> &BufferBlock {
        &self.block
    }
}

impl BufferCopiable for HaVertexBlock {

    fn copy_info(&self) -> BufferCopyInfo {
        BufferCopyInfo::new(&self.block, 0, self.block.size)
    }
}
