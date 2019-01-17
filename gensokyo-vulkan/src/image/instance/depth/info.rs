
use ash::vk;

use crate::core::device::GsDevice;

use crate::image::target::{ GsImage, ImageDescInfo, ImagePropertyInfo, ImageSpecificInfo };
use crate::image::view::ImageViewDescInfo;
use crate::image::storage::{ ImageStorageInfo, ImageSource };
use crate::image::enums::{ ImageInstanceType, DepthStencilImageFormat };
use crate::image::instance::traits::{ GsImageDescAbs, GsImageViewDescAbs, ImageInstanceInfoAbs };
use crate::image::allocator::ImageAllocateInfo;

use crate::error::VkResult;
use crate::types::{ vkuint, vkDim2D, vkDim3D };

pub struct DepthStencilAttachmentInfo {

    image_desc: ImageDescInfo,
    view_desc : ImageViewDescInfo,

    allocate_index: Option<usize>,
}

impl DepthStencilAttachmentInfo {

    pub fn new(dimension: vkDim2D, format: DepthStencilImageFormat) -> DepthStencilAttachmentInfo {

        let mut property = ImagePropertyInfo::default();
        property.image_type = vk::ImageType::TYPE_2D;
        property.tiling = vk::ImageTiling::OPTIMAL;
        property.usages = vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT;

        let mut specific = ImageSpecificInfo::default();
        specific.format = format.to_raw_format();
        specific.dimension = vkDim3D {
            width  : dimension.width,
            height : dimension.height,
            depth  : 1,
        };

        DepthStencilAttachmentInfo {
            image_desc: ImageDescInfo { property, specific },
            view_desc: ImageViewDescInfo::new(vk::ImageViewType::TYPE_2D, format.aspect_mask()),
            allocate_index: None,
        }
    }

    pub(super) fn format(&self) -> vk::Format {
        self.image_desc.specific.format
    }

    pub(crate) fn gen_storage_info(&self) -> ImageStorageInfo {

        ImageStorageInfo {
            source    : ImageSource::NoSource,
            format    : self.image_desc.specific.format,
            dimension : self.image_desc.specific.dimension,
        }
    }
}

impl ImageInstanceInfoAbs for DepthStencilAttachmentInfo {

    fn build_image(&self, device: &GsDevice) -> VkResult<GsImage> {
        self.image_desc.build(device)
    }

    fn allocate_index(&self) -> Option<usize> {
        self.allocate_index
    }

    fn set_allocate_index(&mut self, value: usize) {
        self.allocate_index = Some(value);
    }

    fn allocate_info(&self, image: GsImage, storage: ImageStorageInfo) -> ImageAllocateInfo {

        ImageAllocateInfo::new(
            ImageInstanceType::DepthStencilAttachment,
            storage, image,
            self.image_desc.clone(),
            self.view_desc.clone(),
        )
    }
}

impl_image_desc_info_abs!(DepthStencilAttachmentInfo);
