
use ash::vk;
use ash::vk::uint32_t;

use core::device::PrefabQueuePriority;
use sync::semaphore::HaSemaphore;

use pipeline::stages::PipelineStageFlag;
use resources::command::HaCommandBuffer;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum QueueUsage {
    Graphics,
    Present,
    Transfer,
}

pub struct HaQueue {

    pub(crate) handle: vk::Queue,

    pub(crate) _priority    : PrefabQueuePriority,
    pub(crate) family_index : uint32_t,
    pub(crate) _queue_index : uint32_t,
}


impl HaQueue {

    pub(crate) fn new(handle: vk::Queue, priority: PrefabQueuePriority, family_index: uint32_t, queue_index: uint32_t) -> HaQueue {
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
    pub wait_stages    : &'vec [PipelineStageFlag],
    /// command buffers(s) to execute in this batch (submission).
    pub commands       : &'vec [&'re HaCommandBuffer],
}
