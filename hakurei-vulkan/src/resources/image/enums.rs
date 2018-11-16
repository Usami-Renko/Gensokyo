
use ash::vk;

use resources::memory::HaMemoryType;
use utils::marker::VulkanEnum;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageStorageType {
    Device,
    Cached,
}

impl ImageStorageType {

    pub fn memory_type(&self) -> HaMemoryType {
        match self {
            | ImageStorageType::Cached  => HaMemoryType::CachedMemory,
            | ImageStorageType::Device  => HaMemoryType::DeviceMemory,
        }
    }
}


// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageType {

    #[allow(dead_code)]
    Type1d,
    Type2d,
    #[allow(dead_code)]
    Type3d,
}

impl VulkanEnum for ImageType {
    type EnumType = vk::ImageType;

    fn value(&self) -> Self::EnumType {
        match self {
            | ImageType::Type1d => vk::ImageType::Type1d,
            | ImageType::Type2d => vk::ImageType::Type2d,
            | ImageType::Type3d => vk::ImageType::Type3d,
        }
    }
}

// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageViewType {

    #[allow(dead_code)]
    Type1d,
    Type2d,
    #[allow(dead_code)]
    Type3d,
    #[allow(dead_code)]
    Cube,
    #[allow(dead_code)]
    Type1dArray,
    #[allow(dead_code)]
    Type2dArray,
    #[allow(dead_code)]
    CubeArray,
}

impl VulkanEnum for ImageViewType {
    type EnumType = vk::ImageViewType;

    fn value(&self) -> Self::EnumType {
        match self {
            | ImageViewType::Type1d      => vk::ImageViewType::Type1d,
            | ImageViewType::Type2d      => vk::ImageViewType::Type2d,
            | ImageViewType::Type3d      => vk::ImageViewType::Type3d,
            | ImageViewType::Cube        => vk::ImageViewType::Cube,
            | ImageViewType::Type1dArray => vk::ImageViewType::Type1dArray,
            | ImageViewType::Type2dArray => vk::ImageViewType::Type2dArray,
            | ImageViewType::CubeArray   => vk::ImageViewType::CubeArray,
        }
    }
}

// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ImageTiling {

    Linear,
    Optimal,
}

impl VulkanEnum for ImageTiling {
    type EnumType = vk::ImageTiling;

    fn value(&self) -> Self::EnumType {
        match self {
            | ImageTiling::Linear  => vk::ImageTiling::Linear,
            | ImageTiling::Optimal => vk::ImageTiling::Optimal,
        }
    }
}


// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Filter {

    /// Nearest specifies nearest filtering.
    Nearest,
    /// Linear specifies linear filtering.
    Linear,
}

impl VulkanEnum for Filter {
    type EnumType = vk::Filter;

    fn value(&self) -> Self::EnumType {
        match self {
            | Filter::Nearest => vk::Filter::Nearest,
            | Filter::Linear  => vk::Filter::Linear,
        }
    }
}


// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SamplerMipmapMode {

    /// `Nearest` specifies nearest filtering.
    Nearest,
    /// `Linear` specifies linear filtering.
    Linear,
}

impl VulkanEnum for SamplerMipmapMode {
    type EnumType = vk::SamplerMipmapMode;

    fn value(&self) -> Self::EnumType {
        match self {
            | SamplerMipmapMode::Nearest => vk::SamplerMipmapMode::Nearest,
            | SamplerMipmapMode::Linear  => vk::SamplerMipmapMode::Linear,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SamplerAddressMode {

    /// `Repeat` specifies that the repeat wrap mode will be used.
    Repeat,
    /// `MirroredRepeat` specifies that the mirrored repeat wrap mode will be used.
    MirroredRepeat,
    /// `ClampToEdge` specifies that the clamp to edge wrap mode will be used.
    ClampToEdge,
    /// `ClampToBorder` specifies that the clamp to border wrap mode will be used.
    ClampToBorder,
    // `MirrorClampToEdge` specifies that the mirror clamp to edge wrap mode will be used.
    //
    // This is only valid if the VK_KHR_sampler_mirror_clamp_to_edge extension is enabled.
    // MirrorClampToEdge,
}

impl VulkanEnum for SamplerAddressMode {
    type EnumType = vk::SamplerAddressMode;

    fn value(&self) -> Self::EnumType {
        match self {
            | SamplerAddressMode::Repeat         => vk::SamplerAddressMode::Repeat,
            | SamplerAddressMode::MirroredRepeat => vk::SamplerAddressMode::MirroredRepeat,
            | SamplerAddressMode::ClampToEdge    => vk::SamplerAddressMode::ClampToEdge,
            | SamplerAddressMode::ClampToBorder  => vk::SamplerAddressMode::ClampToBorder,
            // | SamplerAddressMode::MirrorClampToEdge => vk::SamplerAddressMode::MirrorClampToEdge,
        }
    }
}

// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CompareOp {

    /// `Never` specifies that the test never passes.
    Never,
    /// `Less` specifies that the test passes when R < S.
    Less,
    /// `Equal` specifies that the test passes when R = S.
    Equal,
    /// LessOrEqual specifies that the test passes when R ≤ S.
    LessOrEqual,
    /// Greater specifies that the test passes when R > S.
    Greater,
    /// NotEqual specifies that the test passes when R ≠ S.
    NotEqual,
    /// GreaterOrEqual specifies that the test passes when R ≥ S.
    GreaterOrEqual,
    /// Always specifies that the test always passes.
    Always,
}

impl VulkanEnum for CompareOp {
    type EnumType = vk::CompareOp;

    fn value(&self) -> Self::EnumType {
        match self {
            | CompareOp::Never          => vk::CompareOp::Never,
            | CompareOp::Less           => vk::CompareOp::Less,
            | CompareOp::Equal          => vk::CompareOp::Equal,
            | CompareOp::LessOrEqual    => vk::CompareOp::LessOrEqual,
            | CompareOp::Greater        => vk::CompareOp::Greater,
            | CompareOp::NotEqual       => vk::CompareOp::NotEqual,
            | CompareOp::GreaterOrEqual => vk::CompareOp::GreaterOrEqual,
            | CompareOp::Always         => vk::CompareOp::Always,
        }
    }
}


// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BorderColor {

    /// FloatTransparentBlack specifies a transparent, floating-point format, black color.
    FloatTransparentBlack,
    /// IntTransparentBlack specifies a transparent, integer format, black color.
    IntTransparentBlack,
    /// FloatOpaqueBlack specifies an opaque, floating-point format, black color.
    FloatOpaqueBlack,
    /// IntOpaqueBlack specifies an opaque, integer format, black color.
    IntOpaqueBlack,
    /// FloatOpaqueWhite specifies an opaque, floating-point format, white color.
    FloatOpaqueWhite,
    /// IntOpaqueWhite specifies an opaque, integer format, white color.
    IntOpaqueWhite,
}

impl VulkanEnum for BorderColor {
    type EnumType = vk::BorderColor;

    fn value(&self) -> Self::EnumType {
        match self {
            | BorderColor::FloatTransparentBlack => vk::BorderColor::FloatTransparentBlack,
            | BorderColor::IntTransparentBlack   => vk::BorderColor::IntTransparentBlack,
            | BorderColor::FloatOpaqueBlack      => vk::BorderColor::FloatOpaqueBlack,
            | BorderColor::IntOpaqueBlack        => vk::BorderColor::IntOpaqueBlack,
            | BorderColor::FloatOpaqueWhite      => vk::BorderColor::FloatOpaqueWhite,
            | BorderColor::IntOpaqueWhite        => vk::BorderColor::IntOpaqueWhite,
        }
    }
}

// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ComponentSwizzle {
    Identity, Zero, One,
    R, G, B, A,
}

impl VulkanEnum for ComponentSwizzle {
    type EnumType = vk::ComponentSwizzle;

    fn value(&self) -> Self::EnumType {
        match self {
            | ComponentSwizzle::Identity => vk::ComponentSwizzle::Identity,
            | ComponentSwizzle::Zero     => vk::ComponentSwizzle::Zero,
            | ComponentSwizzle::One      => vk::ComponentSwizzle::One,
            | ComponentSwizzle::R        => vk::ComponentSwizzle::R,
            | ComponentSwizzle::G        => vk::ComponentSwizzle::G,
            | ComponentSwizzle::B        => vk::ComponentSwizzle::B,
            | ComponentSwizzle::A        => vk::ComponentSwizzle::A,
        }
    }
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
        match self {
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
