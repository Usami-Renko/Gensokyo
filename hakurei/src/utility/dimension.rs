
use ash::vk;
use ash::vk::types::uint32_t;

use std::fmt;

#[derive(Clone, Copy)]
pub struct Dimension1D {
    pub size: uint32_t,
}

#[derive(Clone, Copy)]
pub struct Dimension2D {
    pub width:  uint32_t,
    pub height: uint32_t,
}

#[derive(Clone, Copy)]
pub struct Dimension3D {
    pub width:  uint32_t,
    pub height: uint32_t,
    pub depth:  uint32_t,
}

#[derive(Clone, Copy)]
pub struct BufferDimension {
    pub extent: vk::Extent2D,
    pub layers: uint32_t,
}

impl BufferDimension {

    pub fn init(extent: vk::Extent2D, layers: uint32_t) -> BufferDimension {
        BufferDimension {
            extent,
            layers,
        }
    }
}

impl fmt::Display for Dimension1D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "size: {}", self.size)
    }
}

impl fmt::Display for Dimension2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "width: {}, height: {}", self.width, self.height)
    }
}

impl fmt::Display for Dimension3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "width: {}, height: {}, depth: {}", self.width, self.height, self.depth)
    }
}

impl fmt::Display for BufferDimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "width: {}, height: {}, layers: {}", self.extent.width, self.extent.height, self.layers)
    }
}