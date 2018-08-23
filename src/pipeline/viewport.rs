
use ash::vk;
use ash::vk::uint32_t;

use std::os::raw::c_float;
use std::ptr;

pub struct HaViewport {

    handles  : Vec<vk::Viewport>,
    scissors : Vec<vk::Rect2D>,
}

impl HaViewport {

    pub fn init() -> HaViewport {
        HaViewport {
            handles:  vec![],
            scissors: vec![],
        }
    }

    pub fn setup(extent: vk::Extent2D) -> HaViewport {

        let handles = [
            vk::Viewport {
                x: 0.0, y: 0.0,
                width:  extent.width  as c_float,
                height: extent.height as c_float,
                min_depth: 0.0,
                max_depth: 0.0,
            },
        ];

        let scissors = [
            vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent,
            }
        ];

        HaViewport { handles, scissors, }
    }

    pub fn add_viewport(&mut self, viewport: vk::Viewport) {
        self.handles.push(viewport);
    }
    pub fn add_scissor(&mut self, scissor: vk::Rect2D) {
        self.scissors.push(scissor);
    }


    pub fn info(&self) -> vk::PipelineViewportStateCreateInfo {
        vk::PipelineViewportStateCreateInfo {
            s_type : vk::StructureType::PipelineViewportStateCreateInfo,
            p_next : ptr::null(),
            // flags is reserved for future use in API version 1.0.82.
            flags  : vk::PipelineViewportStateCreateFlags::empty(),
            viewport_count : self.handles.len() as uint32_t,
            p_viewports    : self.handles.as_ptr(),
            scissor_count  : self.scissors.len() as uint32_t,
            p_scissors     : self.scissors.as_ptr(),
        }
    }
}
