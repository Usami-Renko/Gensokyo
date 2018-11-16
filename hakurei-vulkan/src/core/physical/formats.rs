
use ash::vk;
use ash::version::InstanceV1_0;

use core::instance::HaInstance;
use core::error::{ PhysicalDeviceError, PhysicalFormatUsage };

use pipeline::config::DepthStencilConfig;
use resources::image::ImageTiling;

use utils::types::vkformat;
use utils::marker::VulkanEnum;

const DEPTH_STENCIL_REQUIRE_FLAG: vk::FormatFeatureFlags = vk::FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT;

pub struct PhysicalFormatProperties {

    pub depth_attachment_format: vkformat,
}

impl PhysicalFormatProperties {

    pub(crate) fn inspect(instance: &HaInstance, physical_device: vk::PhysicalDevice, config: &DepthStencilConfig) -> Result<PhysicalFormatProperties, PhysicalDeviceError> {

        let optimal_depth_stencil_format = check_depth_stencil_format(
            instance, physical_device,
            &config.prefer_depth_stencil_formats,
            config.prefer_image_tiling
        );

        let format_properties = PhysicalFormatProperties {

            depth_attachment_format: optimal_depth_stencil_format
                .ok_or(PhysicalDeviceError::FormatUsageNotSupport(PhysicalFormatUsage::DepthStencil))?,
        };

        Ok(format_properties)
    }
}

fn check_depth_stencil_format(instance: &HaInstance, physical_device: vk::PhysicalDevice, prefer_formats: &[vkformat], prefer_image_tiling: ImageTiling) -> Option<vkformat> {

    prefer_formats.iter().find(|&candidate_format| {

        let format_properties = instance.handle.get_physical_device_format_properties(physical_device, candidate_format.value());

        match prefer_image_tiling {
            | ImageTiling::Optimal => {
                format_properties.optimal_tiling_features.subset(DEPTH_STENCIL_REQUIRE_FLAG)
            },
            | ImageTiling::Linear => {
                format_properties.linear_tiling_features.subset(DEPTH_STENCIL_REQUIRE_FLAG)
            },
        }
    }).and_then(|format| Some(format.clone()))
}
