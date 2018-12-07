
use ash::vk;

use image::view::GsImageView;
use image::entity::ImageEntity;
use image::traits::{ ImageInstance, ImageCopiable };
use image::utils::ImageCopyInfo;
use image::instance::ImageInstanceInfoDesc;
use image::instance::depth::DepthStencilAttachmentInfo;
use image::allocator::ImageAllocateInfo;

use pipeline::pass::{ RenderAttachement, RenderAttachementPrefab };

#[derive(Debug, Default)]
pub struct GsDepthStencilAttachment {

    pub(crate) entity: ImageEntity,
    pub(crate) format: vk::Format,

    desc: ImageInstanceInfoDesc,
}

impl GsDepthStencilAttachment {

    pub(crate) fn setup(info: DepthStencilAttachmentInfo, allocate_info: &ImageAllocateInfo, view: &GsImageView) -> GsDepthStencilAttachment {

        GsDepthStencilAttachment {
            entity: ImageEntity::new(&allocate_info.image, view),
            format: info.format(),
            desc: allocate_info.gen_desc(),
        }
    }

    pub fn to_subpass_attachment(&self) -> RenderAttachement {
        RenderAttachement::setup(RenderAttachementPrefab::DepthAttachment, self.format)
    }
}

impl ImageInstance for GsDepthStencilAttachment {}

impl ImageCopiable for GsDepthStencilAttachment {

    fn copy_info(&self) -> ImageCopyInfo {

        use image::utils::image_subrange_to_layers;
        let subrange_layers = image_subrange_to_layers(&self.desc.subrange);

        ImageCopyInfo::new(&self.entity, subrange_layers, self.desc.current_layout, self.desc.dimension)
    }
}
