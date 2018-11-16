
use vk::core::device::HaDevice;

use vk::resources::image::{ HaImageView, ImageViewItem };
use vk::resources::image::ImageLayout;
use vk::resources::image::{ ImageCopiable, ImageCopyInfo };
use vk::resources::image::HaSampler;
use vk::resources::descriptor::{ DescriptorImageBindingInfo, DescriptorImageBindableTarget };
use vk::resources::descriptor::{ HaDescriptorType, ImageDescriptorType, DescriptorBindingContent };
use vk::utils::types::vkint;

use resources::image::sample::SampleImageInfo;
use resources::image::ImageBranchInfoDesc;
use resources::allocator::image::ImageAllocateInfo;

pub struct HaSampleImage {

    sampler: HaSampler,
    binding: DescriptorBindingContent,

    item: ImageViewItem,
    desc: ImageBranchInfoDesc,
}

impl HaSampleImage {

    pub fn uninitialize() -> HaSampleImage {

        HaSampleImage {
            sampler: HaSampler::unitialize(),
            binding: DescriptorBindingContent {
                binding: 0, count: 0,
                descriptor_type: HaDescriptorType::Image(ImageDescriptorType::CombinedImageSampler)
            },
            item: ImageViewItem::unset(),
            desc: ImageBranchInfoDesc::unset(),
        }
    }

    pub(crate) fn setup(info: SampleImageInfo, sampler: HaSampler, allocate_info: &ImageAllocateInfo, view: &HaImageView)
        -> HaSampleImage {

        HaSampleImage {
            sampler,
            binding: info.binding,
            item: ImageViewItem::new(&allocate_info.image, view),
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
            dst_layout: ImageLayout::ShaderReadOnlyOptimal,
            item: &self.item,
        }
    }
}

impl ImageCopiable for HaSampleImage {

    fn copy_info(&self) -> ImageCopyInfo {

        ImageCopyInfo::new(&self.item, &self.desc.sub_range, self.desc.current_layout, self.desc.dimension)
    }
}
