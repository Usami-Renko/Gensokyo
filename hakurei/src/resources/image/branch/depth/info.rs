
use ash::vk;
use ash::vk::uint32_t;

use config::resources::ImageLoadConfig;
use core::physical::HaPhyDevice;

use resources::image::HaImage;
use resources::image::{ ImageType, ImageViewType, ImageTiling, ImageUsageFlag, ImageLayout, ImageAspectFlag };
use resources::image::{ DepthImageUsage, ImagePipelineStage, DepthStencilImageFormat };
use resources::image::{ ImageDescInfo, ImageViewDescInfo, HaImageDescAbs, HaImageViewDescAbs };
use resources::image::{ ImageBranchInfoAbs, ImageBranchType, ImageStorageInfo };
use resources::allocator::ImageAllocateInfo;
use resources::error::ImageError;
use pipeline::state::SampleCountType;

use utility::dimension::Dimension2D;
use utility::marker::VulkanEnum;

pub struct DepthStencilImageInfo {

    dimension: Dimension2D,
    usage: DepthImageUsage,

    pub(crate) format: vk::Format,
    pub(crate) binding: uint32_t,
    pub(crate) count  : uint32_t,

    image_desc: ImageDescInfo,
    view_desc : ImageViewDescInfo,

    allocate_index: Option<usize>,
}

impl DepthStencilImageInfo {

    pub fn new_attachment(dimension: Dimension2D) -> DepthStencilImageInfo {
        DepthStencilImageInfo::new(0, 0, DepthImageUsage::Attachment, dimension)
    }

    pub fn new_image(binding: uint32_t, count: uint32_t, stage: ImagePipelineStage, format: DepthStencilImageFormat, dimension: Dimension2D)
        -> DepthStencilImageInfo {
        DepthStencilImageInfo::new(binding, count, DepthImageUsage::ShaderRead(format, stage), dimension)
    }

    fn new(binding: uint32_t, count: uint32_t, usage: DepthImageUsage, dimension: Dimension2D) -> DepthStencilImageInfo {

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

        DepthStencilImageInfo {
            usage, binding, count, image_desc, view_desc, dimension,
            format: vk::Format::Undefined,
            allocate_index: None,
        }
    }
}

impl ImageBranchInfoAbs for DepthStencilImageInfo {

    fn storage(&mut self, physical: &HaPhyDevice, _config: &ImageLoadConfig) -> Result<ImageStorageInfo, ImageError> {

        let storage = ImageStorageInfo::from_unload(self.dimension, self.usage.dst_format(physical))?;
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
            ImageBranchType::DepthStencilImage(self.usage.clone()),
            storage, image,
            self.image_desc.clone(),
            self.view_desc.clone(),
        )
    }
}

impl_image_desc_info_abs!(DepthStencilImageInfo);
