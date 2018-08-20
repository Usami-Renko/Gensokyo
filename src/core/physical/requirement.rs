
use core::physical::features::PhysicalFeatureType;
use core::physical::PhysicalDeviceType;
use core::physical::family::QueueOperationType;

pub struct PhysicalRequirement {

    pub device_types     : Vec<PhysicalDeviceType>,
    pub features         : Vec<PhysicalFeatureType>,
    pub queue_operations : Vec<QueueOperationType>,

    // TODO: Add memories requriement
}

impl PhysicalRequirement {

    pub fn init() -> PhysicalRequirement {
        PhysicalRequirement {
            device_types: vec![],
            features: vec![],
            queue_operations: vec![],
        }
    }

    pub fn require_device_types(mut self, types: Vec<PhysicalDeviceType>) -> PhysicalRequirement {
        self.device_types = types;
        self
    }

    pub fn require_features(mut self, features: Vec<PhysicalFeatureType>) -> PhysicalRequirement {
        self.features = features;
        self
    }

    pub fn require_queue_operations(mut self, operations: Vec<QueueOperationType>) -> PhysicalRequirement {
        self.queue_operations = operations;
        self
    }
}
