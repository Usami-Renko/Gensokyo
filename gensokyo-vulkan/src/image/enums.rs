
use ash::vk;

use crate::image::format::GsImageFormat;

use crate::types::format::Format;
use crate::types::vkuint;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ImageInstanceType {

    CombinedImageSampler { stage: ImagePipelineStage },
    SampledImage { stage: ImagePipelineStage },
    CubeMapImage { stage: ImagePipelineStage },
    DepthStencilAttachment,
    DepthStencilImage { format: Format, stage: ImagePipelineStage },
}

impl ImageInstanceType {

    pub(super) fn layer_count(&self) -> vkuint {
        match self {
            | ImageInstanceType::CubeMapImage { .. } => 6,
            | _ => 1,
        }
    }
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

impl From<DepthStencilImageFormat> for GsImageFormat {

    fn from(image_format: DepthStencilImageFormat) -> GsImageFormat {

        match image_format {
            | DepthStencilImageFormat::Depth32Bit => GsImageFormat::Uncompressed(Format::D32_SFLOAT),
            | DepthStencilImageFormat::Depth24BitStencil8Bit => GsImageFormat::Uncompressed(Format::D24_UNORM_S8_UINT),
            | DepthStencilImageFormat::Depth32BitStencil8Bit => GsImageFormat::Uncompressed(Format::D32_SFLOAT_S8_UINT),
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
