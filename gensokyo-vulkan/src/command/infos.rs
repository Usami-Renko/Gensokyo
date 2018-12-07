
use crate::pipeline::state::viewport::{ ViewportInfo, ScissorInfo };
use crate::pipeline::state::depth_stencil::DepthBoundInfo;
use crate::pipeline::state::rasterizer::DepthBiasInfo;

pub type CmdViewportInfo   = ViewportInfo;
pub type CmdScissorInfo    = ScissorInfo;
pub type CmdDepthBiasInfo  = DepthBiasInfo;
pub type CmdDepthBoundInfo = DepthBoundInfo;
