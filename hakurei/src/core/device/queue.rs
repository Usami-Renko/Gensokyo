
use ash::vk;
use ash::vk::uint32_t;

use sync::semaphore::HaSemaphore;

use pipeline::stages::PipelineStageFlag;
use resources::command::HaCommandBuffer;

#[derive(Debug, Clone, Copy)]
pub enum QueueUsage {
    Graphics,
    Present,
    Transfer,
}

pub struct HaQueue {
    pub(crate) handle: vk::Queue,

    pub(crate) _usage       : QueueUsage,
    pub(crate) _priority    : f32, // value between [0.0, 1.0]
    pub(crate) family_index : uint32_t,
    pub(crate) _queue_index : uint32_t,
}


impl HaQueue {

    pub fn new(handle: vk::Queue, info: &QueueInfoTmp) -> HaQueue {
        HaQueue {
            handle,
            _usage       : info.usage,
            _priority    : info.priority,
            family_index : info.family_index,
            _queue_index : info.queue_index,
        }
    }
}

pub struct QueueSubmitBundle<'vec, 're: 'vec> {

    pub wait_semaphores: &'vec [&'re HaSemaphore],
    pub sign_semaphores: &'vec [&'re HaSemaphore],
    pub wait_stages    : &'vec [PipelineStageFlag],
    pub commands       : &'vec [&'re HaCommandBuffer],
}

pub struct QueueInfoTmp {

    pub usage: QueueUsage,
    pub priority: f32,
    pub family_index: uint32_t,
    pub queue_index: uint32_t,
}
