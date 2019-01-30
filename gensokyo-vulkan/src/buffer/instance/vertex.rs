
use ash::vk;

use crate::buffer::entity::BufferBlock;
use crate::buffer::traits::{ BufferInstance, BufferCopiable, BufferCopyInfo };
use crate::buffer::instance::types::BufferCIAbstract;

use crate::memory::transfer::MemoryDataDelegate;
use crate::memory::MemoryWritePtr;

use crate::error::VkResult;
use crate::types::vkbytes;

#[derive(Debug, Clone)]
pub struct VertexBufferCI {

    vertex_size: vkbytes,
    vertex_count: usize,
}

impl BufferCIAbstract<IVertex> for VertexBufferCI {
    const VK_FLAG: vk::BufferUsageFlags = vk::BufferUsageFlags::VERTEX_BUFFER;

    fn estimate_size(&self) -> vkbytes {
        (self.vertex_count as vkbytes) * self.vertex_size
    }

    fn into_index(self) -> IVertex {
        IVertex {}
    }
}

pub struct IVertex {
    // Empty.
}

#[derive(Default)]
pub struct GsVertexBuffer {

    block: BufferBlock,
    repository_index: usize,
}

impl BufferInstance for GsVertexBuffer {
    type InfoType = IVertex;

    fn build(block: BufferBlock, _info: Self::InfoType, repository_index: usize) -> Self {
        GsVertexBuffer { block, repository_index }
    }

    fn acquire_write_ptr(&self, agency: &mut Box<dyn MemoryDataDelegate>) -> VkResult<MemoryWritePtr> {
        agency.acquire_write_ptr(&self.block, self.repository_index)
    }
}

impl BufferCopiable for GsVertexBuffer {

    fn copy_info(&self) -> BufferCopyInfo {
        BufferCopyInfo::new(&self.block, 0, self.block.size)
    }
}

impl GsVertexBuffer {

    pub fn new(vertex_size: vkbytes, vertex_count: usize) -> VertexBufferCI {
        VertexBufferCI { vertex_size, vertex_count }
    }

    pub(crate) fn render_info(&self) -> vk::Buffer {
        self.block.handle
    }
}
