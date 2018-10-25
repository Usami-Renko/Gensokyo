
use ash::vk::uint32_t;

use core::physical::features::PhysicalFeatureType;
use core::physical::PhysicalDeviceType;
use core::physical::family::QueueOperationType;
use core::physical::extension::DeviceExtensionType;

pub(crate) struct PhysicalRequirement {

    pub device_types     : Vec<PhysicalDeviceType>,
    pub features         : Vec<PhysicalFeatureType>,
    pub queue_operations : Vec<QueueOperationType>,
    pub extensions       : Vec<DeviceExtensionType>,

    pub swapchain_image_count: uint32_t,

    // TODO: Add memories requriement
}

impl PhysicalRequirement {

    pub fn init() -> PhysicalRequirement {
        PhysicalRequirement {
            device_types:     vec![],
            features:         vec![],
            queue_operations: vec![],
            extensions:       vec![],

            swapchain_image_count: 2,
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

    pub fn require_queue_extensions(mut self, extensions: Vec<DeviceExtensionType>) -> PhysicalRequirement {
        self.extensions = extensions;
        self
    }

    pub fn require_swapchain_image_count(mut self, image_count: uint32_t) -> PhysicalRequirement {
        self.swapchain_image_count = image_count;
        self
    }
}
