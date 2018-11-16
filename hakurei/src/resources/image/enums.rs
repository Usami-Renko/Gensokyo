
use vk::core::physical::HaPhyDevice;
use vk::pipeline::stages::PipelineStageFlag;
use vk::utils::types::vkformat;
use vk::utils::format::VKFormat;

/// ImagePipelineStage indicate in which pipeline stage this image is used.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ImagePipelineStage {

    VertexStage,
    FragmentStage,
}

impl ImagePipelineStage {

    pub(super) fn to_stage_flag(&self) -> PipelineStageFlag {
        match self {
            | ImagePipelineStage::VertexStage   => PipelineStageFlag::VertexShaderBit,
            | ImagePipelineStage::FragmentStage => PipelineStageFlag::FragmentShaderBit,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ImageBranchType {

    SampleImage(ImagePipelineStage),
    DepthStencilAttachment,
    DepthStencilImage(vkformat, ImagePipelineStage),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum DepthStencilImageFormat {

    Depth32Bit,
    Depth32BitStencil8Bit,
    Depth24BitStencil8Bit,
}

impl DepthStencilImageFormat {

    fn to_vk_format(&self) -> vkformat {
        match self {
            | DepthStencilImageFormat::Depth32Bit => VKFormat::D32Sfloat,
            | DepthStencilImageFormat::Depth24BitStencil8Bit => VKFormat::D24UnormS8Uint,
            | DepthStencilImageFormat::Depth32BitStencil8Bit => VKFormat::D32SfloatS8Uint,
        }
    }
}
