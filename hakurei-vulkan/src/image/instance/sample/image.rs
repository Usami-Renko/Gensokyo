
use ash::vk;

use core::device::HaDevice;

use image::view::HaImageView;
use image::entity::ImageEntity;
use image::traits::{ ImageInstance, ImageCopiable };
use image::sampler::HaSampler;
use image::utils::ImageCopyInfo;
use image::instance::sample::SampleImageInfo;
use image::instance::ImageInstanceInfoDesc;
use image::allocator::ImageAllocateInfo;

use descriptor::{ DescriptorImageBindingInfo, DescriptorImageBindableTarget };
use descriptor::DescriptorBindingContent;

pub struct HaSampleImage {

    sampler: HaSampler,
    binding: DescriptorBindingContent,

    entity: ImageEntity,
    desc: ImageInstanceInfoDesc,
}

impl HaSampleImage {

    pub(crate) fn setup(info: SampleImageInfo, sampler: HaSampler, allocate_info: &ImageAllocateInfo, view: &HaImageView)
        -> HaSampleImage {

        HaSampleImage {
            sampler,
            binding: info.binding(),
            entity: ImageEntity::new(&allocate_info.image, view),
            desc: allocate_info.gen_desc(),
        }
    }

    pub fn cleanup(&self, device: &HaDevice) {
        self.sampler.cleanup(device);
    }
}

impl DescriptorImageBindableTarget for HaSampleImage {

    fn binding_info(&self) -> DescriptorImageBindingInfo {

        DescriptorImageBindingInfo {
            content: self.binding.clone(),
            sampler: &self.sampler,
            dst_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            entity: &self.entity,
        }
    }
}

impl ImageInstance for HaSampleImage {}

impl ImageCopiable for HaSampleImage {

    fn copy_info(&self) -> ImageCopyInfo {

        use image::utils::image_subrange_to_layers;
        let subrange_layers = image_subrange_to_layers(&self.desc.subrange);

        ImageCopyInfo::new(&self.entity, subrange_layers, self.desc.current_layout, self.desc.dimension)
    }
}
