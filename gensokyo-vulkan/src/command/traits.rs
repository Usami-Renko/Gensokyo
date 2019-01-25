
use ash::vk;

use crate::pipeline::pass::GsRenderPass;

pub trait IntoVKBarrier {
    type BarrierType;

    fn into_barrier(self) -> Self::BarrierType;
}

pub trait CmdPipelineAbs {

    fn layout(&self)   -> &vk::PipelineLayout;
    fn pipeline(&self) -> &vk::Pipeline;
    fn render_pass(&self) -> &GsRenderPass;
}
