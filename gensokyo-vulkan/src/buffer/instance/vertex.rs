
use ash::vk;

use crate::buffer::target::BufferDescInfo;
use crate::buffer::entity::BufferBlock;
use crate::buffer::instance::enums::BufferInstanceType;
use crate::buffer::allocator::BufferBlockIndex;
use crate::buffer::traits::{ BufferInstance, BufferBlockInfo };
use crate::buffer::traits::{ BufferCopiable, BufferCopyInfo };

use crate::types::vkbytes;

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
pub struct GsVertexBlock {

    block: BufferBlock,
    repository_index: usize,
}

impl GsVertexBlock {

    pub(crate) fn new(block: BufferBlock, index: BufferBlockIndex) -> GsVertexBlock {

        GsVertexBlock {
            block,
            repository_index: index.value,
        }
    }
}

impl BufferInstance for GsVertexBlock {

    fn typ(&self) -> BufferInstanceType {
        BufferInstanceType::VertexBuffer
    }

    fn as_block_ref(&self) -> &BufferBlock {
        &self.block
    }

    fn repository_index(&self) -> usize {
        self.repository_index
    }
}

impl BufferCopiable for GsVertexBlock {

    fn copy_info(&self) -> BufferCopyInfo {
        BufferCopyInfo::new(&self.block, 0, self.block.size)
    }
}
