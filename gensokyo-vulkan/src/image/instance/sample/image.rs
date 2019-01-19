
use ash::vk;

use crate::core::device::GsDevice;

use crate::image::entity::ImageEntity;
use crate::image::traits::{ ImageInstance, ImageCopiable };
use crate::image::sampler::GsSampler;
use crate::image::utils::ImageCopyInfo;
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

    fn new(isi: ISampleImg, entity: ImageEntity, desc: ImageInstanceInfoDesc) -> Self where Self: Sized {
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

    fn copy_info(&self) -> ImageCopyInfo {

        use crate::image::utils::image_subrange_to_layers;
        let subrange_layers = image_subrange_to_layers(&self.desc.subrange);

        ImageCopyInfo::new(&self.entity, subrange_layers, self.desc.current_layout, self.desc.dimension)
    }
}
