
use ash::vk;

use crate::image::entity::ImageEntity;
use crate::image::copy::{ ImageCopiable, ImageFullCopyInfo };
use crate::image::instance::sampler::{ GsSampler, GsSamplerMirror };
use crate::image::instance::traits::{ ImageInstance, IImageConveyor, ImageInstanceInfoDesc };

use crate::descriptor::binding::{ DescriptorBindingImgInfo, DescriptorBindingImgTgt };
use crate::types::{ vkuint, vkDim3D };

/// Wrapper class of Combined Image Sampler in Vulkan.
pub struct GsCombinedImgSampler {

    isi: ICombinedImg,

    entity: ImageEntity,
    desc: ImageInstanceInfoDesc,
}

impl ImageInstance<ICombinedImg> for GsCombinedImgSampler {

    fn build(isi: ICombinedImg, entity: ImageEntity, desc: ImageInstanceInfoDesc) -> Self where Self: Sized {
        GsCombinedImgSampler { isi, entity, desc }
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

pub struct ICombinedImg {

    // no need to destroy sampler manually.
    // it will automatically destroy by GsImageRepository.
    sampler: GsSampler,
}

impl ICombinedImg {

    pub(super) fn new(sampler: GsSampler) -> ICombinedImg {
        ICombinedImg { sampler }
    }
}

impl IImageConveyor for ICombinedImg {

    fn sampler_mirror(&self) -> Option<GsSamplerMirror> {
        Some(self.sampler.mirror())
    }
}
