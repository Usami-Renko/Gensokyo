
use ash::vk;
use ash::vk::uint32_t;

use num::clamp;

use core::surface::HaSurface;
use core::error::SurfaceError;

use constant::swapchain::{ PREFER_SURFACE_FORMAT, PREFER_SURFACE_COLOR_SPACE, PREFER_PRESENT_MODE };

pub struct SwapchainSupport {

    capabilities:  vk::SurfaceCapabilitiesKHR,
    formats:       Vec<vk::SurfaceFormatKHR>,
    present_modes: Vec<vk::PresentModeKHR>,
}

impl SwapchainSupport {

    pub fn query_support(surface: &HaSurface, physical: vk::PhysicalDevice) -> Result<SwapchainSupport, SurfaceError> {
        Ok(SwapchainSupport {
            capabilities  : surface.capabilities(physical)?,
            formats       : surface.formats(physical)?,
            present_modes : surface.present_modes(physical)?,
        })
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
                format      : PREFER_SURFACE_FORMAT,
                color_space : PREFER_SURFACE_COLOR_SPACE,
            }
        }

        for available_format in self.formats.iter() {
            if available_format.format == PREFER_SURFACE_FORMAT &&
                available_format.color_space == PREFER_SURFACE_COLOR_SPACE {

                return available_format.clone()
            }
        }

        self.formats.first().unwrap().clone()
    }

    // TODO: Make present mode preference configurable.
    pub fn optimal_present_mode(&self) -> vk::PresentModeKHR {

        let mut best_mode = PREFER_PRESENT_MODE;

        for &available_mode in self.present_modes.iter() {
            match available_mode {
                | vk::PresentModeKHR::Mailbox => return available_mode,
                | vk::PresentModeKHR::Immediate => best_mode = available_mode,
                | _ => ()
            }
        }

        best_mode
    }

    pub fn current_transform(&self) -> vk::SurfaceTransformFlagsKHR {
        self.capabilities.current_transform
    }
}