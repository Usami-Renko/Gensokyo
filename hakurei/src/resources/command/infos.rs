
use pipeline::state::{ ViewportInfo, ScissorInfo, DepthBiasInfo, DepthBoundInfo };
use resources::buffer::{ HaVertexBlock, HaIndexBlock };

pub struct CmdVertexBindingInfo<'a> {

    pub block: &'a HaVertexBlock,
    pub sub_block_index: Option<usize>,
}

pub struct CmdIndexBindingInfo<'a> {

    pub block: &'a HaIndexBlock,
    pub sub_block_index: Option<usize>,
}

pub type CmdViewportInfo   = ViewportInfo;
pub type CmdScissorInfo    = ScissorInfo;
pub type CmdDepthBiasInfo  = DepthBiasInfo;
pub type CmdDepthBoundInfo = DepthBoundInfo;
