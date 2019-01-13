
use ash::vk;

use crate::buffer::entity::BufferBlock;
use crate::buffer::instance::types::BufferInfoAbstract;
use crate::buffer::traits::{ BufferInstance, BufferCopiable, BufferCopyInfo };

use crate::memory::transfer::MemoryDataDelegate;
use crate::memory::{ MemoryWritePtr, MemoryError };
use crate::types::{ vkuint, vkbytes };

use std::mem;

#[derive(Debug, Clone)]
pub struct GsBufIndicesInfo {

    indices_count: vkuint,
    indices_type: vk::IndexType,
}

impl GsBufIndicesInfo {

    pub fn new(indices_count: usize) -> GsBufIndicesInfo {

        GsBufIndicesInfo {
            indices_count: indices_count as vkuint,
            indices_type: vk::IndexType::UINT32,
        }
    }
}

impl BufferInfoAbstract<IIndices> for GsBufIndicesInfo {
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
            indices_type: self.indices_type,
        }
    }
}

pub struct IIndices {

    indices_type: vk::IndexType,
}

pub struct GsIndexBuffer {

    indices_type: vk::IndexType,

    block: BufferBlock,
    repository_index: usize,
}

impl BufferInstance for GsIndexBuffer {
    type InfoType = IIndices;

    fn new(block: BufferBlock, info: Self::InfoType, repository_index: usize) -> Self {

        GsIndexBuffer {
            indices_type: info.indices_type,
            block, repository_index,
        }
    }

    fn acquire_write_ptr(&self, agency: &mut Box<dyn MemoryDataDelegate>) -> Result<MemoryWritePtr, MemoryError> {
        agency.acquire_write_ptr(&self.block, self.repository_index)
    }
}

impl BufferCopiable for GsIndexBuffer {

    fn copy_info(&self) -> BufferCopyInfo {
        BufferCopyInfo::new(&self.block, 0, self.block.size)
    }
}

impl GsIndexBuffer {

    pub fn set_indices_type(&mut self, typ: vk::IndexType) {
        self.indices_type = typ;
    }

    pub(crate) fn render_info(&self) -> (vk::Buffer, vk::IndexType) {
        (self.block.handle, self.indices_type)
    }
}
