
use ash::vk;

use crate::core::GsDevice;

use crate::image::target::{ GsImage, ImageDescInfo, ImagePropertyInfo, ImageSpecificInfo };
use crate::image::view::ImageViewDescInfo;
use crate::image::sampler::GsSamplerCI;
use crate::image::enums::{ ImageInstanceType, ImagePipelineStage };
use crate::image::storage::ImageStorageInfo;
use crate::image::instance::traits::{ GsImageDescAbs, GsImageViewDescAbs, ImageInfoAbstract };
use crate::image::instance::sample::ISampleImg;
use crate::image::allocator::ImageAllotInfo;

use crate::descriptor::{ DescriptorBindingContent, GsDescriptorType, ImageDescriptorType };

use crate::error::VkResult;
use crate::types::vkuint;

pub struct GsSampleImgInfo {

    pipeline_stage: ImagePipelineStage,
    image_desc  : ImageDescInfo,
    view_desc   : ImageViewDescInfo,

    sampler_ci: GsSamplerCI,
    binding: DescriptorBindingContent,

    storage: ImageStorageInfo,
}

impl GsSampleImgInfo {

    pub fn new(binding: vkuint, count: vkuint, storage: ImageStorageInfo, pipeline_stage: ImagePipelineStage) -> GsSampleImgInfo {

        let mut property = ImagePropertyInfo::default();
        property.image_type = vk::ImageType::TYPE_2D;
        property.tiling = vk::ImageTiling::OPTIMAL;
        property.usages = vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST;

        let mut specific = ImageSpecificInfo::default();
        specific.format    = storage.format;
        specific.dimension = storage.dimension;

        GsSampleImgInfo {
            pipeline_stage, storage,
            image_desc: ImageDescInfo { property, specific },
            view_desc : ImageViewDescInfo::new(vk::ImageViewType::TYPE_2D, vk::ImageAspectFlags::COLOR),
            sampler_ci: GsSamplerCI::new().build(),
            binding: DescriptorBindingContent {
                binding, count,
                descriptor_type: GsDescriptorType::Image(ImageDescriptorType::CombinedImageSampler)
            },
        }
    }

    pub fn reset_sampler(&mut self, ci: GsSamplerCI) {
        self.sampler_ci = ci;
    }
}

impl ImageInfoAbstract<ISampleImg> for GsSampleImgInfo {

    fn build(&self, device: &GsDevice) -> VkResult<GsImage> {
        self.image_desc.build(device)
    }

    fn refactor(self, device: &GsDevice, image: GsImage) -> VkResult<(ImageAllotInfo, ISampleImg)> {

        let sampler = self.sampler_ci.build(device)?;
        let isi = ISampleImg::new(sampler, self.binding);

        let allot = ImageAllotInfo::new(
            ImageInstanceType::SampleImage { stage: self.pipeline_stage },
            self.storage, image, self.image_desc, self.view_desc
        );

        Ok((allot, isi))
    }
}

impl_image_desc_info_abs!(GsSampleImgInfo);
