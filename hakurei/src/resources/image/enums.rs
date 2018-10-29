
use ash::vk;

use core::physical::HaPhysicalDevice;
use pipeline::stages::PipelineStageFlag;
use utility::marker::VulkanEnum;

/// ImagePipelineStage indicate in which pipeline stage this image is used.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ImagePipelineStage {
    VertexStage,
    FragmentStage,
}

impl ImagePipelineStage {

    pub(crate) fn to_stage_flag(&self) -> PipelineStageFlag {
        match self {
            | ImagePipelineStage::VertexStage   => PipelineStageFlag::VertexShaderBit,
            | ImagePipelineStage::FragmentStage => PipelineStageFlag::FragmentShaderBit,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum ImageBranchType {
    SampleImage(ImagePipelineStage),
    DepthStencilImage(DepthImageUsage),
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) enum DepthImageUsage {
    Attachment,
    ShaderRead(DepthStencilImageFormat, ImagePipelineStage),
}

impl DepthImageUsage {

    pub(crate) fn dst_stage_flag(&self) -> PipelineStageFlag {
        match self {
            | DepthImageUsage::Attachment => PipelineStageFlag::EarlyFragmentTestsBit,
            | DepthImageUsage::ShaderRead(_format, pipeline_stage) => pipeline_stage.to_stage_flag(),
        }
    }

    pub(crate) fn dst_format(&self, physical: &HaPhysicalDevice) -> vk::Format {
        match self {
            | DepthImageUsage::Attachment => physical.formats.depth_stencil_format,
            | DepthImageUsage::ShaderRead(format, _pipeline_stage) => format.to_vk_format(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum DepthStencilImageFormat {
    Depth32Bit,
    Depth32BitStencil8Bit,
    Depth24BitStencil8Bit,
}

impl DepthStencilImageFormat {

    pub(crate) fn to_vk_format(&self) -> vk::Format {
        match self {
            | DepthStencilImageFormat::Depth32Bit => vk::Format::D32Sfloat,
            | DepthStencilImageFormat::Depth24BitStencil8Bit => vk::Format::D24UnormS8Uint,
            | DepthStencilImageFormat::Depth32BitStencil8Bit => vk::Format::D32SfloatS8Uint,
        }
    }
}

// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum ImageType {
    #[allow(dead_code)]
    Type1d,
    Type2d,
    #[allow(dead_code)]
    Type3d,
}

impl VulkanEnum for ImageType {
    type EnumType = vk::ImageType;

    fn value(&self) -> Self::EnumType {
        match *self {
            | ImageType::Type1d => vk::ImageType::Type1d,
            | ImageType::Type2d => vk::ImageType::Type2d,
            | ImageType::Type3d => vk::ImageType::Type3d,
        }
    }
}

// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum ImageViewType {
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
        match *self {
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
        match *self {
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
        match *self {
            | Filter::Nearest => vk::Filter::Nearest,
            | Filter::Linear  => vk::Filter::Linear,
        }
    }
}


// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum MipmapMode {
    /// Nearest specifies nearest filtering.
    Nearest,
    /// Linear specifies linear filtering.
    Linear,
}

impl VulkanEnum for MipmapMode {
    type EnumType = vk::SamplerMipmapMode;

    fn value(&self) -> Self::EnumType {
        match *self {
            | MipmapMode::Nearest => vk::SamplerMipmapMode::Nearest,
            | MipmapMode::Linear  => vk::SamplerMipmapMode::Linear,
        }
    }
}

// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CompareOp {
    /// Never specifies that the test never passes.
    Never,
    /// Less specifies that the test passes when R < S.
    Less,
    /// Equal specifies that the test passes when R = S.
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
        match *self {
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
        match *self {
            | BorderColor::FloatTransparentBlack => vk::BorderColor::FloatTransparentBlack,
            | BorderColor::IntTransparentBlack   => vk::BorderColor::IntTransparentBlack,
            | BorderColor::FloatOpaqueBlack      => vk::BorderColor::FloatOpaqueBlack,
            | BorderColor::IntOpaqueBlack        => vk::BorderColor::IntOpaqueBlack,
            | BorderColor::FloatOpaqueWhite      => vk::BorderColor::FloatOpaqueWhite,
            | BorderColor::IntOpaqueWhite        => vk::BorderColor::IntOpaqueWhite,
        }
    }
}
