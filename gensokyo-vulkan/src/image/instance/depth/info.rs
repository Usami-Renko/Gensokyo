
use ash::vk;

use crate::core::device::GsDevice;

use crate::image::target::{ GsImage, ImageDescInfo, ImagePropertyInfo, ImageSpecificInfo };
use crate::image::view::ImageViewDescInfo;
use crate::image::storage::{ ImageStorageInfo, ImageSource };
use crate::image::enums::{ ImageInstanceType, DepthStencilImageFormat };
use crate::image::instance::depth::IDepthStencilImg;
use crate::image::instance::traits::{ GsImageDescAbs, GsImageViewDescAbs, ImageInfoAbstract };
use crate::image::allocator::ImageAllotInfo;

use crate::error::VkResult;
use crate::types::{ vkuint, vkDim2D, vkDim3D };

pub struct GsDSAttachmentInfo {

    image_desc: ImageDescInfo,
    view_desc : ImageViewDescInfo,
}

impl GsDSAttachmentInfo {

    pub fn new(dimension: vkDim2D, format: DepthStencilImageFormat) -> GsDSAttachmentInfo {

        let mut property = ImagePropertyInfo::default();
        property.image_type = vk::ImageType::TYPE_2D;
        property.tiling = vk::ImageTiling::OPTIMAL;
        property.usages = vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT;

        let mut specific = ImageSpecificInfo::default();
        specific.format = format.into();
        specific.dimension = vkDim3D {
            width  : dimension.width,
            height : dimension.height,
            depth  : 1,
        };

        GsDSAttachmentInfo {
            image_desc: ImageDescInfo { property, specific },
            view_desc: ImageViewDescInfo::new(vk::ImageViewType::TYPE_2D, format.aspect_mask()),
        }
    }
}

impl ImageInfoAbstract<IDepthStencilImg> for GsDSAttachmentInfo {

    fn build(&self, device: &GsDevice) -> VkResult<GsImage> {
        self.image_desc.build(device)
    }

    fn refactor(self, _: &GsDevice, image: GsImage) -> VkResult<(ImageAllotInfo, IDepthStencilImg)> {

        let storage = ImageStorageInfo {
            source: ImageSource::NoSource,
            format   : self.image_desc.specific.format,
            dimension: self.image_desc.specific.dimension,
        };

        let idsi = IDepthStencilImg::new(self.image_desc.specific.format);

        let allot = ImageAllotInfo::new(
            ImageInstanceType::DepthStencilAttachment,
            storage, image, self.image_desc, self.view_desc
        );

        Ok((allot, idsi))
    }
}

impl_image_desc_info_abs!(GsDSAttachmentInfo);
