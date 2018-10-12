
use ash::vk::uint32_t;

use config::core::{ DeviceConfig, SwapchainConfig };
use core::debug::{ ValidationInfo, DebugReportFlag };
use core::physical::PhysicalRequirement;

pub const APPLICATION_VERSION: uint32_t = vk_make_version!(1, 0, 0);
pub const ENGINE_VERSION:      uint32_t = vk_make_version!(1, 0, 0);
pub const API_VERSION:         uint32_t = vk_make_version!(1, 0, 85);

pub const APPLICATION_NAME: &'static str = "Hakurei Program";
pub const ENGINE_NAME:      &'static str = "Hakurei Rendering Engine";

pub struct CoreConfig {

    pub validation: ValidationInfo,
    pub device    : DeviceConfig,
    pub swapchain : SwapchainConfig,
}

impl Default for CoreConfig {

    fn default() -> CoreConfig {
        CoreConfig {
            validation: ValidationInfo {
                is_enable: true,
                required_validation_layers: vec![
                    String::from("VK_LAYER_LUNARG_standard_validation"),
                ],
                flags: vec![
                    DebugReportFlag::ErrorBit,
                    // DebugReportFlag::InformationBit,
                    DebugReportFlag::WarningBit,
                    DebugReportFlag::PerformanceWarningBit,
                ],
            },

            device: DeviceConfig::default(),

            swapchain: SwapchainConfig::default(),
        }
    }
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
