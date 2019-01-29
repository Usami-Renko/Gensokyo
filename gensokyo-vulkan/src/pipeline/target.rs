
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::GsDevice;

use crate::pipeline::layout::GsPipelineLayout;
use crate::pipeline::pass::GsRenderPass;

use crate::command::CmdPipelineAbs;
use crate::utils::phantom::{ Graphics, Compute };

use std::marker::PhantomData;
use std::ops::{ BitAnd, BitAndAssign, BitOr, BitOrAssign };


// -------------------------------------------------------------------------------------
pub trait GsVkPipelineType {
    const BIND_POINT: ash::vk::PipelineBindPoint;
}

impl GsVkPipelineType for Graphics {
    const BIND_POINT: vk::PipelineBindPoint = vk::PipelineBindPoint::GRAPHICS;
}

impl GsVkPipelineType for Compute {
    const BIND_POINT: vk::PipelineBindPoint = vk::PipelineBindPoint::COMPUTE;
}
// -------------------------------------------------------------------------------------

// -------------------------------------------------------------------------------------
pub struct GsPipeline<T>
    where
        T: GsVkPipelineType {

    phantom_type: PhantomData<T>,

    pub(crate) handle: vk::Pipeline,
    pub(crate) pass  : GsRenderPass,
    pub(crate) layout: GsPipelineLayout,

    device: GsDevice,
}

impl<T> GsPipeline<T>
    where
        T: GsVkPipelineType {

    pub(super) fn new(device: GsDevice, handle: vk::Pipeline, layout: vk::PipelineLayout, pass: GsRenderPass) -> GsPipeline<T> {

        GsPipeline {
            phantom_type: PhantomData,
            layout: GsPipelineLayout { handle: layout },
            device, handle, pass,
        }
    }

    pub fn frame_count(&self) -> usize {
        self.pass.frame_count()
    }
}

impl<T> Drop for GsPipeline<T>
    where
        T: GsVkPipelineType {

    fn drop(&mut self) {

        unsafe {
            self.device.logic.handle.destroy_pipeline(self.handle, None);
        }

        self.layout.destroy(&self.device);
        self.pass.destroy(&self.device);
    }
}

impl<T> CmdPipelineAbs for GsPipeline<T>
    where
        T: GsVkPipelineType {

    fn layout(&self)   -> &vk::PipelineLayout {
        &self.layout.handle
    }

    fn pipeline(&self) -> &vk::Pipeline {
        &self.handle
    }

    fn render_pass(&self) -> &GsRenderPass {
        &self.pass
    }
}
// -------------------------------------------------------------------------------------

// -------------------------------------------------------------------------------------
pub struct GsPipelineSet<T>
    where
        T: GsVkPipelineType {

    phantom_type: PhantomData<T>,

    pub(crate) handles: Vec<vk::Pipeline>,
    pub(crate) pass   : GsRenderPass,
    pub(crate) layout : GsPipelineLayout,

    device: GsDevice,
}

pub struct PipelineIndex(pub(crate) usize);

pub struct GsPipelineElement<'a> {
    pipeline: vk::Pipeline,
    layout  : &'a GsPipelineLayout,
    pass    : &'a GsRenderPass,
}

impl<T> GsPipelineSet<T>
    where
        T: GsVkPipelineType {

    pub(crate) fn new(device: GsDevice, handles: Vec<vk::Pipeline>, layout: vk::PipelineLayout, pass: GsRenderPass) -> GsPipelineSet<Graphics> {

        GsPipelineSet {
            phantom_type: PhantomData,
            layout: GsPipelineLayout { handle: layout },
            device, handles, pass,
        }
    }

    pub fn element(&self, at: &PipelineIndex) -> GsPipelineElement {

        GsPipelineElement {
            pipeline: self.handles[at.0],
            layout  : &self.layout,
            pass    : &self.pass,
        }
    }

    pub fn frame_count(&self) -> usize {
        self.pass.frame_count()
    }
}

impl<T> Drop for GsPipelineSet<T>
    where
        T: GsVkPipelineType {

    fn drop(&mut self) {

        for &handle in self.handles.iter() {
            unsafe {
                self.device.logic.handle.destroy_pipeline(handle, None);
            }
        }
        self.layout.destroy(&self.device);
        self.pass.destroy(&self.device);
    }
}

impl<'a> CmdPipelineAbs for GsPipelineElement<'a> {

    fn layout(&self) -> &vk::PipelineLayout {
        &self.layout.handle
    }

    fn pipeline(&self) -> &vk::Pipeline {
        &self.pipeline
    }

    fn render_pass(&self) -> &GsRenderPass {
        &self.pass
    }
}
// -------------------------------------------------------------------------------------

// -------------------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct GsPipelineStage(pub(crate) vk::ShaderStageFlags);

impl GsPipelineStage {
    pub const VERTEX                 : GsPipelineStage = GsPipelineStage(vk::ShaderStageFlags::VERTEX);
    pub const TESSELLATION_CONTROL   : GsPipelineStage = GsPipelineStage(vk::ShaderStageFlags::TESSELLATION_CONTROL);
    pub const TESSELLATION_EVALUATION: GsPipelineStage = GsPipelineStage(vk::ShaderStageFlags::TESSELLATION_EVALUATION);
    pub const GEOMETRY               : GsPipelineStage = GsPipelineStage(vk::ShaderStageFlags::GEOMETRY);
    pub const FRAGMENT               : GsPipelineStage = GsPipelineStage(vk::ShaderStageFlags::FRAGMENT);
    pub const COMPUTE                : GsPipelineStage = GsPipelineStage(vk::ShaderStageFlags::COMPUTE);
}

impl BitAnd for GsPipelineStage {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self {
        GsPipelineStage(self.0 & rhs.0)
    }
}

impl BitAndAssign for GsPipelineStage {

    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOr for GsPipelineStage {
    type Output = Self;

    // rhs is the "right-hand side" of the expression `a | b`
    fn bitor(self, rhs: Self) -> Self {
        GsPipelineStage(self.0 | rhs.0)
    }
}

impl BitOrAssign for GsPipelineStage {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}
// -------------------------------------------------------------------------------------
