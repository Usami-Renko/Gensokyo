
use ash::vk;

use core::device::GsDevice;

use image::view::GsImageView;
use image::entity::ImageEntity;
use image::traits::{ ImageInstance, ImageCopiable };
use image::sampler::GsSampler;
use image::utils::ImageCopyInfo;
use image::instance::sample::SampleImageInfo;
use image::instance::ImageInstanceInfoDesc;
use image::allocator::ImageAllocateInfo;

use descriptor::{ DescriptorImageBindingInfo, DescriptorImageBindableTarget };
use descriptor::DescriptorBindingContent;

pub struct GsSampleImage {

    sampler: GsSampler,
    binding: DescriptorBindingContent,

    entity: ImageEntity,
    desc: ImageInstanceInfoDesc,
}

impl GsSampleImage {

    pub(crate) fn setup(info: SampleImageInfo, sampler: GsSampler, allocate_info: &ImageAllocateInfo, view: &GsImageView)
        -> GsSampleImage {

        GsSampleImage {
            sampler,
            binding: info.binding(),
            entity: ImageEntity::new(&allocate_info.image, view),
            desc: allocate_info.gen_desc(),
        }
    }

    pub fn cleanup(&self, device: &GsDevice) {
        self.sampler.cleanup(device);
    }
}

impl DescriptorImageBindableTarget for GsSampleImage {

    fn binding_info(&self) -> DescriptorImageBindingInfo {

        DescriptorImageBindingInfo {
            content        : self.binding.clone(),
            sampler_handle : self.sampler.handle,
            dst_layout     : vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            view_handle    : self.entity.view,
        }
    }
}

impl ImageInstance for GsSampleImage {}

impl ImageCopiable for GsSampleImage {

    fn copy_info(&self) -> ImageCopyInfo {

        use image::utils::image_subrange_to_layers;
        let subrange_layers = image_subrange_to_layers(&self.desc.subrange);

        ImageCopyInfo::new(&self.entity, subrange_layers, self.desc.current_layout, self.desc.dimension)
    }
}
