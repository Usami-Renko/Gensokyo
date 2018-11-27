
use pipeline::state::viewport::{ ViewportInfo, ScissorInfo };
use pipeline::state::depth_stencil::DepthBoundInfo;
use pipeline::state::rasterizer::DepthBiasInfo;

pub type CmdViewportInfo   = ViewportInfo;
pub type CmdScissorInfo    = ScissorInfo;
pub type CmdDepthBiasInfo  = DepthBiasInfo;
pub type CmdDepthBoundInfo = DepthBoundInfo;
