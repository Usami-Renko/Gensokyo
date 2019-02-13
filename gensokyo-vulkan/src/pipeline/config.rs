
use crate::types::format::Format;

pub struct PipelineConfig {

    pub depth_stencil: DepthStencilConfig,
}

pub struct DepthStencilConfig {

    /// The prefer format for depth or stencil buffer.
    ///
    /// Although this format can be specified in pipeline creation, it's recommended to specify the format in this config setting, because in this way the Gensokyo engine can help to check if this format is supported in the system.
    ///
    /// The pipeline will use the first format which support VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT in vkGetPhysicalDeviceFormatProperties call.
    pub prefer_depth_stencil_formats: Vec<Format>,
}
