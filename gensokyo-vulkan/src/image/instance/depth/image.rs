
use crate::image::entity::ImageEntity;
use crate::image::traits::{ ImageInstance, ImageCopiable };
use crate::image::utils::{ ImageCopyInfo, ImageCopySubrange };
use crate::image::instance::desc::ImageInstanceInfoDesc;

use crate::pipeline::pass::{ RenderAttachmentCI, DepthStencil };
use crate::types::format::GsFormat;

pub struct GsDSAttachment {

    idsi: IDepthStencilImg,

    entity: ImageEntity,
    desc: ImageInstanceInfoDesc,
}

pub struct IDepthStencilImg {

    format: GsFormat,
}

impl ImageInstance<IDepthStencilImg> for GsDSAttachment {

    fn build(idsi: IDepthStencilImg, entity: ImageEntity, desc: ImageInstanceInfoDesc) -> GsDSAttachment {
        GsDSAttachment { idsi, entity, desc }
    }
}

impl GsDSAttachment {

    pub fn attachment(&self) -> RenderAttachmentCI<DepthStencil> {

        let frame_view = DepthStencil(self.entity.view.clone());
        RenderAttachmentCI::create(frame_view, self.idsi.format)
    }
}

impl IDepthStencilImg {

    pub(super) fn new(format: GsFormat) -> IDepthStencilImg {
        IDepthStencilImg { format }
    }
}

impl ImageCopiable for GsDSAttachment {

    fn copy_range(&self, subrange: ImageCopySubrange) -> ImageCopyInfo {

        ImageCopyInfo {
            handle: self.entity.image,
            layout: self.desc.current_layout,
            extent: self.desc.dimension,
            sub_resource_layers: subrange,
        }
    }
}
