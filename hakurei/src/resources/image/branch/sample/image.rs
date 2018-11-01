
use ash::vk;
use ash::vk::uint32_t;

use core::device::HaDevice;

use resources::image::ImageViewItem;
use resources::image::ImageLayout;
use resources::image::{ HaSampler, SampleImageInfo };
use resources::image::{ ImageCopiable, ImageCopyInfo };
use resources::image::ImageBranchInfoDesc;
use resources::descriptor::{ DescriptorImageBindingInfo, ImageDescriptorType, DescriptorImageBindableTarget };
use resources::allocator::ImageAllocateInfo;

use utility::marker::VulkanEnum;

pub struct HaSampleImage {

    sampler: HaSampler,
    binding: uint32_t,
    count  : uint32_t,

    item: ImageViewItem,
    desc: ImageBranchInfoDesc,
}

impl HaSampleImage {

    pub fn uninitialize() -> HaSampleImage {
        HaSampleImage {
            sampler: HaSampler::unitialize(),
            binding: 0,
            count  : 0,

            item: ImageViewItem::unset(),
            desc: ImageBranchInfoDesc::unset(),
        }
    }

    pub(crate) fn setup(info: SampleImageInfo, sampler: HaSampler, allocate_info: &ImageAllocateInfo, view_handle: vk::ImageView) -> HaSampleImage {

        HaSampleImage {
            sampler,
            binding: info.binding,
            count  : info.count,
            item: ImageViewItem::new(allocate_info.image.handle, view_handle),
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
            type_  : ImageDescriptorType::CombinedImageSampler,
            binding: self.binding,
            count  : self.count,
            sampler: self.sampler.handle,
            dst_layout: ImageLayout::ShaderReadOnlyOptimal,
            item: self.item.clone(),
        }
    }
}

impl ImageCopiable for HaSampleImage {

    fn copy_info(&self) -> ImageCopyInfo {
        ImageCopyInfo {
            handle: self.item.image_handle,
            layout: self.desc.current_layout.value(),
            extent: self.desc.dimension,
            sub_resource: self.desc.gen_sublayers(),
        }
    }
}
