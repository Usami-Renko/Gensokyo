
use ash::vk;
use ash::vk::{ uint32_t, int32_t };

use std::os::raw::c_float;
use std::ptr;

use utility::dimension::Dimension2D;

pub struct HaViewport {

    ports   : Vec<vk::Viewport>,
    scissors: Vec<vk::Rect2D>,
}

impl HaViewport {

    pub fn single(info: ViewportInfo) -> HaViewport {

        HaViewport {
            ports   : vec![info.port],
            scissors: vec![info.scissor],
        }
    }

    pub fn multi(infos: Vec<ViewportInfo>) -> HaViewport {

        let mut ports = vec![];
        let mut scissors = vec![];

        for info in infos.into_iter() {
            ports.push(info.port);
            scissors.push(info.scissor);
        }

        HaViewport { ports, scissors }
    }

    pub fn add_viewport(&mut self, viewport: ViewportInfo) {
        self.ports.push(viewport.port);
        self.scissors.push(viewport.scissor);
    }

    pub(crate) fn info(&self) -> vk::PipelineViewportStateCreateInfo {
        vk::PipelineViewportStateCreateInfo {
            s_type: vk::StructureType::PipelineViewportStateCreateInfo,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags : vk::PipelineViewportStateCreateFlags::empty(),
            viewport_count: self.ports.len() as uint32_t,
            p_viewports   : self.ports.as_ptr(),
            scissor_count : self.scissors.len() as uint32_t,
            p_scissors    : self.scissors.as_ptr(),
        }
    }
}

impl Default for HaViewport {

    fn default() -> HaViewport {
        HaViewport {
            ports   : vec![],
            scissors: vec![],
        }
    }
}

pub struct ViewportInfo {

    port   : vk::Viewport,
    scissor: vk::Rect2D,
}

impl ViewportInfo {

    pub fn new(dimension: Dimension2D) -> ViewportInfo {

        let port = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width : dimension.width  as c_float,
            height: dimension.height as c_float,
            min_depth: 0.0,
            max_depth: 1.0,
        };

        let scissor = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: vk::Extent2D { width: dimension.width, height: dimension.height }
        };

        ViewportInfo { port, scissor }
    }

    /// Set the all the detail of viewport information.
    ///
    /// `x` is x offset from the viewport’s upper left corner(0, 0).
    ///
    /// `y` is y offset from the viewport’s upper left corner(0, 0).
    ///
    /// `width` is the width of viewport.
    ///
    /// `height` is the height of viewport.
    ///
    /// `min_depth` is minimum depth value for the viewport.
    ///
    /// `max_depth` is maximum depth value for the viewport.
    pub fn set_viewport(&mut self, x: c_float, y: c_float, width: c_float, height: c_float, min_depth: c_float, max_depth: c_float) {
        self.port = vk::Viewport {
            x, y, width, height, min_depth, max_depth,
        };
    }

    /// Set all the detail of scissor information.
    ///
    /// `x` is x offset from the upper left corner for scissor area.
    ///
    /// `y` is y offset from the upper left corner for scissor area.
    ///
    /// `width` is the width of scissor area.
    ///
    /// `height` is the width of scissor area.
    pub fn set_scissor(&mut self, x: int32_t, y: int32_t, width: uint32_t, height: uint32_t) {
        self.scissor = vk::Rect2D {
            offset: vk::Offset2D { x, y },
            extent: vk::Extent2D { width, height },
        }
    }
}
