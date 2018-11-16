
use vk::resources::image::{ HaImageView, ImageViewItem };
use vk::resources::image::{ ImageCopiable, ImageCopyInfo };

use resources::image::ImageBranchInfoDesc;
use resources::image::depth::DepthStencilAttachmentInfo;
use resources::allocator::image::ImageAllocateInfo;

use vk::utils::types::{ vkformat, vkint };
use vk::utils::format::VKFormat;

pub struct HaDepthStencilAttachment {

    format: vkformat,

    item: ImageViewItem,
    desc: ImageBranchInfoDesc,
}

impl HaDepthStencilAttachment {

    pub fn uninitialize() -> HaDepthStencilAttachment {

        HaDepthStencilAttachment {
            format: VKFormat::D32Sfloat,
            item: ImageViewItem::unset(),
            desc: ImageBranchInfoDesc::unset(),
        }
    }

    pub(crate) fn setup(info: DepthStencilAttachmentInfo, allocate_info: &ImageAllocateInfo, view: &HaImageView) -> HaDepthStencilAttachment {

        HaDepthStencilAttachment {
            format: info.format,
            item: ImageViewItem::new(&allocate_info.image, view),
            desc: allocate_info.gen_desc(),
        }
    }

    pub fn get_format(&self) -> vkformat {
        self.format
    }

    fn get_item(&self) -> &ImageViewItem {
        &self.item
    }
}

impl ImageCopiable for HaDepthStencilAttachment {

    fn copy_info(&self) -> ImageCopyInfo {

        ImageCopyInfo::new(&self.item, &self.desc.sub_range, self.desc.current_layout, self.desc.dimension)
    }
}
