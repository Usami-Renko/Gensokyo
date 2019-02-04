
use ash::vk;

use crate::image::entity::ImageEntity;
use crate::image::copy::{ ImageCopiable, ImageFullCopyInfo };
use crate::image::instance::sampler::GsSamplerMirror;
use crate::image::instance::traits::{ ImageInstance, IImageConveyor, ImageInstanceInfoDesc };
use crate::image::format::GsImageFormat;

use crate::pipeline::pass::{ RenderAttachmentCI, DepthStencil };
use crate::types::{ vkuint, vkDim3D };

pub struct GsDSAttachment {

    idsi: IDepthStencilImg,

    entity: ImageEntity,
    desc: ImageInstanceInfoDesc,
}

pub struct IDepthStencilImg {

    aspect: vk::ImageAspectFlags,
    format: GsImageFormat,
}

impl ImageInstance<IDepthStencilImg> for GsDSAttachment {

    fn build(idsi: IDepthStencilImg, entity: ImageEntity, desc: ImageInstanceInfoDesc) -> GsDSAttachment {
        GsDSAttachment { idsi, entity, desc }
    }
}

impl GsDSAttachment {

    pub fn attachment(&self) -> RenderAttachmentCI<DepthStencil> {

        let frame_view = DepthStencil(self.entity.view.clone());
        RenderAttachmentCI::create(frame_view, self.idsi.format.clone().into())
    }
}

impl IDepthStencilImg {

    pub(super) fn new(format: GsImageFormat, aspect: vk::ImageAspectFlags) -> IDepthStencilImg {
        IDepthStencilImg { format, aspect }
    }
}

impl ImageCopiable for GsDSAttachment {

    fn full_copy_mipmap(&self, copy_mip_level: vkuint) -> ImageFullCopyInfo {

        use std::cmp::max;

        ImageFullCopyInfo {
            handle: self.entity.image,
            layout: self.desc.current_layout,
            extent: vkDim3D {
                width  : max(self.desc.dimension.width  >> copy_mip_level, 1),
                height : max(self.desc.dimension.height >> copy_mip_level, 1),
                depth  : 1,
            },
            sub_resource_layers: vk::ImageSubresourceLayers {
                aspect_mask      : self.idsi.aspect,
                mip_level        : copy_mip_level,
                base_array_layer : 0,
                layer_count      : 1,
            },
        }
    }
}

impl IImageConveyor for IDepthStencilImg {

    fn sampler_mirror(&self) -> Option<GsSamplerMirror> {
        None
    }
}
