
use ash::vk;

pub struct DepthStencilConfig {

    /// The prefer format for depth or stencil buffer.
    ///
    /// Although this format can be specified in pipeline creation, it's recommended to specify the format in this config setting, because in this way the hakurei engine can help to check if this format is supported in the system.
    ///
    /// The pipeline will use the first format which support VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT in vkGetPhysicalDeviceFormatProperties call.
    pub prefer_depth_stencil_formats: Vec<vk::Format>,
    /// The prefer image tiling mode for depth or stencil buffer.
    pub prefer_image_tiling: vk::ImageTiling,
}

impl Default for DepthStencilConfig {

    fn default() -> DepthStencilConfig {

        DepthStencilConfig {

            prefer_depth_stencil_formats: vec![
                vk::Format::D32Sfloat,
                vk::Format::D32SfloatS8Uint,
                vk::Format::D24UnormS8Uint,
            ],
            prefer_image_tiling: vk::ImageTiling::Optimal,
        }
    }
}
