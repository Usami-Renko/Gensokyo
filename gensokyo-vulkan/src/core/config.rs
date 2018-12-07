
use crate::core::instance::InstanceConfig;
use crate::core::debug::ValidationConfig;
use crate::core::device::DeviceConfig;
use crate::core::physical::PhysicalConfig;
use crate::core::swapchain::SwapchainConfig;

pub struct CoreConfig {

    pub instance  : InstanceConfig,
    pub validation: ValidationConfig,
    pub device    : DeviceConfig,
    pub physical  : PhysicalConfig,
    pub swapchain : SwapchainConfig,
}
