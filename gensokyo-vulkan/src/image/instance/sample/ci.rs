
use ash::vk;

use crate::core::GsDevice;

use crate::image::target::{ GsImage, ImageTgtCI, ImagePropertyCI, ImageSpecificCI };
use crate::image::view::ImageViewCI;
use crate::image::sampler::GsSamplerCI;
use crate::image::enums::{ ImageInstanceType, ImagePipelineStage };
use crate::image::storage::ImageStorageInfo;
use crate::image::instance::traits::{ ImageCIAbstract, ImageTgtCIAbs, ImageViewCIAbs };
use crate::image::instance::sample::{ GsSampleImage, ISampleImg };
use crate::image::allocator::ImageAllotCI;

use crate::descriptor::{ DescriptorBindingContent, GsDescriptorType, ImageDescriptorType };

use crate::error::VkResult;
use crate::types::vkuint;

/// Sample Image Create Info.
pub struct SampleImageCI {

    pipeline_stage: ImagePipelineStage,
    image_ci: ImageTgtCI,
    view_ci : ImageViewCI,

    sampler_ci: GsSamplerCI,
    binding: DescriptorBindingContent,

    storage: ImageStorageInfo,
}

impl GsSampleImage {

    pub fn new(binding: vkuint, count: vkuint, storage: ImageStorageInfo, pipeline_stage: ImagePipelineStage) -> SampleImageCI {

        let mut property = ImagePropertyCI::default();
        property.image_type = vk::ImageType::TYPE_2D;
        property.tiling = vk::ImageTiling::OPTIMAL;
        property.usages = vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST;

        let mut specific = ImageSpecificCI::default();
        specific.format    = storage.format;
        specific.dimension = storage.dimension;

        SampleImageCI {
            pipeline_stage, storage,
            image_ci: ImageTgtCI { property, specific },
            view_ci : ImageViewCI::new(vk::ImageViewType::TYPE_2D, vk::ImageAspectFlags::COLOR),
            sampler_ci: GsSamplerCI::new().build(),
            binding: DescriptorBindingContent {
                binding, count,
                descriptor_type: GsDescriptorType::Image(ImageDescriptorType::CombinedImageSampler)
            },
        }
    }
}

impl SampleImageCI {

    pub fn reset_sampler(&mut self, ci: GsSamplerCI) {
        self.sampler_ci = ci;
    }
}

impl ImageCIAbstract<ISampleImg> for SampleImageCI {

    fn build(&self, device: &GsDevice) -> VkResult<GsImage> {
        self.image_ci.build(device)
    }

    fn refactor(self, device: &GsDevice, image: GsImage) -> VkResult<(ImageAllotCI, ISampleImg)> {

        let sampler = self.sampler_ci.build(device)?;
        let isi = ISampleImg::new(sampler, self.binding);

        let allot_cis = ImageAllotCI::new(
            ImageInstanceType::SampleImage { stage: self.pipeline_stage },
            self.storage, image, self.image_ci, self.view_ci
        );

        Ok((allot_cis, isi))
    }
}

impl_image_desc_info_abs!(SampleImageCI);
