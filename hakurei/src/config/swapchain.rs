
use ash::vk;
use ash::vk::uint32_t;

use utility::time::TimePeriod;

#[derive(Debug, Clone)]
pub struct SwapchainConfig {

    pub image_count: uint32_t,
    /// the value of layers property in vk::Framebuffer.
    pub framebuffer_layers: uint32_t,

    pub(crate) prefer_surface_format     : vk::Format,
    pub(crate) prefer_surface_color_space: vk::ColorSpaceKHR,

    pub(crate) prefer_primary_present_mode  : vk::PresentModeKHR,
    pub(crate) prefer_secondary_present_mode: vk::PresentModeKHR,

    pub acquire_image_time_out: TimePeriod,
}

impl Default for SwapchainConfig {

    fn default() -> SwapchainConfig {
        SwapchainConfig {
            image_count: 2,
            framebuffer_layers: 1,

            prefer_surface_format     : vk::Format::B8g8r8a8Unorm,
            prefer_surface_color_space: vk::ColorSpaceKHR::SrgbNonlinear,

            prefer_primary_present_mode  : vk::PresentModeKHR::Mailbox,
            prefer_secondary_present_mode: vk::PresentModeKHR::Fifo,

            acquire_image_time_out: TimePeriod::Infinte,
        }
    }
}
