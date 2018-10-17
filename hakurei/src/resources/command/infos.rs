
use ash::vk;
use ash::vk::{ uint32_t, int32_t };

use utility::dimension::Dimension2D;

use std::os::raw::c_float;

pub struct CmdVertexBindingInfos {

    pub(crate) handles: Vec<vk::Buffer>,
    pub(crate) offsets: Vec<vk::DeviceSize>,
}

pub struct CmdIndexBindingInfo {

    pub(crate) handle: vk::Buffer,
    pub(crate) offset: vk::DeviceSize,
}

pub struct CmdDescriptorBindingInfos {

    pub(crate) handles: Vec<vk::DescriptorSet>,
}

pub struct CmdViewportInfo {

    pub(crate) viewport: vk::Viewport,
}

impl CmdViewportInfo {

    pub fn new(dimension: Dimension2D) -> CmdViewportInfo {

        let viewport = vk::Viewport {
            x: 0.0, y: 0.0,
            min_depth: 0.0, max_depth: 1.0,
            width : dimension.width  as c_float,
            height: dimension.height as c_float,
        };

        CmdViewportInfo { viewport }
    }

    pub fn detail(x: c_float, y: c_float, width: c_float, height: c_float, min_depth: c_float, max_depth: c_float) -> CmdViewportInfo {

        CmdViewportInfo {
            viewport: vk::Viewport {
                x, y, width, height, min_depth, max_depth,
            }
        }
    }
}

pub struct CmdScissorInfo {

    pub(crate) scissor: vk::Rect2D,
}

impl CmdScissorInfo {

    pub fn new(dimension: Dimension2D) -> CmdScissorInfo {

        let scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: vk::Extent2D {
                width : dimension.width,
                height: dimension.height,
            },
        };

        CmdScissorInfo { scissor }
    }

    pub fn detail(x: int32_t, y: int32_t, width: uint32_t, height: uint32_t) -> CmdScissorInfo {

        CmdScissorInfo {
            scissor: vk::Rect2D {
                offset: vk::Offset2D { x, y },
                extent: vk::Extent2D { width, height },
            }
        }
    }
}
