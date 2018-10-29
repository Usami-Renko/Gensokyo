
use ash::vk;
use ash::vk::uint32_t;

use resources::image::ImageViewItem;
use resources::image::ImageBranchInfoDesc;
use resources::image::DepthStencilImageInfo;
use resources::image::{ ImageCopiable, ImageCopyInfo };
use resources::descriptor::{ DescriptorImageBindingInfo, DescriptorImageBindableTarget };
use resources::allocator::ImageAllocateInfo;

use utility::marker::VulkanEnum;

pub struct HaDepthStencilImage {

    format: vk::Format,

    _binding: uint32_t,
    _count  : uint32_t,

    item: ImageViewItem,
    desc: ImageBranchInfoDesc,
}

impl HaDepthStencilImage {

    pub fn uninitialize() -> HaDepthStencilImage {
        HaDepthStencilImage {

            format: vk::Format::D32Sfloat,

            _binding: 0,
            _count  : 0,

            item: ImageViewItem::unset(),
            desc: ImageBranchInfoDesc::unset(),
        }
    }

    pub(crate) fn setup(info: DepthStencilImageInfo, format: vk::Format, allocate_info: &ImageAllocateInfo, view_handle: vk::ImageView) -> HaDepthStencilImage {

        HaDepthStencilImage {
            format,
            _binding: info.binding,
            _count  : info.count,
            item: ImageViewItem::new(allocate_info.image.handle, view_handle),
            desc: allocate_info.gen_desc(),
        }
    }

    pub fn get_format(&self) -> vk::Format {
        self.format
    }

    pub(crate) fn get_item(&self) -> &ImageViewItem {
        &self.item
    }
}

impl DescriptorImageBindableTarget for HaDepthStencilImage {

    fn binding_info(&self) -> DescriptorImageBindingInfo {
        // implement binding info for DepthImageUsage::ShaderRead(DepthStencilImageFormat, ImagePipelineStage)
        unimplemented!()
    }
}

impl ImageCopiable for HaDepthStencilImage {

    fn copy_info(&self) -> ImageCopyInfo {
        ImageCopyInfo {
            handle: self.item.image_handle,
            layout: self.desc.current_layout.value(),
            extent: self.desc.dimension,
            sub_resource: self.desc.gen_sublayers(),
        }
    }
}
