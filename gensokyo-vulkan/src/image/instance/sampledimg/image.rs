
use ash::vk;

use crate::image::entity::ImageEntity;
use crate::image::traits::{ ImageInstance, ImageCopiable };
use crate::image::utils::{ ImageCopyInfo, ImageCopySubrange };
use crate::image::instance::sampler::GsSamplerMirror;
use crate::image::instance::traits::{ IImageConveyor, ImageInstanceInfoDesc };

use crate::descriptor::DescriptorBindingContent;
use crate::descriptor::{ DescriptorImageBindingInfo, DescriptorImageBindableTarget };

/// Wrapper class of Sampled Image in Vulkan.
pub struct GsSampledImage {

    isi: ISampledImg,

    entity: ImageEntity,
    desc: ImageInstanceInfoDesc,
}

pub struct ISampledImg {

    binding: DescriptorBindingContent,
}

impl ImageInstance<ISampledImg> for GsSampledImage {

    fn build(isi: ISampledImg, entity: ImageEntity, desc: ImageInstanceInfoDesc) -> Self where Self: Sized {
        GsSampledImage { isi, entity, desc }
    }
}

impl DescriptorImageBindableTarget for GsSampledImage {

    fn binding_info(&self) -> DescriptorImageBindingInfo {

        DescriptorImageBindingInfo {
            content        : self.isi.binding.clone(),
            sampler_handle : vk::Sampler::null(),
            dst_layout     : vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            view_handle    : self.entity.view,
        }
    }
}

impl ImageCopiable for GsSampledImage {

    fn copy_range(&self, subrange: ImageCopySubrange) -> ImageCopyInfo {

        ImageCopyInfo {
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

    pub(super) fn new(binding: DescriptorBindingContent) -> ISampledImg {
        ISampledImg { binding }
    }
}
