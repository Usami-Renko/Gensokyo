
use ash::vk::uint32_t;

use core::ValidationInfo;
use core::debug::DebugReportFlag;

use core::physical::DeviceExtensionType;

use utility::time::TimePeriod;

pub const APPLICATION_VERSION: uint32_t = vk_make_version!(1, 0, 0);
pub const ENGINE_VERSION:      uint32_t = vk_make_version!(1, 0, 0);
pub const API_VERSION:         uint32_t = vk_make_version!(1, 0, 82);

pub const APPLICATION_NAME: &'static str = "Hakurei Program";
pub const ENGINE_NAME:      &'static str = "Hakurei Rendering Engine";

pub const DEVICE_EXTENSION: [DeviceExtensionType; 1] = [
    DeviceExtensionType::Swapchain,
];

pub struct CoreConfig {

    pub validation: ValidationInfo,

    pub transfer_wait_time: TimePeriod,
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

            transfer_wait_time: TimePeriod::Infinte,
        }
    }
}
