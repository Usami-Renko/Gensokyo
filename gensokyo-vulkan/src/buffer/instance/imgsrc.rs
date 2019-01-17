
use ash::vk;

use crate::buffer::entity::BufferBlock;
use crate::buffer::traits::{ BufferInstance, BufferCopiable, BufferCopyInfo };
use crate::buffer::instance::types::BufferInfoAbstract;

use crate::memory::transfer::MemoryDataDelegate;
use crate::memory::MemoryWritePtr;

use crate::error::VkResult;
use crate::types::vkbytes;

pub struct GsBufImgsrcInfo {

    estimate_size: vkbytes,
}

impl BufferInfoAbstract<IImgSrc> for GsBufImgsrcInfo {
    const VK_FLAG: vk::BufferUsageFlags = vk::BufferUsageFlags::TRANSFER_SRC;

    fn estimate_size(&self) -> vkbytes {
        self.estimate_size
    }

    fn into_index(self) -> IImgSrc {
        IImgSrc {}
    }
}

impl GsBufImgsrcInfo {

    pub fn new(estimate_size: vkbytes) -> GsBufImgsrcInfo {

        GsBufImgsrcInfo { estimate_size }
    }
}

pub struct IImgSrc {
    // Empty.
}

pub struct GsImgsrcBuffer {

    block: BufferBlock,
    repository_index: usize,
}

impl BufferInstance for GsImgsrcBuffer {
    type InfoType = IImgSrc;

    fn new(block: BufferBlock, _info: Self::InfoType, repository_index: usize) -> Self {
        GsImgsrcBuffer { block, repository_index }
    }

    fn acquire_write_ptr(&self, agency: &mut Box<dyn MemoryDataDelegate>) -> VkResult<MemoryWritePtr> {
        agency.acquire_write_ptr(&self.block, self.repository_index)
    }
}

impl BufferCopiable for GsImgsrcBuffer {

    fn copy_info(&self) -> BufferCopyInfo {
        BufferCopyInfo::new(&self.block, 0, self.block.size)
    }
}
