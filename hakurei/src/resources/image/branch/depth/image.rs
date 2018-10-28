
use ash::vk;
use ash::vk::uint32_t;

use resources::image::{ ImageType, ImageViewType, ImageTiling, ImageUsageFlag, ImageLayout, ImageAspectFlag };
use resources::image::{ ImageDescInfo, ImageViewDescInfo, ImageViewItem };
use resources::image::{HaImageDescAbs, HaImageViewDescAbs, HaImageBranchAbs };
use resources::image::{ DepthImageUsage, ImagePipelineStage, DepthStencilImageFormat };
use resources::descriptor::{ DescriptorImageBindingInfo, DescriptorImageBindableTarget };
use resources::error::DescriptorError;

use pipeline::state::SampleCountType;
use utility::marker::VulkanEnum;

pub struct DepthStencilImageInfo {

    pub(crate) usage: DepthImageUsage,

    pub(crate) binding: uint32_t,
    pub(crate) count  : uint32_t,

    pub(crate) image_desc  : ImageDescInfo,
    pub(crate) view_desc   : ImageViewDescInfo,
}

impl DepthStencilImageInfo {

    pub fn new_attachment() -> DepthStencilImageInfo {
        DepthStencilImageInfo::new(0, 0, DepthImageUsage::Attachment)
    }

    pub fn new_image(binding: uint32_t, count: uint32_t, stage: ImagePipelineStage, format: DepthStencilImageFormat) -> DepthStencilImageInfo {
        DepthStencilImageInfo::new(binding, count, DepthImageUsage::ShaderRead(format, stage))
    }

    fn new(binding: uint32_t, count: uint32_t, usage: DepthImageUsage) -> DepthStencilImageInfo {

        let image_desc = ImageDescInfo::init(
            // TODO: Currently HaSampleImage only support
            ImageType::Type2d,
            ImageTiling::Optimal,
            &[
                ImageUsageFlag::DepthStencilAttachmentBit,
            ],
            ImageLayout::Undefined
        );

        let view_desc = ImageViewDescInfo::init(
            ImageViewType::Type2d,
            &[ImageAspectFlag::DepthBit]
        );

        DepthStencilImageInfo {
            usage, binding, count, image_desc, view_desc,
        }
    }
}

pub struct HaDepthStencilImage {

    format: vk::Format,

    _binding: uint32_t,
    _count  : uint32_t,

    item   : ImageViewItem,
}

impl HaDepthStencilImage {

    pub fn uninitialize() -> HaDepthStencilImage {
        HaDepthStencilImage {
            _binding: 0,
            _count  : 0,

            item: ImageViewItem::from_unallocate(0),
            format: vk::Format::D32Sfloat,
        }
    }

    pub(crate) fn setup(binding: uint32_t, count: uint32_t, index: usize, format: vk::Format) -> HaDepthStencilImage {

        HaDepthStencilImage {
            _binding: binding, _count: count,format,
            item: ImageViewItem::from_unallocate(index),
        }
    }

    pub fn get_format(&self) -> vk::Format {
        self.format
    }

    pub(crate) fn get_view_handle(&self) -> Option<vk::ImageView> {
        self.item.get_view_handle()
    }
}

impl DescriptorImageBindableTarget for HaDepthStencilImage {

    fn binding_info(&self) -> Result<DescriptorImageBindingInfo, DescriptorError> {
        // implement binding info for DepthImageUsage::ShaderRead(DepthStencilImageFormat, ImagePipelineStage)
        unimplemented!()
    }
}


impl_image_branch_abs!(HaDepthStencilImage);
impl_image_desc_info_abs!(DepthStencilImageInfo);