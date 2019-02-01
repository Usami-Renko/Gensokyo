
use ash::vk;

use crate::core::GsDevice;

use crate::image::entity::ImageEntity;
use crate::image::traits::{ ImageInstance, ImageCopiable };
use crate::image::utils::{ ImageCopyInfo, ImageCopySubrange };
use crate::image::instance::sampler::{ GsSampler, GsSamplerMirror };
use crate::image::instance::desc::ImageInstanceInfoDesc;
use crate::image::instance::traits::IImageConveyor;

use crate::descriptor::{ DescriptorImageBindingInfo, DescriptorImageBindableTarget };

/// Wrapper class of Combined Image Sampler in Vulkan.
pub struct GsCombinedImgSampler {

    isi: ICombinedImg,

    entity: ImageEntity,
    desc: ImageInstanceInfoDesc,
}

pub struct ICombinedImg {

    sampler: GsSampler,
}

impl ImageInstance<ICombinedImg> for GsCombinedImgSampler {

    fn build(isi: ICombinedImg, entity: ImageEntity, desc: ImageInstanceInfoDesc) -> Self where Self: Sized {
        GsCombinedImgSampler { isi, entity, desc }
    }
}

impl GsCombinedImgSampler {

    pub fn destroy(&self, device: &GsDevice) {
        self.isi.sampler.destroy(device);
    }
}

impl ICombinedImg {

    pub(super) fn new(sampler: GsSampler) -> ICombinedImg {
        ICombinedImg { sampler }
    }
}

impl DescriptorImageBindableTarget for GsCombinedImgSampler {

    fn binding_info(&self) -> DescriptorImageBindingInfo {

        DescriptorImageBindingInfo {
            content        : self.isi.sampler.binding.clone(),
            sampler_handle : self.isi.sampler.handle,
            dst_layout     : vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            view_handle    : self.entity.view,
        }
    }
}

impl ImageCopiable for GsCombinedImgSampler {

    fn copy_range(&self, subrange: ImageCopySubrange) -> ImageCopyInfo {

        ImageCopyInfo {
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
