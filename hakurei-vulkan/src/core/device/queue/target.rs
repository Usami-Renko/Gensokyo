
use ash::vk;

use core::device::enums::PrefabQueuePriority;

use sync::HaSemaphore;
use command::HaCommandBuffer;

use types::vkuint;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum QueueUsage {
    Graphics,
    Present,
    Transfer,
}

pub struct HaQueue {

    pub handle: vk::Queue,

    pub _priority    : PrefabQueuePriority,
    pub family_index : vkuint,
    pub _queue_index : vkuint,
}


impl HaQueue {

    pub fn new(handle: vk::Queue, priority: PrefabQueuePriority, family_index: vkuint, queue_index: vkuint) -> HaQueue {
        HaQueue {
            handle,
            _priority: priority,
            family_index,
            _queue_index: queue_index,
        }
    }
}

pub struct QueueSubmitBundle<'vec, 're: 'vec> {

    /// semaphore(s) to wait upon before the submitted command buffer starts executing.
    pub wait_semaphores: &'vec [&'re HaSemaphore],
    /// semaphore(s) to be signaled when command buffers have completed.
    pub sign_semaphores: &'vec [&'re HaSemaphore],
    /// list of pipeline stages that the semaphore waits will occur at.
    pub wait_stages    : &'vec [vk::PipelineStageFlags],
    /// command buffers(s) to execute in this batch (submission).
    pub commands       : &'vec [&'re HaCommandBuffer],
}
