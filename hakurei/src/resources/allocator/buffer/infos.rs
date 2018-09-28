
use ash::vk;

use resources::allocator::BufferConfigsAllocatable;

pub struct BufferAllocateInfos {

    pub configs: Vec<Box<BufferConfigsAllocatable>>,
    pub spaces : Vec<vk::DeviceSize>,
}

impl BufferAllocateInfos {
    
    pub fn new() -> BufferAllocateInfos {
        BufferAllocateInfos { configs: vec![], spaces: vec![], }
    }

    pub fn from_spaces(spaces: &Vec<vk::DeviceSize>) -> BufferAllocateInfos {
        BufferAllocateInfos {
            configs: vec![],
            spaces : spaces.clone(),
        }
    }
}
