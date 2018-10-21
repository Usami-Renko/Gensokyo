
use ash::vk;

use resources::allocator::BufferInfosAllocatable;

pub(crate) struct BufferAllocateInfos {

    pub infos: Vec<Box<BufferInfosAllocatable>>,
    pub spaces: Vec<vk::DeviceSize>,
}

impl BufferAllocateInfos {
    
    pub fn new() -> BufferAllocateInfos {
        BufferAllocateInfos { infos: vec![], spaces: vec![], }
    }
}
