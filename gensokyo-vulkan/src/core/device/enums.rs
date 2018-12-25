
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
        match self {
            | PrefabQueuePriority::Highest => 1.0,
            | PrefabQueuePriority::High    => 0.8,
            | PrefabQueuePriority::Medium  => 0.6,
            | PrefabQueuePriority::Low     => 0.4,
            | PrefabQueuePriority::Lowest  => 0.2,
        }
    }
}

pub struct DeviceQueueIndex(pub(super) usize);

#[derive(Debug)]
pub enum DeviceQueueIdentifier {

    Graphics,
    Present,
    Transfer,
}

/// The strategy used when request for create device queues.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum QueueRequestStrategy {

    SingleFamilyMultiQueues,
    SingleFamilySingleQueue,
}
