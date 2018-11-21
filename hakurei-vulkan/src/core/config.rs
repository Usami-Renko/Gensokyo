
use core::instance::InstanceConfig;
use core::debug::ValidationConfig;
use core::device::DeviceConfig;
use core::swapchain::SwapchainConfig;

pub struct CoreConfig {

    pub instance  : InstanceConfig,
    pub validation: ValidationConfig,
    pub device    : DeviceConfig,
    pub swapchain : SwapchainConfig,
}
