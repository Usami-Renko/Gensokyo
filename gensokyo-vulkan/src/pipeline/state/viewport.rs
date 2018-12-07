
use ash::vk;

use crate::types::{ vkuint, vksint, vkfloat, vkDim2D };
use std::ptr;

pub struct GsViewportState {

    ports   : Vec<vk::Viewport>,
    scissors: Vec<vk::Rect2D>,
    /// manage the count of `ports` of `scissors` manually.
    length: usize,
}

impl GsViewportState {

    pub fn single(info: ViewportStateInfo) -> GsViewportState {

        GsViewportState {
            ports   : vec![info.viewport.content],
            scissors: vec![info.scissor.content],

            length: 1,
        }
    }

    pub fn multi(infos: Vec<ViewportStateInfo>) -> GsViewportState {

        let mut ports = vec![];
        let mut scissors = vec![];
        let length = infos.len();

        for info in infos.into_iter() {
            ports.push(info.viewport.content);
            scissors.push(info.scissor.content);
        }

        GsViewportState {
            ports, scissors, length,
        }
    }

    pub fn add_viewport(&mut self, viewport: ViewportStateInfo) {

        self.ports.push(viewport.viewport.content);
        self.scissors.push(viewport.scissor.content);
        self.length += 1;
    }

    pub(crate) fn info(&self) -> vk::PipelineViewportStateCreateInfo {

        vk::PipelineViewportStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags : vk::PipelineViewportStateCreateFlags::empty(),
            viewport_count: self.length as vkuint,
            p_viewports   : self.ports.as_ptr(),
            scissor_count : self.length as vkuint,
            p_scissors    : self.scissors.as_ptr(),
        }
    }
}

pub enum ViewportStateType {

    /// `Fixed` specifies that viewports and scissors is set to fixed value during the whole pipeline.
    ///
    /// `state` specifies all the information of fixed viewport and scissor properties.
    Fixed { state: GsViewportState },
    /// `Dynamic` specifies that viewports and scissors will be dynamically set in command buffer recording(`GsCommandRecorder::set_viewport` and `GsCommandRecorder::set_scissor`).
    /// And the count of viewports and scissors are required here.
    ///
    /// `count` specifies the count of viewport and scissors will be set in command buffer recording.
    Dynamic { count: usize },
    /// `DynamicViewportFixedScissor` specifies that viewport is set dynamically in command buffer recording(`GsCommandRecorder::set_viewport`) and the scissors is set to fixed value during the whole pipeline.
    ///
    /// `scissors` specifies all the information of fixed scissors. The length of `scissors` must be the same with viewport_count.
    ///
    /// The count of dynamic viewports will keep the same with the length of `scissors`.
    DynamicViewportFixedScissor { scissors: Vec<ScissorInfo> },
    /// `FixedViewportDynamicScissor` specifies that scissor is set dynamically in command buffer recording(`GsCommandRecorder::set_scissor`) and the viewports is set to fixed value during the whole pipeline.
    ///
    /// `viewports` specifies all the information of fixed viewports. The length of `viewports` must be the same with scissor_count.
    ///
    /// The count of dynamic scissors will keep the same with the length of `viewports`.
    FixedViewportDynamicScissor { viewports: Vec<ViewportInfo> },
}

impl ViewportStateType {

    pub(crate) fn into_viewport_state(self) -> GsViewportState {
        match self {
            | ViewportStateType::Fixed { state } => {
                state
            },
            | ViewportStateType::Dynamic { count } => {
                GsViewportState {
                    ports: vec![],
                    scissors: vec![],
                    length: count,
                }
            },
            | ViewportStateType::DynamicViewportFixedScissor { scissors } => {
                let length = scissors.len();
                GsViewportState {
                    ports: vec![],
                    scissors: scissors.into_iter().map(|s| s.content).collect(),
                    length,
                }
            },
            | ViewportStateType::FixedViewportDynamicScissor { viewports } => {
                let length = viewports.len();
                GsViewportState {
                    ports: viewports.into_iter().map(|v| v.content).collect(),
                    scissors: vec![],
                    length,
                }
            },
        }
    }
}

pub struct ViewportInfo {

    pub(crate) content: vk::Viewport,
}

impl ViewportInfo {

    pub fn new(dimension: vkDim2D) -> ViewportInfo {

        ViewportInfo {
            content: vk::Viewport {
                x: 0.0,
                y: 0.0,
                width : dimension.width  as vkfloat,
                height: dimension.height as vkfloat,
                min_depth: 0.0,
                max_depth: 1.0,
            }
        }
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
    pub fn set_detail(&mut self, x: vkfloat, y: vkfloat, width: vkfloat, height: vkfloat, min_depth: vkfloat, max_depth: vkfloat) {
        self.content = vk::Viewport {
            x, y, width, height, min_depth, max_depth,
        };
    }
}

pub struct ScissorInfo {

    pub(crate) content: vk::Rect2D,
}

impl ScissorInfo {

    pub fn new(dimension: vkDim2D) -> ScissorInfo {

        ScissorInfo {
            content: vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: vkDim2D {
                    width: dimension.width,
                    height: dimension.height
                }
            }
        }
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
    pub fn set_detail(&mut self, x: vksint, y: vksint, width: vkuint, height: vkuint) {
        self.content = vk::Rect2D {
            offset: vk::Offset2D { x, y },
            extent: vkDim2D { width, height },
        }
    }
}


pub struct ViewportStateInfo {

    pub viewport: ViewportInfo,
    pub scissor : ScissorInfo,
}

impl ViewportStateInfo {

    pub fn new(dimension: vkDim2D) -> ViewportStateInfo {

        ViewportStateInfo {
            viewport: ViewportInfo::new(dimension),
            scissor : ScissorInfo::new(dimension),
        }
    }
}
