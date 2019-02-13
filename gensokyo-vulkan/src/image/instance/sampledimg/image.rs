
use ash::vk;

use crate::image::entity::ImageEntity;
use crate::image::copy::{ ImageCopiable, ImageFullCopyInfo };
use crate::image::instance::sampler::GsSamplerMirror;
use crate::image::instance::traits::{ ImageInstance, IImageConveyor, ImageInstanceInfoDesc };

use crate::descriptor::binding::DescriptorMeta;
use crate::descriptor::binding::{ DescriptorBindingImgInfo, DescriptorBindingImgTgt };

use crate::types::{ vkuint, vkDim3D };

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

    fn full_copy_mipmap(&self, copy_mip_level: vkuint) -> ImageFullCopyInfo {

        use std::cmp::max;

        ImageFullCopyInfo {
            handle: self.entity.image,
            layout: self.desc.current_layout,
            extent: vkDim3D {
                width : max(self.desc.dimension.width  >> copy_mip_level, 1),
                height: max(self.desc.dimension.height >> copy_mip_level, 1),
                depth : 1,
            },
            sub_resource_layers: vk::ImageSubresourceLayers {
                aspect_mask      : vk::ImageAspectFlags::COLOR,
                mip_level        : copy_mip_level,
                base_array_layer : 0,
                layer_count      : 1,
            },
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
