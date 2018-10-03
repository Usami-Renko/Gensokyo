
use core::physical::PhysicalDeviceType;
use core::physical::PhysicalFeatureType;
use core::physical::DeviceExtensionType;
use core::physical::QueueOperationType;

pub struct DeviceConfig {

    pub device_types: Vec<PhysicalDeviceType>,
    pub features    : Vec<PhysicalFeatureType>,
    pub extensions  : Vec<DeviceExtensionType>,
    pub queue_operations: Vec<QueueOperationType>,
}

impl Default for DeviceConfig {

    fn default() -> DeviceConfig {

        DeviceConfig {
            device_types: vec![
                PhysicalDeviceType::CPU,
                PhysicalDeviceType::IntegratedGPU,
                PhysicalDeviceType::DiscreteGPU,
            ],
            features: vec![],
            extensions: vec![
                DeviceExtensionType::Swapchain,
            ],
            queue_operations: vec![],
        }
    }
}

