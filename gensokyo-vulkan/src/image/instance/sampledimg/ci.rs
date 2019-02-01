
use ash::vk;

use crate::image::target::{ ImageTgtCI, ImagePropertyCI, ImageSpecificCI };
use crate::image::view::ImageViewCI;
use crate::image::enums::ImagePipelineStage;
use crate::image::storage::ImageStorageInfo;
use crate::image::instance::sampledimg::image::GsSampledImage;

use crate::descriptor::{ DescriptorBindingContent, GsDescriptorType, ImageDescriptorType };

use crate::types::vkuint;

pub struct SampledImageCI {

    pipeline_stage: ImagePipelineStage,
    image_ci: ImageTgtCI,
    view_ci : ImageViewCI,

    storage: ImageStorageInfo,

    binding: DescriptorBindingContent,
}

impl GsSampledImage {

    pub fn new(binding: vkuint, count: vkuint, storage: ImageStorageInfo, pipeline_stage: ImagePipelineStage) -> SampledImageCI {

        let mut property = ImagePropertyCI::default();
        property.image_type = vk::ImageType::TYPE_2D;
        property.tiling     = vk::ImageTiling::OPTIMAL;
        property.usages     = vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST;
        //property.mipmap     =

        let mut specific = ImageSpecificCI::default();
        specific.format    = storage.format;
        specific.dimension = storage.dimension;

        SampledImageCI {
            pipeline_stage, storage,
            image_ci: ImageTgtCI { property, specific },
            view_ci : ImageViewCI::new(vk::ImageViewType::TYPE_2D, vk::ImageAspectFlags::COLOR),
            binding : DescriptorBindingContent {
                binding, count,
                descriptor_type: GsDescriptorType::Image(ImageDescriptorType::SampledImage),
            },
        }
    }
}
