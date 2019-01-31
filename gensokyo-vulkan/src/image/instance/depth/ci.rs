
use ash::vk;

use crate::core::GsDevice;

use crate::image::target::{ GsImage, ImageTgtCI, ImagePropertyCI, ImageSpecificCI };
use crate::image::view::{ ImageViewCI, ImageSubRange };
use crate::image::storage::{ ImageStorageInfo, ImageSource };
use crate::image::enums::{ ImageInstanceType, DepthStencilImageFormat };
use crate::image::instance::depth::{ GsDSAttachment, IDepthStencilImg };
use crate::image::instance::traits::{ ImageCIAbstract, ImageTgtCIAbs, ImageViewCIAbs };
use crate::image::allocator::ImageAllotCI;

use crate::error::{ VkResult, VkError };
use crate::types::{ vkuint, vkDim2D, vkDim3D };

/// Depth Stencil Attachment Create Info.
pub struct DSAttachmentCI {

    image_ci: ImageTgtCI,
    view_ci : ImageViewCI,
}

impl GsDSAttachment {

    pub fn new(dimension: vkDim2D, format: DepthStencilImageFormat) -> DSAttachmentCI {

        let mut property = ImagePropertyCI::default();
        property.image_type = vk::ImageType::TYPE_2D;
        property.tiling = vk::ImageTiling::OPTIMAL;
        property.usages = vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT;

        let mut specific = ImageSpecificCI::default();
        specific.format = format.into();
        specific.dimension = vkDim3D {
            width  : dimension.width,
            height : dimension.height,
            depth  : 1,
        };

        DSAttachmentCI {
            image_ci: ImageTgtCI { property, specific },
            view_ci : ImageViewCI::new(vk::ImageViewType::TYPE_2D, format.aspect_mask()),
        }
    }
}

impl ImageCIAbstract<IDepthStencilImg> for DSAttachmentCI {

    fn check_physical_support(&self, device: &GsDevice) -> VkResult<()> {

        let is_depth_support = match self.image_ci.property.tiling {
            | vk::ImageTiling::LINEAR => {
                device.phys.formats.query_format_linear(self.image_ci.specific.format, vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT)?
            },
            | vk::ImageTiling::OPTIMAL => {
                device.phys.formats.query_format_optimal(self.image_ci.specific.format, vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT)?
            },
            | _ => {
                unreachable!("vk::ImageTiling should be LINEAR or OPTIMAL.")
            },
        };

        if is_depth_support {
            Ok(())
        } else {
            Err(VkError::other(format!("vk::Format: {:?} is not support for DepthStencil Attachment", self.image_ci.specific.format)))
        }
    }

    fn build(&self, device: &GsDevice) -> VkResult<GsImage> {
        self.image_ci.build(device)
    }

    fn refactor(self, _: &GsDevice, image: GsImage) -> VkResult<(ImageAllotCI, IDepthStencilImg)> {

        let storage = ImageStorageInfo {
            source: ImageSource::NoSource,
            format   : self.image_ci.specific.format,
            dimension: self.image_ci.specific.dimension,
        };

        let idsi = IDepthStencilImg::new(self.image_ci.specific.format);

        let allot_cis = ImageAllotCI::new(
            ImageInstanceType::DepthStencilAttachment,
            storage, image, self.image_ci, self.view_ci
        );

        Ok((allot_cis, idsi))
    }
}

impl_image_desc_info_abs!(DSAttachmentCI);
