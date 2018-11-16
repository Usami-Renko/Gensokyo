
use vk::utils::types::vkMemorySize;

use resources::allocator::buffer::traits::BufferInfosAllocatable;

pub struct BufferAllocateInfos {

    pub infos: Vec<Box<BufferInfosAllocatable>>,
    pub spaces: Vec<vkMemorySize>,
}

impl BufferAllocateInfos {
    
    pub fn new() -> BufferAllocateInfos {
        BufferAllocateInfos { infos: vec![], spaces: vec![], }
    }
}
