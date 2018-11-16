
use vk::core::physical::HaPhyDevice;
use vk::core::device::SharingMode;

use vk::pipeline::state::multisample::SampleCountType;
use vk::resources::image::{ HaImage, ImageDescInfo };
use vk::resources::image::ImageViewDescInfo;
use vk::resources::image::{ ImageType, ImageViewType, ImageTiling, ImageLayout, ComponentSwizzle };
use vk::resources::image::{ ImageUsageFlag, ImageAspectFlag };
use vk::resources::error::ImageError;

use vk::utils::types::{ vkint, vkformat, vkDimension2D };
use vk::utils::format::VKFormat;

use resources::image::io::ImageStorageInfo;
use resources::image::enums::{ ImageBranchType, ImagePipelineStage, DepthStencilImageFormat };

use resources::image::traits::{ HaImageDescAbs, HaImageViewDescAbs, ImageBranchInfoAbs };
use resources::image::infos::ImageBranchInfoDesc;

use resources::allocator::image::ImageAllocateInfo;

use resources::image::io::ImageLoadConfig;

pub struct DepthStencilAttachmentInfo {

    dimension: vkDimension2D,

    image_desc: ImageDescInfo,
    view_desc : ImageViewDescInfo,

    allocate_index: Option<usize>,

    pub(super) format: vkformat,
}

impl DepthStencilAttachmentInfo {

    pub fn new(dimension: vkDimension2D) -> DepthStencilAttachmentInfo {

        let image_desc = ImageDescInfo::init(
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

        DepthStencilAttachmentInfo {
            image_desc, view_desc, dimension,
            format: VKFormat::Undefined,
            allocate_index: None,
        }
    }
}

impl ImageBranchInfoAbs for DepthStencilAttachmentInfo {

    fn storage(&mut self, physical: &HaPhyDevice, _config: &ImageLoadConfig) -> Result<ImageStorageInfo, ImageError> {

        let storage = ImageStorageInfo::from_unload(self.dimension, physical.formats.depth_attachment_format)?;
        self.format = storage.format;
        self.view_desc.reset_depth_image_aspect_mask(self.format);

        Ok(storage)
    }

    fn view_desc(&self) -> &ImageDescInfo {
        &self.image_desc
    }

    fn allocate_index(&self) -> Option<usize> {
        self.allocate_index
    }

    fn set_allocate_index(&mut self, value: usize) {
        self.allocate_index = Some(value);
    }

    fn allocate_info(&self, image: HaImage, storage: ImageStorageInfo) -> ImageAllocateInfo {

        ImageAllocateInfo::new(
            ImageBranchType::DepthStencilAttachment,
            storage, image,
            self.image_desc.clone(),
            self.view_desc.clone(),
        )
    }
}

impl_image_desc_info_abs!(DepthStencilAttachmentInfo);
