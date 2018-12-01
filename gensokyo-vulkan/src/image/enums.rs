
use ash::vk;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum ImageInstanceType {

    SampleImage { stage: ImagePipelineStage },
    DepthStencilAttachment,
    DepthStencilImage { format: vk::Format, stage: ImagePipelineStage },
}

/// ImagePipelineStage indicate in which pipeline stage this image is used.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ImagePipelineStage {

    VertexStage,
    FragmentStage,
}

impl ImagePipelineStage {

    pub(super) fn to_raw_flag(&self) -> vk::PipelineStageFlags {
        match self {
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

impl DepthStencilImageFormat {

    pub(super) fn to_raw_format(&self) -> vk::Format {

        match self {
            | DepthStencilImageFormat::Depth32Bit => vk::Format::D32_SFLOAT,
            | DepthStencilImageFormat::Depth24BitStencil8Bit => vk::Format::D24_UNORM_S8_UINT,
            | DepthStencilImageFormat::Depth32BitStencil8Bit => vk::Format::D32_SFLOAT_S8_UINT,
        }
    }

    pub(super) fn aspect_mask(&self) -> vk::ImageAspectFlags {

        match self {
            | DepthStencilImageFormat::Depth32Bit            => vk::ImageAspectFlags::DEPTH,
            | DepthStencilImageFormat::Depth24BitStencil8Bit
            | DepthStencilImageFormat::Depth32BitStencil8Bit => vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL,
        }
    }
}
