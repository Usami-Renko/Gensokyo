
use ash::vk;

use crate::core::GsDevice;

use crate::image::target::GsImage;
use crate::image::storage::{ ImageStorageInfo, ImageSource };
use crate::image::enums::{ ImageInstanceType, DepthStencilImageFormat };
use crate::image::instance::base::GsBackendImage;
use crate::image::instance::depth::{ GsDSAttachment, IDepthStencilImg };
use crate::image::instance::api::ImageCIInheritApi;
use crate::image::instance::traits::ImageCISpecificApi;
use crate::image::allocator::ImageAllotCI;

use crate::error::{ VkResult, VkError };
use crate::types::{ vkDim2D, vkDim3D };

/// Depth Stencil Attachment Create Info.
pub struct DSAttachmentCI {

    backend: GsBackendImage,
}

impl GsDSAttachment {

    pub fn new(dimension: vkDim2D, format: DepthStencilImageFormat) -> DSAttachmentCI {

        let storage = ImageStorageInfo {
            source: ImageSource::NoSource,
            format   : format.into(),
            dimension: vkDim3D {
                width : dimension.width,
                height: dimension.height,
                depth : 1, // Depth Stencil Attachment usually use 1 layer.
            },
        };

        let mut backend = GsBackendImage::from(storage);

        backend.image_ci.property.image_type = vk::ImageType::TYPE_2D;
        backend.image_ci.property.tiling     = vk::ImageTiling::OPTIMAL;
        backend.image_ci.property.usages     = vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT;

        backend.view_ci.view_type = vk::ImageViewType::TYPE_2D;
        backend.view_ci.subrange.0.aspect_mask = format.aspect_mask();

        DSAttachmentCI { backend }
    }
}

impl ImageCISpecificApi for DSAttachmentCI {
    type IConveyor = IDepthStencilImg;

    fn check_physical_support(&self, device: &GsDevice) -> VkResult<()> {

        let is_depth_support = match self.backend.image_ci.property.tiling {
            | vk::ImageTiling::LINEAR => {
                device.phys.formats.query_format_linear(self.backend.image_ci.specific.format, vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT)?
            },
            | vk::ImageTiling::OPTIMAL => {
                device.phys.formats.query_format_optimal(self.backend.image_ci.specific.format, vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT)?
            },
            | _ => {
                unreachable!("vk::ImageTiling should be LINEAR or OPTIMAL.")
            },
        };

        if is_depth_support {
            Ok(())
        } else {
            Err(VkError::other(format!("vk::Format: {:?} is not support for DepthStencil Attachment", self.backend.image_ci.specific.format)))
        }
    }

    fn refactor(self, _: &GsDevice, image: GsImage) -> VkResult<(ImageAllotCI, Self::IConveyor)> {

        let idsi = IDepthStencilImg::new(self.backend.image_ci.specific.format, self.backend.view_ci.subrange.0.aspect_mask);

        let allot_cis = ImageAllotCI::new(
            ImageInstanceType::DepthStencilAttachment,
            image, self.backend
        );

        Ok((allot_cis, idsi))
    }
}

impl ImageCIInheritApi for DSAttachmentCI {

    fn backend(&self) -> &GsBackendImage {
        &self.backend
    }

    fn backend_mut(&mut self) -> &mut GsBackendImage {
        &mut self.backend
    }
}
