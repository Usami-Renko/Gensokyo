

use ash::vk::uint32_t;
use core::ValidationInfo;

pub const APPLICATION_VERSION: uint32_t = vk_make_version!(1, 0, 0);
pub const ENGINE_VERSION:      uint32_t = vk_make_version!(1, 0, 0);
pub const API_VERSION:         uint32_t = vk_make_version!(1, 0, 82);

pub const APPLICATION_NAME: &'static str = "Hakurei Program";
pub const ENGINE_NAME:      &'static str = "Hakurei Rendering Engine";

pub const VALIDATION: ValidationInfo = ValidationInfo {
    is_enable: false,
    required_validation_layers: [
        "VK_LAYER_LUNARG_standard_validation",
    ],
};
