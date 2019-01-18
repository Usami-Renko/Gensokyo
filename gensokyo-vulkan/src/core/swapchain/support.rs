
use ash::vk;

use num::clamp;

use crate::core::surface::GsSurface;
use crate::core::swapchain::SwapchainConfig;
use crate::types::{ vkDim2D, vkuint };
use crate::error::VkResult;

pub struct SwapchainSupport {

    capabilities : vk::SurfaceCapabilitiesKHR,
    formats      : Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,

    config: SwapchainConfig,
}

impl SwapchainSupport {

    pub fn query_support(surface: &GsSurface, physical: vk::PhysicalDevice, config: &SwapchainConfig) -> VkResult<SwapchainSupport> {

        let support = SwapchainSupport {
            capabilities  : surface.query_capabilities(physical)?,
            formats       : surface.query_formats(physical)?,
            present_modes : surface.query_present_modes(physical)?,
            config        : config.clone(),
        };
        Ok(support)
    }

    pub fn optimal_extent(&self, window_size: &vkDim2D) -> VkResult<vkDim2D> {

        const SPECIAL_EXTEND: vkuint = 0xFFFF_FFFF;

        let optimal_extent = if self.capabilities.current_extent.width == SPECIAL_EXTEND &&
            self.capabilities.current_extent.height == SPECIAL_EXTEND {

            vkDim2D {
                width: clamp(
                    window_size.width as _,
                    self.capabilities.min_image_extent.width,
                    self.capabilities.max_image_extent.width
                ),
                height: clamp(
                    window_size.height as _,
                    self.capabilities.min_image_extent.height,
                    self.capabilities.max_image_extent.height,
                )
            }

        } else {
            self.capabilities.current_extent
        };

        Ok(optimal_extent)
    }

    pub fn optimal_format(&self) -> vk::SurfaceFormatKHR {

        if self.formats.len() == 1 && self.formats[0].format == vk::Format::UNDEFINED {
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

        self.formats[0]
    }

    pub fn optimal_present_mode(&self) -> vk::PresentModeKHR {

        if self.present_modes.contains(&self.config.prefer_primary_present_mode) {
            self.config.prefer_primary_present_mode
        } else if self.present_modes.contains(&self.config.prefer_secondary_present_mode) {
            self.config.prefer_secondary_present_mode
        } else {
            self.present_modes[0]
        }
    }

    pub fn current_transform(&self) -> vk::SurfaceTransformFlagsKHR {

        self.capabilities.current_transform
    }
}
