
use ash::vk;

use utility::marker::VulkanEnum;

pub struct HaImage {

    pub(crate) handle: vk::Image,
}

// TODO: Map to raw value
// TODO: Some enum is not available in ash crate yet.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageLayout {
    /// Undefine does not support device access.
    ///
    /// This layout must only be used as the initialLayout member of VkImageCreateInfo or VkAttachmentDescription, or as the oldLayout in an image transition.
    ///
    /// When transitioning out of this layout, the contents of the memory are not guaranteed to be preserved.
    Undefined,
    /// General supports all types of device access.
    General,
    /// ColorAttachmentOptimal must only be used as a color or resolve attachment in a VkFramebuffer.
    /// This layout is valid only for image subresources of images created with the VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT usage bit enabled.
    ColorAttachmentOptimal,
    /// DepthStencilAttachmentOptimal must only be used as a depth/stencil attachment in a VkFramebuffer.
    ///
    /// This layout is valid only for image subresources of images created with the VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT usage bit enabled.
    DepthStencilAttachmentOptimal,
    /// DepthStencilReadOnlyOptimal must only be used as a read-only depth/stencil attachment in a VkFramebuffer and/or as a read-only image in a shader (which can be read as a sampled image, combined image/sampler and/or input attachment).
    ///
    /// This layout is valid only for image subresources of images created with the VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT usage bit enabled.
    ///
    /// Only image subresources of images created with VK_IMAGE_USAGE_SAMPLED_BIT can be used as a sampled image or combined image/sampler in a shader.
    ///
    /// Similarly, only image subresources of images created with VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT can be used as input attachments.
    DepthStencilReadOnlyOptimal,
    /// ShaderReadOnlyOptimal must only be used as a read-only image in a shader (which can be read as a sampled image, combined image/sampler and/or input attachment).
    ///
    /// This layout is valid only for image subresources of images created with the VK_IMAGE_USAGE_SAMPLED_BIT or VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT usage bit enabled.
    ShaderReadOnlyOptimal,
    /// TransferSrcOptimal must only be used as a source image of a transfer command (see the definition of VK_PIPELINE_STAGE_TRANSFER_BIT).
    ///
    /// This layout is valid only for image subresources of images created with the VK_IMAGE_USAGE_TRANSFER_SRC_BIT usage bit enabled.
    TransferSrcOptimal,
    /// must only be used as a destination image of a transfer command.
    ///
    /// This layout is valid only for image subresources of images created with the VK_IMAGE_USAGE_TRANSFER_DST_BIT usage bit enabled.
    TransferDstOptimal,
    /// Preinitialized does not support device access.
    ///
    /// This layout must only be used as the initialLayout member of VkImageCreateInfo or VkAttachmentDescription, or as the oldLayout in an image transition.
    ///
    /// When transitioning out of this layout, the contents of the memory are preserved.
    ///
    /// This layout is intended to be used as the initial layout for an image whose contents are written by the host, and hence the data can be written to memory immediately, without first executing a layout transition.
    ///
    /// Currently, VK_IMAGE_LAYOUT_PREINITIALIZED is only useful with VK_IMAGE_TILING_LINEAR images because there is not a standard layout defined for VK_IMAGE_TILING_OPTIMAL images.
    Preinitialized,
//    /// DepthReadOnlyStencilAttachmentOptimal must only be used as a depth/stencil attachment in a VkFramebuffer, where the depth aspect is read-only, and/or as a read-only image in a shader (which can be read as a sampled image, combined image/sampler and/or input attachment) where only the depth aspect is accessed.
//    ///
//    /// This layout is valid only for image subresources of images created with the VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT usage bit enabled.
//    ///
//    /// Only image subresources of images created with VK_IMAGE_USAGE_SAMPLED_BIT can be used as a sampled image or combined image/sampler in a shader.
//    ///
//    /// Similarly, only image subresources of images created with VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT can be used as input attachments.
//    DepthReadOnlyStencilAttachmentOptimal,
//    /// DepthAttachmentStencilReadOnlyOptimal must only be used as a depth/stencil attachment in a VkFramebuffer, where the stencil aspect is read-only,
//    /// and/or as a read-only image in a shader (which can be read as a sampled image, combined image/sampler and/or input attachment) where only the stencil aspect is accessed.
//    ///
//    /// This layout is valid only for image subresources of images created with the VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT usage bit enabled.
//    ///
//    /// Only image subresources of images created with VK_IMAGE_USAGE_SAMPLED_BIT can be used as a sampled image or combined image/sampler in a shader.
//    ///
//    /// Similarly, only image subresources of images created with VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT can be used as input attachments.
//    DepthAttachmentStencilReadOnlyOptimal,
    /// PresentSrcKHR must only be used for presenting a presentable image for display.
    ///
    /// A swapchain’s image must be transitioned to this layout before calling vkQueuePresentKHR, and must be transitioned away from this layout after calling vkAcquireNextImageKHR.
    PresentSrcKHR,
//    /// ShaderPresentKHR is valid only for shared presentable images, and must be used for any usage the image supports.
//    ShaderPresentKHR,
//    /// DepthReadOnlyStencilAttachmentOptimalKHR is same as DepthReadOnlyStencilAttachmentOptimal.
//    DepthReadOnlyStencilAttachmentOptimalKHR,
//    /// DepthAttachmentStencilReadOnlyOptimalKHR is same as DepthAttachmentStencilReadOnlyOptimal.
//    DepthAttachmentStencilReadOnlyOptimalKHR,
}

impl VulkanEnum for ImageLayout {
    type EnumType = vk::ImageLayout;

    fn value(&self) -> Self::EnumType {
        match *self {
            | ImageLayout::Undefined                                => vk::ImageLayout::Undefined,
            | ImageLayout::General                                  => vk::ImageLayout::General,
            | ImageLayout::ColorAttachmentOptimal                   => vk::ImageLayout::ColorAttachmentOptimal,
            | ImageLayout::DepthStencilAttachmentOptimal            => vk::ImageLayout::DepthStencilAttachmentOptimal,
            | ImageLayout::DepthStencilReadOnlyOptimal              => vk::ImageLayout::DepthStencilReadOnlyOptimal,
            | ImageLayout::ShaderReadOnlyOptimal                    => vk::ImageLayout::ShaderReadOnlyOptimal,
            | ImageLayout::TransferSrcOptimal                       => vk::ImageLayout::TransferSrcOptimal,
            | ImageLayout::TransferDstOptimal                       => vk::ImageLayout::TransferDstOptimal,
            | ImageLayout::Preinitialized                           => vk::ImageLayout::Preinitialized,
//            | ImageLayout::DepthReadOnlyStencilAttachmentOptimal    => vk::ImageLayout::DepthReadOnlyStencilAttachmentOptimal,
//            | ImageLayout::DepthAttachmentStencilReadOnlyOptimal    => vk::ImageLayout::DepthAttachmentStencilReadOnlyOptimal,
            | ImageLayout::PresentSrcKHR                            => vk::ImageLayout::PresentSrcKhr,
//            | ImageLayout::ShaderPresentKHR                         => vk::ImageLayout::ShaderPresentKHR,
//            | ImageLayout::DepthReadOnlyStencilAttachmentOptimalKHR => vk::ImageLayout::DepthReadOnlyStencilAttachmentOptimalKHR,
//            | ImageLayout::DepthAttachmentStencilReadOnlyOptimalKHR => vk::ImageLayout::DepthAttachmentStencilReadOnlyOptimalKHR,
        }
    }
}
