
use types::vkbytes;

use buffer::target::BufferDescInfo;

#[derive(Default)]
pub struct BufferAllocateInfos {

    pub infos: Vec<BufferDescInfo>,
    pub spaces: Vec<vkbytes>,
}

impl BufferAllocateInfos {
    
    pub fn new() -> BufferAllocateInfos {
        Default::default()
    }
}
