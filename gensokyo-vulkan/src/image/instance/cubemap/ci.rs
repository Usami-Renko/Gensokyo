
use ash::vk;

use crate::core::GsDevice;

use crate::image::target::GsImage;
use crate::image::enums::{ ImageInstanceType, ImagePipelineStage };
use crate::image::instance::base::GsBackendImage;
use crate::image::mipmap::MipmapMethod;
use crate::image::storage::ImageStorageInfo;
use crate::image::instance::traits::ImageCISpecificApi;
use crate::image::instance::api::ImageCIInheritApi;
use crate::image::instance::cubemap::image::{ GsCubeMapImg, ICubeMap };
use crate::image::instance::sampler::{ GsSampler, SamplerCI };
use crate::image::allocator::ImageAllotCI;

use crate::descriptor::binding::DescriptorMeta;
use crate::descriptor::{ GsDescriptorType, ImageDescriptorType };

use crate::error::VkResult;
use crate::types::vkuint;

pub struct CubeMapImgCI {

    pipeline_stage: ImagePipelineStage,
    backend: GsBackendImage,

    sampler_ci: SamplerCI,
}

impl GsCubeMapImg {

    pub fn new(binding: vkuint, storage: ImageStorageInfo, pipeline_stage: ImagePipelineStage) -> CubeMapImgCI {

        let mut backend = GsBackendImage::from(storage);
        backend.image_ci.property.flags        = vk::ImageCreateFlags::CUBE_COMPATIBLE;
        backend.image_ci.property.image_type   = vk::ImageType::TYPE_2D;
        backend.image_ci.property.tiling       = vk::ImageTiling::OPTIMAL;
        backend.image_ci.property.usages       = vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST;
        backend.image_ci.property.mipmap       = MipmapMethod::Disable;
        backend.image_ci.property.array_layers = 6; // cube map is always load from an 2D image with 6 layers.

        backend.view_ci.view_type = vk::ImageViewType::CUBE;
        backend.view_ci.subrange.0.layer_count = 6;

        let mut sampler_ci = GsSampler::new();
        sampler_ci.reset_descriptor(DescriptorMeta {
            binding,
            descriptor_type: GsDescriptorType::Image(ImageDescriptorType::CombinedImageSampler)
        });

        CubeMapImgCI { pipeline_stage, backend, sampler_ci }
    }
}

impl CubeMapImgCI {

    pub fn reset_sampler(&mut self, sampler_ci: SamplerCI) {
        self.sampler_ci = sampler_ci;
    }
}

impl ImageCISpecificApi for CubeMapImgCI {
    type IConveyor = ICubeMap;

    fn check_physical_support(&self, device: &GsDevice) -> VkResult<()> {

        self.backend.check_mipmap_support(device)
    }

    fn refactor(self, device: &GsDevice, image: GsImage) -> VkResult<(ImageAllotCI, Self::IConveyor)> {

        let sampler = self.sampler_ci.build(device)?;
        let isi = ICubeMap::new(sampler);

        let allot_cis = ImageAllotCI::new(
            ImageInstanceType::CombinedImageSampler { stage: self.pipeline_stage },
            image, self.backend,
        );

        Ok((allot_cis, isi))
    }
}

impl ImageCIInheritApi for CubeMapImgCI {

    fn backend(&self) -> &GsBackendImage {
        &self.backend
    }

    fn backend_mut(&mut self) -> &mut GsBackendImage {
        &mut self.backend
    }
}
