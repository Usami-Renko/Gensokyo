
use ash::vk;

use pipeline::state::{ ViewportInfo, ScissorInfo, DepthBiasInfo, DepthBoundInfo };

pub struct CmdDescriptorBindingInfos {

    pub(crate) handles: Vec<vk::DescriptorSet>,
}

pub type CmdViewportInfo   = ViewportInfo;
pub type CmdScissorInfo    = ScissorInfo;
pub type CmdDepthBiasInfo  = DepthBiasInfo;
pub type CmdDepthBoundInfo = DepthBoundInfo;
