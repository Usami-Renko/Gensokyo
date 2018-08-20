

use ash::vk::uint32_t;

use core::ValidationInfo;
use core::debug::DebugReportFlags;

use core::physical::DeviceExtensionType;

pub const APPLICATION_VERSION: uint32_t = vk_make_version!(1, 0, 0);
pub const ENGINE_VERSION:      uint32_t = vk_make_version!(1, 0, 0);
pub const API_VERSION:         uint32_t = vk_make_version!(1, 0, 82);

pub const APPLICATION_NAME: &'static str = "Hakurei Program";
pub const ENGINE_NAME:      &'static str = "Hakurei Rendering Engine";

pub const VALIDATION: ValidationInfo = ValidationInfo {
    is_enable: true,
    required_validation_layers: [
        "VK_LAYER_LUNARG_standard_validation",
    ],
};

pub const VALIDATION_FLAGS: [DebugReportFlags; 4] = [
    DebugReportFlags::ErrorBit,
    DebugReportFlags::InformationBit,
    DebugReportFlags::WarningBit,
    DebugReportFlags::PerformanceWarningBit,
];

pub const DEVICE_EXTENSION: [DeviceExtensionType; 1] = [
    DeviceExtensionType::Swapchain,
];
