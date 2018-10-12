
use core::physical::QueueOperationType;
use core::physical::{ PhysicalDeviceType, PhysicalFeatureType, DeviceExtensionType };

use core::device::QueueRequestStrategy;
use core::platforms::QUEUE_REQUEST_STRATEGY;

use utility::time::TimePeriod;

#[derive(Debug, Clone)]
pub struct DeviceConfig {

    pub device_types: Vec<PhysicalDeviceType>,
    pub features    : Vec<PhysicalFeatureType>,
    pub extensions  : Vec<DeviceExtensionType>,
    pub queue_operations: Vec<QueueOperationType>,

    pub transfer_wait_time: TimePeriod,
    pub queue_request_strategy: QueueRequestStrategy,
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

            transfer_wait_time: TimePeriod::Infinte,
            queue_request_strategy: QUEUE_REQUEST_STRATEGY,
        }
    }
}

