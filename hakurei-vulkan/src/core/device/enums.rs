
use ash::vk;

use utils::marker::VulkanEnum;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PrefabQueuePriority {

    Highest,
    High,
    Medium,
    Low,
    Lowest,
}

impl PrefabQueuePriority {

    pub fn value(&self) -> f32 {
        match *self {
            | PrefabQueuePriority::Highest => 1.0,
            | PrefabQueuePriority::High    => 0.8,
            | PrefabQueuePriority::Medium  => 0.6,
            | PrefabQueuePriority::Low     => 0.4,
            | PrefabQueuePriority::Lowest  => 0.2,
        }
    }
}

pub enum DeviceQueueIdentifier {

    Graphics,
    Present,
    Transfer,
    Custom { identifier: Box<DeviceQueueIdentifier>, queue_index: usize },
}

/// The strategy used when request for create device queues.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum QueueRequestStrategy {

    SingleFamilyMultiQueues,
    SingleFamilySingleQueue,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SharingMode {

    /// `Exculsive`  specifies that access to any range or image subresource of the object will be exclusive to a single queue family at a time.
    Exclusive,
    /// `Concurrent` specifies that concurrent access to any range or image subresource of the object from multiple queue families is supported.
    Concurrent,
}

impl VulkanEnum for SharingMode {
    type EnumType = vk::SharingMode;

    fn value(&self) -> Self::EnumType {
        match self {
            | SharingMode::Exclusive  => vk::SharingMode::Exclusive,
            | SharingMode::Concurrent => vk::SharingMode::Concurrent,
        }
    }
}
