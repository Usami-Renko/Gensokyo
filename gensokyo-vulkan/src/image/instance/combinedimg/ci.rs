
use ash::vk;

use crate::core::GsDevice;

use crate::image::target::{ GsImage, ImageTgtCI, ImagePropertyCI, ImageSpecificCI };
use crate::image::view::{ ImageViewCI, ImageSubRange };
use crate::image::enums::{ ImageInstanceType, ImagePipelineStage };
use crate::image::storage::ImageStorageInfo;
use crate::image::instance::traits::{ ImageCIAbstract, ImageTgtCIAbs, ImageViewCIAbs };
use crate::image::instance::combinedimg::image::{ GsCombinedImgSampler, ICombinedImg };
use crate::image::instance::combinedimg::mipmap::MipmapMethod;
use crate::image::instance::sampler::{ GsSampler, SamplerCI };
use crate::image::allocator::ImageAllotCI;

use crate::descriptor::{ DescriptorBindingContent, GsDescriptorType, ImageDescriptorType };

use crate::error::{ VkResult, VkError };
use crate::types::vkuint;

/// Combined Image Sampler Create Info.
pub struct CombinedImgSamplerCI {

    pipeline_stage: ImagePipelineStage,
    image_ci: ImageTgtCI,
    view_ci : ImageViewCI,

    sampler_ci: SamplerCI,

    storage: ImageStorageInfo,
}

impl GsCombinedImgSampler {

    pub fn new(binding: vkuint, count: vkuint, storage: ImageStorageInfo, pipeline_stage: ImagePipelineStage) -> CombinedImgSamplerCI {

        let mut property = ImagePropertyCI::default();
        property.image_type = vk::ImageType::TYPE_2D;
        property.tiling = vk::ImageTiling::OPTIMAL;
        property.usages = vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST;
        property.mipmap = MipmapMethod::Disable; // default to disable mipmap generation.

        let mut specific = ImageSpecificCI::default();
        specific.format    = storage.format;
        specific.dimension = storage.dimension;

        let mut sampler_ci = GsSampler::new();
        sampler_ci.reset_binding(DescriptorBindingContent {
            binding, count,
            descriptor_type: GsDescriptorType::Image(ImageDescriptorType::CombinedImageSampler),
        });

        CombinedImgSamplerCI {
            pipeline_stage, storage, sampler_ci,
            image_ci: ImageTgtCI { property, specific },
            view_ci : ImageViewCI::new(vk::ImageViewType::TYPE_2D, vk::ImageAspectFlags::COLOR),
        }
    }
}

impl CombinedImgSamplerCI {

    pub fn set_mipmap(&mut self, method: MipmapMethod) {
        self.image_ci.property.mipmap = method;

        match method {
            | MipmapMethod::Disable => {
                self.image_ci.property.usages = vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST;
                self.image_ci.property.mip_levels = 1;
                self.view_ci.subrange.0.base_mip_level = 0;
                self.view_ci.subrange.0.level_count    = 1;
            },
            | MipmapMethod::StepBlit
            | MipmapMethod::BaseLevelBlit => {
                let mip_level = self.estimate_mip_levels();

                self.image_ci.property.usages = vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_SRC | vk::ImageUsageFlags::TRANSFER_DST;
                self.image_ci.property.mip_levels = mip_level;
                self.view_ci.subrange.0.base_mip_level = 0;
                self.view_ci.subrange.0.level_count    = mip_level;
            },
        }
    }

    pub fn reset_sampler(&mut self, sampler_ci: SamplerCI) {
        self.sampler_ci.reset_ci(sampler_ci);
    }

    pub fn estimate_mip_levels(&self) -> vkuint {

        use std::cmp::max;
        let max_extent = max(self.image_ci.specific.dimension.width, self.image_ci.specific.dimension.height) as f32;
        (max_extent.log2().floor() as vkuint) + 1
    }
}

impl ImageCIAbstract<ICombinedImg> for CombinedImgSamplerCI {

    fn check_physical_support(&self, device: &GsDevice) -> VkResult<()> {

        if self.image_ci.property.mipmap.is_support_by_device(device, &self.image_ci)? {
            Ok(())
        } else {
            Err(VkError::other(format!("vk::Format: {:?} is not support for mipmap generation", self.image_ci.specific.format)))
        }
    }

    fn build(&self, device: &GsDevice) -> VkResult<GsImage> {
        self.image_ci.build(device)
    }

    fn refactor(self, device: &GsDevice, image: GsImage) -> VkResult<(ImageAllotCI, ICombinedImg)> {

        let sampler = self.sampler_ci.build(device)?;
        let isi = ICombinedImg::new(sampler);

        let allot_cis = ImageAllotCI::new(
            ImageInstanceType::SampleImage { stage: self.pipeline_stage },
            self.storage, image, self.image_ci, self.view_ci
        );

        Ok((allot_cis, isi))
    }
}

impl_image_desc_info_abs!(CombinedImgSamplerCI);
