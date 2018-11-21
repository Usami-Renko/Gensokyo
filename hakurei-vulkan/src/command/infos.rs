
use pipeline::state::viewport::{ ViewportInfo, ScissorInfo };
use pipeline::state::depth_stencil::DepthBoundInfo;
use pipeline::state::rasterizer::DepthBiasInfo;

use buffer::BufferInstance;

pub struct CmdBufferBindingInfo<'a> {

    pub block: &'a BufferInstance,
    pub sub_block_index: Option<usize>,
}

pub type CmdViewportInfo   = ViewportInfo;
pub type CmdScissorInfo    = ScissorInfo;
pub type CmdDepthBiasInfo  = DepthBiasInfo;
pub type CmdDepthBoundInfo = DepthBoundInfo;
