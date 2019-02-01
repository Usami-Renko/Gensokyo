
use ash::vk;
use crate::types::format::GsFormat;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ImageInstanceType {

    CombinedImageSampler { stage: ImagePipelineStage },
    SampledImage { stage: ImagePipelineStage },
    DepthStencilAttachment,
    DepthStencilImage { format: GsFormat, stage: ImagePipelineStage },
}

/// ImagePipelineStage indicate in which pipeline stage this image is used.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ImagePipelineStage {

    VertexStage,
    FragmentStage,
}

impl From<ImagePipelineStage> for vk::PipelineStageFlags {

    fn from(flag: ImagePipelineStage) -> vk::PipelineStageFlags {
        match flag {
            | ImagePipelineStage::VertexStage   => vk::PipelineStageFlags::VERTEX_SHADER,
            | ImagePipelineStage::FragmentStage => vk::PipelineStageFlags::FRAGMENT_SHADER,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum DepthStencilImageFormat {

    Depth32Bit,
    Depth32BitStencil8Bit,
    Depth24BitStencil8Bit,
}

impl From<DepthStencilImageFormat> for GsFormat {

    fn from(image_format: DepthStencilImageFormat) -> GsFormat {

        match image_format {
            | DepthStencilImageFormat::Depth32Bit => GsFormat::D32_SFLOAT,
            | DepthStencilImageFormat::Depth24BitStencil8Bit => GsFormat::D24_UNORM_S8_UINT,
            | DepthStencilImageFormat::Depth32BitStencil8Bit => GsFormat::D32_SFLOAT_S8_UINT,
        }
    }
}

impl DepthStencilImageFormat {

    pub(super) fn aspect_mask(&self) -> vk::ImageAspectFlags {

        match self {
            | DepthStencilImageFormat::Depth32Bit            => vk::ImageAspectFlags::DEPTH,
            | DepthStencilImageFormat::Depth24BitStencil8Bit
            | DepthStencilImageFormat::Depth32BitStencil8Bit => vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL,
        }
    }
}
