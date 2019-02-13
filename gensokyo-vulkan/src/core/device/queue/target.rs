
use ash::vk;

use crate::sync::GsSemaphore;
use crate::command::GsCommandBuffer;

use crate::types::vkuint;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum QueueUsage {
    Graphics,
    Present,
    Transfer,
}

impl QueueUsage {

    pub fn vk_flag(&self) -> vk::QueueFlags {
        match self {
            | QueueUsage::Graphics
            | QueueUsage::Present  => vk::QueueFlags::GRAPHICS,
            | QueueUsage::Transfer => vk::QueueFlags::TRANSFER,
        }
    }
}

pub struct GsQueue {

    pub handle: vk::Queue,

    pub _usage: QueueUsage,
    pub family_index : vkuint,
    pub _queue_index : vkuint,
}

impl GsQueue {

    pub fn new(handle: vk::Queue, usage: QueueUsage, family_index: vkuint, queue_index: vkuint) -> GsQueue {
        GsQueue {
            handle,
            _usage: usage,
            family_index,
            _queue_index: queue_index,
        }
    }
}

pub struct QueueInitInfo {

    /// The index of requested queue family.
    pub family_index: vkuint,
    /// The priority of each queue in this queue family.
    pub priorities: Vec<f32>,
}

pub struct QueueSubmitBundle<'vec, 're: 'vec> {

    /// semaphore(s) to wait upon before the submitted command buffer starts executing.
    pub wait_semaphores: &'vec [&'re GsSemaphore],
    /// semaphore(s) to be signaled when command buffers have completed.
    pub sign_semaphores: &'vec [&'re GsSemaphore],
    /// list of pipeline stages that the semaphore waits will occur at.
    pub wait_stages    : &'vec [vk::PipelineStageFlags],
    /// command buffers(s) to execute in this batch (submission).
    pub commands       : &'vec [&'re GsCommandBuffer],
}
