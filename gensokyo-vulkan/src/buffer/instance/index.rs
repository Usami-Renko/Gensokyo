
use ash::vk;

use crate::buffer::entity::BufferBlock;
use crate::buffer::instance::types::BufferCIApi;
use crate::buffer::traits::{ BufferInstance, BufferCopiable, BufferFullCopyInfo };

use crate::memory::transfer::MemoryDataDelegate;
use crate::memory::MemoryWritePtr;

use crate::error::VkResult;
use crate::types::{ vkuint, vkbytes };

use std::mem;

#[derive(Debug, Clone)]
pub struct IndicesBufferCI {

    indices_count: vkuint,
    indices_type: vk::IndexType,
}

impl IndicesBufferCI {

    pub fn set_indices_type(&mut self, typ: vk::IndexType) {
        self.indices_type = typ;
    }
}

impl BufferCIApi for IndicesBufferCI {
    type IConveyor = IIndices;

    const VK_FLAG: vk::BufferUsageFlags = vk::BufferUsageFlags::INDEX_BUFFER;

    fn estimate_size(&self) -> vkbytes {

        let indices_type_size = match self.indices_type {
            | vk::IndexType::UINT16 => mem::size_of::<u16>() as vkbytes,
            | vk::IndexType::UINT32 => mem::size_of::<u32>() as vkbytes,
            | _ => unreachable!(),
        };

        indices_type_size * (self.indices_count as vkbytes)
    }

    fn into_index(self) -> IIndices {

        IIndices {
            indices_type : self.indices_type,
            indices_count: self.indices_count,
        }
    }
}

pub struct IIndices {

    indices_type: vk::IndexType,
    indices_count: vkuint,
}

pub struct GsIndexBuffer {

    indices_count: vkuint,
    indices_type: vk::IndexType,

    block: BufferBlock,
    repository_index: usize,
}

impl BufferInstance for GsIndexBuffer {
    type InfoType = IIndices;

    fn build(block: BufferBlock, info: Self::InfoType, repository_index: usize) -> Self {

        GsIndexBuffer {
            indices_count: info.indices_count,
            indices_type: info.indices_type,
            block, repository_index,
        }
    }

    fn acquire_write_ptr(&self, agency: &mut Box<dyn MemoryDataDelegate>) -> VkResult<MemoryWritePtr> {
        agency.acquire_write_ptr(&self.block, self.repository_index)
    }
}

impl BufferCopiable for GsIndexBuffer {

    fn full_copy(&self) -> BufferFullCopyInfo {
        self.block.full_copy()
    }
}

impl GsIndexBuffer {

    pub fn new(indices_count: usize) -> IndicesBufferCI {

        IndicesBufferCI {
            indices_count: indices_count as vkuint,
            indices_type: vk::IndexType::UINT32,
        }
    }

    pub(crate) fn render_info(&self) -> (vk::Buffer, vk::IndexType) {
        (self.block.handle, self.indices_type)
    }

    pub fn total_count(&self) -> vkuint {
        self.indices_count
    }
}
