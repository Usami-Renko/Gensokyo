
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
            ports   : vec![info.viewport.0],
            scissors: vec![info.scissor.0],

            length: 1,
        }
    }

    pub fn multi(infos: Vec<ViewportStateInfo>) -> GsViewportState {

        let mut ports = vec![];
        let mut scissors = vec![];
        let length = infos.len();

        for info in infos.into_iter() {
            ports.push(info.viewport.0);
            scissors.push(info.scissor.0);
        }

        GsViewportState {
            ports, scissors, length,
        }
    }

    pub fn add_viewport(&mut self, viewport: ViewportStateInfo) {

        self.ports.push(viewport.viewport.0);
        self.scissors.push(viewport.scissor.0);
        self.length += 1;
    }

    pub(crate) fn ci(&self) -> vk::PipelineViewportStateCreateInfo {

        vk::PipelineViewportStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags : vk::PipelineViewportStateCreateFlags::empty(),
            viewport_count: self.length as _,
            p_viewports   : self.ports.as_ptr(),
            scissor_count : self.length as _,
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

impl From<ViewportStateType> for GsViewportState {

    fn from(raw: ViewportStateType) -> GsViewportState {
        match raw {
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
                    scissors: scissors.into_iter().map(|s| s.0).collect(),
                    length,
                }
            },
            | ViewportStateType::FixedViewportDynamicScissor { viewports } => {
                let length = viewports.len();
                GsViewportState {
                    ports: viewports.into_iter().map(|v| v.0).collect(),
                    scissors: vec![],
                    length,
                }
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct ViewportInfo(pub(crate) vk::Viewport);

impl From<vkDim2D> for ViewportInfo {

    fn from(dimension: vkDim2D) -> ViewportInfo {
        let content = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width : dimension.width  as _,
            height: dimension.height as _,
            min_depth: 0.0,
            max_depth: 1.0,
        };
        ViewportInfo(content)
    }
}

impl ViewportInfo {

    pub fn new(x: vkuint, y: vkuint, width: vkuint, height: vkuint) -> ViewportInfo {

        let content = vk::Viewport {
            x: x as _,
            y: y as _,
            width: width as _,
            height: height as _,
            min_depth: 0.0,
            max_depth: 1.0,
        };
        ViewportInfo(content)
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
    pub fn set_detail(&mut self, x: vkuint, y: vkuint, width: vkuint, height: vkuint, min_depth: vkfloat, max_depth: vkfloat) {
        self.0 = vk::Viewport {
            x: x as _,
            y: y as _,
            width: width as _,
            height: height as _,
            min_depth, max_depth,
        };
    }
}

#[derive(Debug, Clone)]
pub struct ScissorInfo(pub(crate) vk::Rect2D);

impl From<vkDim2D> for ScissorInfo {

    fn from(dimension: vkDim2D) -> ScissorInfo {

        let content = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent: vkDim2D {
                width: dimension.width,
                height: dimension.height,
            }
        };
        ScissorInfo(content)
    }
}

impl ScissorInfo {

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
        self.0 = vk::Rect2D {
            offset: vk::Offset2D { x, y },
            extent: vkDim2D { width, height },
        }
    }
}


pub struct ViewportStateInfo {

    pub viewport: ViewportInfo,
    pub scissor : ScissorInfo,
}

impl From<vkDim2D> for ViewportStateInfo {

    fn from(dimension: vkDim2D) -> ViewportStateInfo {
        ViewportStateInfo {
            viewport: ViewportInfo::from(dimension),
            scissor : ScissorInfo::from(dimension),
        }
    }
}
