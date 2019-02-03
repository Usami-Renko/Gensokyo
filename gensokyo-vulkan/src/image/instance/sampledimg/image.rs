
use ash::vk;

use crate::image::entity::ImageEntity;
use crate::image::traits::{ ImageInstance, ImageCopiable };
use crate::image::utils::{ ImageFullCopyInfo, ImageCopySubrange };
use crate::image::instance::sampler::GsSamplerMirror;
use crate::image::instance::traits::{ IImageConveyor, ImageInstanceInfoDesc };

use crate::descriptor::binding::DescriptorMeta;
use crate::descriptor::binding::{ DescriptorBindingImgInfo, DescriptorBindingImgTgt };

/// Wrapper class of Sampled Image in Vulkan.
pub struct GsSampledImage {

    isi: ISampledImg,

    entity: ImageEntity,
    desc: ImageInstanceInfoDesc,
}

pub struct ISampledImg {

    descriptor: DescriptorMeta,
}

impl ImageInstance<ISampledImg> for GsSampledImage {

    fn build(isi: ISampledImg, entity: ImageEntity, desc: ImageInstanceInfoDesc) -> Self where Self: Sized {
        GsSampledImage { isi, entity, desc }
    }
}

impl DescriptorBindingImgTgt for GsSampledImage {

    fn binding_info(&self) -> DescriptorBindingImgInfo {

        DescriptorBindingImgInfo {
            meta           : self.isi.descriptor.clone(),
            sampler_handle : vk::Sampler::null(),
            dst_layout     : vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            view_handle    : self.entity.view,
        }
    }
}

impl ImageCopiable for GsSampledImage {

    fn copy_range(&self, subrange: ImageCopySubrange) -> ImageFullCopyInfo {

        ImageFullCopyInfo {
            handle: self.entity.image,
            layout: self.desc.current_layout,
            extent: self.desc.dimension,
            sub_resource_layers: subrange,
        }
    }
}

impl IImageConveyor for ISampledImg {

    fn sampler_mirror(&self) -> Option<GsSamplerMirror> {
        None
    }
}

impl ISampledImg {

    pub(super) fn new(descriptor: DescriptorMeta) -> ISampledImg {
        ISampledImg { descriptor }
    }
}
