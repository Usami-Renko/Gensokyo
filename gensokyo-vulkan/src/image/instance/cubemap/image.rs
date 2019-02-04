
use ash::vk;

use crate::image::entity::ImageEntity;
use crate::image::copy::{ ImageCopiable, ImageFullCopyInfo };
use crate::image::instance::sampler::{ GsSampler, GsSamplerMirror };
use crate::image::instance::traits::{ ImageInstance, IImageConveyor, ImageInstanceInfoDesc };

use crate::descriptor::binding::{ DescriptorBindingImgInfo, DescriptorBindingImgTgt };
use crate::types::{ vkuint, vkDim3D };

pub struct GsCubeMapImg {

    icm: ICubeMap,

    entity : ImageEntity,
    desc   : ImageInstanceInfoDesc,
}

impl ImageInstance<ICubeMap> for GsCubeMapImg {

    fn build(icm: ICubeMap, entity: ImageEntity, desc: ImageInstanceInfoDesc) -> Self where Self: Sized {
        GsCubeMapImg { icm, entity, desc }
    }
}

impl DescriptorBindingImgTgt for GsCubeMapImg {

    fn binding_info(&self) -> DescriptorBindingImgInfo {

        DescriptorBindingImgInfo {
            meta           : self.icm.sampler.descriptor.clone(),
            sampler_handle : self.icm.sampler.handle,
            dst_layout     : vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            view_handle    : self.entity.view,
        }
    }
}

impl ImageCopiable for GsCubeMapImg {

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
                layer_count      : 6, // cube map image has 6 layer.
            },
        }
    }
}

pub struct ICubeMap {

    sampler: GsSampler,
}

impl ICubeMap {

    pub(super) fn new(sampler: GsSampler) -> ICubeMap {
        ICubeMap { sampler }
    }
}

impl IImageConveyor for ICubeMap {

    fn sampler_mirror(&self) -> Option<GsSamplerMirror> {
        Some(self.sampler.mirror())
    }
}
