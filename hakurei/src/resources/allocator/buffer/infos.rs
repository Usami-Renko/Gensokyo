
use ash::vk;

use resources::buffer::BufferConfigAbstract;

pub(crate) struct BufferAllocateInfos {

    pub configs: Vec<Box<BufferConfigAbstract>>,
    pub spaces : Vec<vk::DeviceSize>,
}

impl BufferAllocateInfos {
    
    pub fn new() -> BufferAllocateInfos {
        BufferAllocateInfos { configs: vec![], spaces: vec![], }
    }
}
