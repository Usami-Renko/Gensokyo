
use ash::vk;

use crate::buffer::entity::BufferBlock;
use crate::buffer::traits::{ BufferInstance, BufferCopiable, BufferFullCopyInfo };
use crate::buffer::instance::types::BufferCIApi;

use crate::memory::transfer::MemoryDataDelegate;
use crate::memory::MemoryWritePtr;

use crate::error::VkResult;
use crate::types::vkbytes;

pub struct ImgSrcBufferCI {

    estimate_size: vkbytes,
}

impl BufferCIApi for ImgSrcBufferCI {
    type IConveyor = IImgSrc;

    const VK_FLAG: vk::BufferUsageFlags = vk::BufferUsageFlags::TRANSFER_SRC;

    fn estimate_size(&self) -> vkbytes {
        self.estimate_size
    }

    fn into_index(self) -> IImgSrc {
        IImgSrc {}
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

    fn build(block: BufferBlock, _info: Self::InfoType, repository_index: usize) -> Self {
        GsImgsrcBuffer { block, repository_index }
    }

    fn acquire_write_ptr(&self, agency: &mut Box<dyn MemoryDataDelegate>) -> VkResult<MemoryWritePtr> {
        agency.acquire_write_ptr(&self.block, self.repository_index)
    }
}

impl BufferCopiable for GsImgsrcBuffer {

    fn full_copy(&self) -> BufferFullCopyInfo {
        self.block.full_copy()
    }
}

impl GsImgsrcBuffer {

    pub fn new(estimate_size: vkbytes) -> ImgSrcBufferCI {

        ImgSrcBufferCI { estimate_size }
    }
}
