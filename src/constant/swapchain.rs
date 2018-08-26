
use ash::vk;
use ash::vk::uint32_t;

use structures::time::TimePeriod;

pub const SWAPCHAIN_IMAGE_COUNT: uint32_t = 2;
/// the value of layers property in vk::Framebuffer.
pub const FRAMEBUFFER_LAYERS: uint32_t = 1;

pub const PREFER_SURFACE_FORMAT: vk::Format = vk::Format::B8g8r8a8Unorm;
pub const PREFER_SURFACE_COLOR_SPACE: vk::ColorSpaceKHR = vk::ColorSpaceKHR::SrgbNonlinear;
pub const PREFER_PRESENT_MODE: vk::PresentModeKHR = vk::PresentModeKHR::Fifo;

pub const ACQUIRE_IMAGE_TIME_OUT: TimePeriod = TimePeriod::Infinte;
