
use core::instance::InstanceConfig;
use core::debug::ValidationConfig;
use core::device::DeviceConfig;
use core::swapchain::SwapchainConfig;
use core::physical::PhysicalRequirement;

pub struct CoreConfig {

    pub instance  : InstanceConfig,
    pub validation: ValidationConfig,
    pub device    : DeviceConfig,
    pub swapchain : SwapchainConfig,
}

impl CoreConfig {

    pub(crate) fn to_physical_requirement(&self) -> PhysicalRequirement {
        PhysicalRequirement::init()
            .require_device_types(self.device.device_types.clone())
            .require_features(self.device.features.clone())
            .require_queue_extensions(self.device.extensions.clone())
            .require_queue_operations(self.device.queue_operations.clone())
            .require_swapchain_image_count(self.swapchain.image_count)
    }
}
