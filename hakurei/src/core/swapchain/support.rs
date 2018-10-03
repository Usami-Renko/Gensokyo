
use ash::vk;
use ash::vk::uint32_t;

use num::clamp;

use config::engine::EngineConfig;
use config::core::SwapchainConfig;
use core::surface::HaSurface;
use core::error::SurfaceError;

pub struct SwapchainSupport {

    capabilities:  vk::SurfaceCapabilitiesKHR,
    formats:       Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,

    config: SwapchainConfig,
}

impl SwapchainSupport {

    pub fn query_support(surface: &HaSurface, physical: vk::PhysicalDevice, config: &EngineConfig) -> Result<SwapchainSupport, SurfaceError> {

        let support = SwapchainSupport {
            capabilities  : surface.capabilities(physical)?,
            formats       : surface.formats(physical)?,
            present_modes : surface.present_modes(physical)?,
            config        : config.core.swapchain.clone(),
        };
        Ok(support)
    }

    pub fn optimal_extent(&self, surface: &HaSurface) -> vk::Extent2D {

        const SPECIAL_EXTEND: uint32_t = 0xFFFF_FFFF;
        if self.capabilities.current_extent.width  == SPECIAL_EXTEND &&
            self.capabilities.current_extent.height == SPECIAL_EXTEND {

            let window_size = surface.window_size();

            vk::Extent2D {
                width: clamp(
                    window_size.width,
                    self.capabilities.min_image_extent.width,
                    self.capabilities.max_image_extent.width
                ),
                height: clamp(
                    window_size.height,
                    self.capabilities.min_image_extent.height,
                    self.capabilities.max_image_extent.height,
                )
            }

        } else {
            self.capabilities.current_extent
        }
    }

    // TODO: Make format preference configurable.
    pub fn optimal_format(&self) -> vk::SurfaceFormatKHR {

        if self.formats.len() == 1 && self.formats[0].format == vk::Format::Undefined {
            return vk::SurfaceFormatKHR {
                format      : self.config.prefer_surface_format,
                color_space : self.config.prefer_surface_color_space,
            }
        }

        for available_format in self.formats.iter() {
            if available_format.format == self.config.prefer_surface_format &&
                available_format.color_space == self.config.prefer_surface_color_space {

                return available_format.clone()
            }
        }

        self.formats.first().unwrap().clone()
    }

    // TODO: Make present mode preference configurable.
    pub fn optimal_present_mode(&self) -> vk::PresentModeKHR {

        if self.present_modes.iter().find(|&mode| *mode == self.config.prefer_primary_present_mode).is_some() {
            self.config.prefer_primary_present_mode
        } else if self.present_modes.iter().find(|&mode| *mode == self.config.prefer_secondary_present_mode).is_some() {
            self.config.prefer_secondary_present_mode
        } else {
            self.present_modes[0]
        }
    }

    pub fn current_transform(&self) -> vk::SurfaceTransformFlagsKHR {
        self.capabilities.current_transform
    }
}
