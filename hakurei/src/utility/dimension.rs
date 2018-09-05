
use ash::vk;
use ash::vk::types::uint32_t;

use std::fmt;

pub type Dimension2D = vk::Extent2D;
pub type Dimension3D = vk::Extent3D;

#[derive(Clone, Copy)]
pub struct BufferDimension {
    pub extent: vk::Extent2D,
    pub layers: uint32_t,
}

impl BufferDimension {

    pub fn new(extent: vk::Extent2D, layers: uint32_t) -> BufferDimension {
        BufferDimension {
            extent,
            layers,
        }
    }
}

impl fmt::Display for BufferDimension {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "width: {}, height: {}, layers: {}", self.extent.width, self.extent.height, self.layers)
    }
}


