
use ash::vk;

use utils::types::vkformat;
use utils::marker::VulkanEnum;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct SurfaceFormat {

    pub format: vkformat,
    pub color_space: ColorSpace,
}

impl SurfaceFormat {

    pub fn to_vk_format(&self) -> vk::SurfaceFormatKHR {
        vk::SurfaceFormatKHR {
            format: self.format.value(),
            color_space: self.color_space.value(),
        }
    }
}

impl From<vk::SurfaceFormatKHR> for SurfaceFormat {

    fn from(raw: vk::SurfaceFormatKHR) -> SurfaceFormat {
        SurfaceFormat {
            format: From::from(raw.format),
            color_space: match raw.color_space {
                | vk::ColorSpaceKHR::SrgbNonlinear => ColorSpace::SrgbNonlinear,
            },
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ColorSpace {
    SrgbNonlinear,
}

impl VulkanEnum for ColorSpace {
    type EnumType = vk::ColorSpaceKHR;

    fn value(&self) -> Self::EnumType {
        match self {
            | ColorSpace::SrgbNonlinear => vk::ColorSpaceKHR::SrgbNonlinear,
        }
    }
}

impl From<vk::ColorSpaceKHR> for ColorSpace {

    fn from(raw: vk::ColorSpaceKHR) -> ColorSpace {
        match raw {
            | vk::ColorSpaceKHR::SrgbNonlinear => ColorSpace::SrgbNonlinear,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PresentMode {
    Immediate,
    Mailbox,
    Fifo,
    FifoRelaxed,
}

impl VulkanEnum for PresentMode {
    type EnumType = vk::PresentModeKHR;

    fn value(&self) -> Self::EnumType {
        match self {
            | PresentMode::Immediate   => vk::PresentModeKHR::Immediate,
            | PresentMode::Mailbox     => vk::PresentModeKHR::Mailbox,
            | PresentMode::Fifo        => vk::PresentModeKHR::Fifo,
            | PresentMode::FifoRelaxed => vk::PresentModeKHR::FifoRelaxed,
        }
    }
}

impl From<vk::PresentModeKHR> for PresentMode {

    fn from(mode: vk::PresentModeKHR) -> PresentMode {
        match mode {
            | vk::PresentModeKHR::Immediate   => PresentMode::Immediate,
            | vk::PresentModeKHR::Mailbox     => PresentMode::Mailbox,
            | vk::PresentModeKHR::Fifo        => PresentMode::Fifo,
            | vk::PresentModeKHR::FifoRelaxed => PresentMode::FifoRelaxed,
        }
    }
}
