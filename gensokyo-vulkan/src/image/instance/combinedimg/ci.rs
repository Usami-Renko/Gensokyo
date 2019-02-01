
use ash::vk;

use crate::core::GsDevice;

use crate::image::target::GsImage;
use crate::image::enums::{ ImageInstanceType, ImagePipelineStage };
use crate::image::storage::ImageStorageInfo;
use crate::image::instance::base::{ GsBackendImage, MipmapMethod };
use crate::image::instance::traits::ImageCISpecificApi;
use crate::image::instance::combinedimg::image::{ GsCombinedImgSampler, ICombinedImg };
use crate::image::instance::api::ImageCIInheritApi;
use crate::image::instance::sampler::{ GsSampler, SamplerCI };
use crate::image::allocator::ImageAllotCI;

use crate::descriptor::{ DescriptorBindingContent, GsDescriptorType, ImageDescriptorType };

use crate::error::{ VkResult, VkError };
use crate::types::vkuint;

/// Combined Image Sampler Create Info.
pub struct CombinedImgSamplerCI {

    pipeline_stage: ImagePipelineStage,
    backend: GsBackendImage,

    sampler_ci: SamplerCI,
}

impl GsCombinedImgSampler {

    pub fn new(binding: vkuint, count: vkuint, storage: ImageStorageInfo, pipeline_stage: ImagePipelineStage) -> CombinedImgSamplerCI {

        let mut backend = GsBackendImage::from(storage);
        backend.image_ci.property.image_type = vk::ImageType::TYPE_2D;
        backend.image_ci.property.tiling = vk::ImageTiling::OPTIMAL;
        backend.image_ci.property.usages = vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST;
        backend.image_ci.property.mipmap = MipmapMethod::Disable; // default to disable mipmap generation.

        let mut sampler_ci = GsSampler::new();
        sampler_ci.reset_binding(DescriptorBindingContent {
            binding, count,
            descriptor_type: GsDescriptorType::Image(ImageDescriptorType::CombinedImageSampler),
        });

        CombinedImgSamplerCI { pipeline_stage, sampler_ci, backend }
    }
}

impl CombinedImgSamplerCI {

    pub fn reset_sampler(&mut self, sampler_ci: SamplerCI) {
        self.sampler_ci.reset_ci(sampler_ci);
    }
}

impl ImageCISpecificApi for CombinedImgSamplerCI {
    type IConveyor = ICombinedImg;

    fn check_physical_support(&self, device: &GsDevice) -> VkResult<()> {

        if self.backend.image_ci.property.mipmap.is_support_by_device(device, &self.backend.image_ci)? {
            Ok(())
        } else {
            Err(VkError::other(format!("vk::Format: {:?} is not support for mipmap generation", self.backend.image_ci.specific.format)))
        }
    }

    fn refactor(self, device: &GsDevice, image: GsImage) -> VkResult<(ImageAllotCI, Self::IConveyor)> {

        let sampler = self.sampler_ci.build(device)?;
        let isi = ICombinedImg::new(sampler);

        let allot_cis = ImageAllotCI::new(
            ImageInstanceType::CombinedImageSampler { stage: self.pipeline_stage },
            image, self.backend,
        );

        Ok((allot_cis, isi))
    }
}

impl ImageCIInheritApi for CombinedImgSamplerCI {

    fn backend(&self) -> &GsBackendImage {
        &self.backend
    }

    fn backend_mut(&mut self) -> &mut GsBackendImage {
        &mut self.backend
    }
}