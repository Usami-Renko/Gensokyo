
use crate::pipeline::state::viewport::{ ViewportInfo, ScissorInfo };
use crate::pipeline::state::depth_stencil::DepthBoundInfo;
use crate::pipeline::state::rasterizer::DepthBiasInfo;

use crate::descriptor::DescriptorSet;
use crate::types::vkuint;

pub type CmdViewportInfo   = ViewportInfo;
pub type CmdScissorInfo    = ScissorInfo;
pub type CmdDepthBiasInfo  = DepthBiasInfo;
pub type CmdDepthBoundInfo = DepthBoundInfo;

pub struct CmdDescriptorSetBindInfo<'a> {

    pub set: &'a DescriptorSet,
    pub dynamic_offsets: Option<&'a [vkuint]>
}
