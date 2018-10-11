
use ash::vk;
use ash::version::InstanceV1_0;

use config::engine::EngineConfig;
use core::instance::HaInstance;
use core::error::{ PhysicalDeviceError, PhysicalFormatUsage };

const DEPTH_STENCIL_REQUIRE_FLAG: vk::FormatFeatureFlags = vk::FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT;

pub struct PhysicalFormatProperties {

    pub(crate) depth_stencil_format: vk::Format,
}

impl PhysicalFormatProperties {

    pub fn inspect(instance: &HaInstance, physical_device: vk::PhysicalDevice, config: &EngineConfig) -> Result<PhysicalFormatProperties, PhysicalDeviceError> {

        let optimal_depth_stencil_format = check_depth_stencil_format(
            instance, physical_device,
            &config.pipeline.depth_stencil.prefer_depth_stencil_formats,
            config.pipeline.depth_stencil.prefer_image_tiling
        );

        let format_properties = PhysicalFormatProperties {

            depth_stencil_format: optimal_depth_stencil_format
                .ok_or(PhysicalDeviceError::FormatUsageNotSupport(PhysicalFormatUsage::DepthStencil))?,
        };

        Ok(format_properties)
    }
}

fn check_depth_stencil_format(instance: &HaInstance, physical_device: vk::PhysicalDevice, prefer_formats: &[vk::Format], prefer_image_tiling: vk::ImageTiling) -> Option<vk::Format> {

    prefer_formats.iter().find(|&candidate_format| {

        let format_properties = instance.handle.get_physical_device_format_properties(physical_device, *candidate_format);

        match prefer_image_tiling {
            | vk::ImageTiling::Optimal => {
                format_properties.optimal_tiling_features.subset(DEPTH_STENCIL_REQUIRE_FLAG)
            },
            | vk::ImageTiling::Linear => {
                format_properties.linear_tiling_features.subset(DEPTH_STENCIL_REQUIRE_FLAG)
            },
        }
    }).and_then(|format| Some(format.clone()))
}
