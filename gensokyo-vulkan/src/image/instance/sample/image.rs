
use ash::vk;

use crate::core::GsDevice;

use crate::image::entity::ImageEntity;
use crate::image::traits::{ ImageInstance, ImageCopiable };
use crate::image::sampler::GsSampler;
use crate::image::utils::{ ImageCopyInfo, ImageCopySubrange };
use crate::image::instance::desc::ImageInstanceInfoDesc;

use crate::descriptor::{ DescriptorImageBindingInfo, DescriptorImageBindableTarget };
use crate::descriptor::DescriptorBindingContent;

pub struct GsSampleImage {

    isi: ISampleImg,

    entity: ImageEntity,
    desc: ImageInstanceInfoDesc,
}

pub struct ISampleImg {

    sampler: GsSampler,
    binding: DescriptorBindingContent,
}

impl ImageInstance<ISampleImg> for GsSampleImage {

    fn build(isi: ISampleImg, entity: ImageEntity, desc: ImageInstanceInfoDesc) -> Self where Self: Sized {
        GsSampleImage { isi, entity, desc }
    }
}

impl GsSampleImage {

    pub fn destroy(&self, device: &GsDevice) {
        self.isi.sampler.destroy(device);
    }
}

impl ISampleImg {

    pub(super) fn new(sampler: GsSampler, binding: DescriptorBindingContent) -> ISampleImg {
        ISampleImg { sampler, binding }
    }
}

impl DescriptorImageBindableTarget for GsSampleImage {

    fn binding_info(&self) -> DescriptorImageBindingInfo {

        DescriptorImageBindingInfo {
            content        : self.isi.binding.clone(),
            sampler_handle : self.isi.sampler.handle,
            dst_layout     : vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            view_handle    : self.entity.view,
        }
    }
}

impl ImageCopiable for GsSampleImage {

    fn copy_range(&self, subrange: ImageCopySubrange) -> ImageCopyInfo {

        ImageCopyInfo {
            handle: self.entity.image,
            layout: self.desc.current_layout,
            extent: self.desc.dimension,
            sub_resource_layers: subrange,
        }
    }
}
