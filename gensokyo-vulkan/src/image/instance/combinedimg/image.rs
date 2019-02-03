
use ash::vk;

use crate::image::entity::ImageEntity;
use crate::image::traits::{ ImageInstance, ImageCopiable };
use crate::image::utils::{ ImageFullCopyInfo, ImageCopySubrange };
use crate::image::instance::sampler::{ GsSampler, GsSamplerMirror };
use crate::image::instance::traits::{ IImageConveyor, ImageInstanceInfoDesc };

use crate::descriptor::binding::{ DescriptorBindingImgInfo, DescriptorBindingImgTgt };

/// Wrapper class of Combined Image Sampler in Vulkan.
pub struct GsCombinedImgSampler {

    isi: ICombinedImg,

    entity: ImageEntity,
    desc: ImageInstanceInfoDesc,
}

pub struct ICombinedImg {

    // no need to destroy sampler manually.
    // it will automatically destroy by GsImageRepository.
    sampler: GsSampler,
}

impl ImageInstance<ICombinedImg> for GsCombinedImgSampler {

    fn build(isi: ICombinedImg, entity: ImageEntity, desc: ImageInstanceInfoDesc) -> Self where Self: Sized {
        GsCombinedImgSampler { isi, entity, desc }
    }
}

impl ICombinedImg {

    pub(super) fn new(sampler: GsSampler) -> ICombinedImg {
        ICombinedImg { sampler }
    }
}

impl DescriptorBindingImgTgt for GsCombinedImgSampler {

    fn binding_info(&self) -> DescriptorBindingImgInfo {

        DescriptorBindingImgInfo {
            meta           : self.isi.sampler.descriptor.clone(),
            sampler_handle : self.isi.sampler.handle,
            dst_layout     : vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            view_handle    : self.entity.view,
        }
    }
}

impl ImageCopiable for GsCombinedImgSampler {

    fn copy_range(&self, subrange: ImageCopySubrange) -> ImageFullCopyInfo {

        ImageFullCopyInfo {
            handle: self.entity.image,
            layout: self.desc.current_layout,
            extent: self.desc.dimension,
            sub_resource_layers: subrange,
        }
    }
}

impl IImageConveyor for ICombinedImg {

    fn sampler_mirror(&self) -> Option<GsSamplerMirror> {
        Some(self.sampler.mirror())
    }
}
