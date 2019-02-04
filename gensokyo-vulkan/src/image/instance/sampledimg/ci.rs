
use ash::vk;

use crate::core::GsDevice;

use crate::image::target::GsImage;
use crate::image::instance::base::GsBackendImage;
use crate::image::mipmap::MipmapMethod;
use crate::image::enums::{ ImageInstanceType, ImagePipelineStage };
use crate::image::storage::ImageStorageInfo;
use crate::image::instance::traits::ImageCISpecificApi;
use crate::image::instance::api::ImageCIInheritApi;
use crate::image::instance::sampledimg::image::{ GsSampledImage, ISampledImg };
use crate::image::allocator::ImageAllotCI;

use crate::descriptor::binding::DescriptorMeta;
use crate::descriptor::{ GsDescriptorType, ImageDescriptorType };

use crate::error::VkResult;
use crate::types::vkuint;

pub struct SampledImageCI {

    pipeline_stage: ImagePipelineStage,
    backend: GsBackendImage,

    descriptor: DescriptorMeta,
}

impl GsSampledImage {

    pub fn new(binding: vkuint, storage: ImageStorageInfo, pipeline_stage: ImagePipelineStage) -> SampledImageCI {

        let mut backend = GsBackendImage::from(storage);
        backend.image_ci.property.image_type = vk::ImageType::TYPE_2D;
        backend.image_ci.property.tiling     = vk::ImageTiling::OPTIMAL;
        backend.image_ci.property.usages     = vk::ImageUsageFlags::SAMPLED | vk::ImageUsageFlags::TRANSFER_DST;
        backend.image_ci.property.mipmap     = MipmapMethod::Disable;

        let descriptor = DescriptorMeta {
            binding,
            descriptor_type: GsDescriptorType::Image(ImageDescriptorType::SampledImage),
        };

        SampledImageCI { pipeline_stage, backend, descriptor }
    }
}

impl ImageCISpecificApi for SampledImageCI {
    type IConveyor = ISampledImg;

    fn check_physical_support(&self, device: &GsDevice) -> VkResult<()> {

        self.backend.check_physical_support(device)
    }

    fn refactor(self, _device: &GsDevice, image: GsImage) -> VkResult<(ImageAllotCI, Self::IConveyor)> {

        let isi = ISampledImg::new(self.descriptor);

        let allot_cis = ImageAllotCI::new(
            ImageInstanceType::SampledImage { stage: self.pipeline_stage },
            image, self.backend,
        );

        Ok((allot_cis, isi))
    }
}

impl ImageCIInheritApi for SampledImageCI {

    fn backend(&self) -> &GsBackendImage {
        &self.backend
    }

    fn backend_mut(&mut self) -> &mut GsBackendImage {
        &mut self.backend
    }
}
