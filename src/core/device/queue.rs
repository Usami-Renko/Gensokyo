
use ash::vk;
use ash::vk::uint32_t;

#[derive(Debug, Clone, Copy)]
pub enum QueueUsage {
    Graphics,
    Present,
}

pub struct QueueInfo {
    pub handle       : vk::Queue,

    pub usage        : QueueUsage,
    pub priority     : f32,
    pub family_index : uint32_t,
    pub queue_index  : uint32_t,
}

impl QueueInfo {

    pub fn new(handle: vk::Queue, info: &QueueInfoTmp) -> QueueInfo {
        QueueInfo {
            handle,
            usage        : info.usage,
            priority     : info.priority,
            family_index : info.family_index,
            queue_index  : info.queue_index,
        }
    }
}

pub struct QueueInfoTmp {

    pub usage: QueueUsage,
    pub priority: f32,
    pub family_index: uint32_t,
    pub queue_index: uint32_t,
}
