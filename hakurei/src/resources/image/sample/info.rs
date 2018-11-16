
use vk::core::device::HaDevice;
use vk::core::physical::HaPhyDevice;
use vk::core::device::SharingMode;

use vk::pipeline::state::multisample::SampleCountType;
use vk::resources::image::{ HaImage, HaSampler };
use vk::resources::image::{ ImageDescInfo, ImageViewDescInfo };
use vk::resources::image::{
    ImageType, ImageViewType,
    ImageTiling, ImageLayout, Filter, CompareOp, BorderColor, ComponentSwizzle,
    SamplerMipmapMode, SamplerAddressMode
};
use vk::resources::image::{ ImageUsageFlag, ImageAspectFlag };
use vk::resources::image::HaSamplerDescAbs;
use vk::resources::image::SamplerDescInfo;
use vk::resources::descriptor::{ DescriptorBindingContent, HaDescriptorType, ImageDescriptorType };
use vk::resources::error::ImageError;
use vk::utils::types::{ vkint, vkfloat, VK_TRUE, vkformat };

use resources::allocator::image::ImageAllocateInfo;
use resources::image::io::ImageStorageInfo;
use resources::image::traits::{ HaImageDescAbs, HaImageViewDescAbs };
use resources::image::enums::{ ImagePipelineStage };
use resources::image::{ ImageBranchInfoAbs, ImageBranchType };

use resources::image::io::ImageLoadConfig;

use std::path::PathBuf;

pub struct SampleImageInfo {

    path: PathBuf,
    pipeline_stage: ImagePipelineStage,

    image_desc  : ImageDescInfo,
    view_desc   : ImageViewDescInfo,
    sampler_desc: SamplerDescInfo,

    allocate_index: Option<usize>,

    pub(super) binding: DescriptorBindingContent,
}

impl SampleImageInfo {

    pub fn new(binding: vkint, count: vkint, path: impl Into<PathBuf>, pipeline_stage: ImagePipelineStage) -> SampleImageInfo {

        let image_desc = ImageDescInfo::init(
            // TODO: Currently HaSampleImage only support 2D image.
            ImageType::Type2d,
            ImageTiling::Optimal,
            &[
                ImageUsageFlag::TransferDstBit,
                ImageUsageFlag::SampledBit,
            ],
            ImageLayout::Undefined
        );

        let view_desc = ImageViewDescInfo::init(
            ImageViewType::Type2d,
            &[ImageAspectFlag::ColorBit]
        );

        let sampler_desc = SamplerDescInfo::default();

        let binding = DescriptorBindingContent {
            binding, count,
            descriptor_type: HaDescriptorType::Image(ImageDescriptorType::CombinedImageSampler)
        };

        SampleImageInfo {
            path: path.into(),
            pipeline_stage, sampler_desc, image_desc, view_desc, binding,
            allocate_index: None,
        }
    }

    pub(crate) fn gen_sample(&self, device: &HaDevice) -> Result<HaSampler, ImageError> {
        HaSampler::new(device, &self.sampler_desc)
    }
}

impl ImageBranchInfoAbs for SampleImageInfo {

    fn storage(&mut self, _physical: &HaPhyDevice, config: &ImageLoadConfig) -> Result<ImageStorageInfo, ImageError> {
        ImageStorageInfo::from_load2d(&self.path, config)
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
            ImageBranchType::SampleImage(self.pipeline_stage.clone()),
            storage, image,
            self.image_desc.clone(),
            self.view_desc.clone()
        )
    }
}

impl_image_desc_info_abs!(SampleImageInfo);

impl HaSamplerDescAbs for SampleImageInfo {

    fn set_filter(&mut self, mag: Filter, min: Filter) {
        self.sampler_desc.mag_filter = mag;
        self.sampler_desc.min_filter = min;
    }
    fn set_mipmap(&mut self, mode: SamplerMipmapMode, u: SamplerAddressMode, v: SamplerAddressMode, w: SamplerAddressMode) {
        self.sampler_desc.mipmap_mode = mode;
        self.sampler_desc.address_u = u;
        self.sampler_desc.address_v = v;
        self.sampler_desc.address_w = w;
    }
    fn set_lod(&mut self, min: vkfloat, max: vkfloat, mip_bias: vkfloat) {
        self.sampler_desc.min_lod = min;
        self.sampler_desc.max_lod = max;
        self.sampler_desc.mip_lod_bias = mip_bias;
    }
    fn set_anisotropy(&mut self, max: vkfloat) {
        self.sampler_desc.anisotropy_enable = VK_TRUE;
        self.sampler_desc.max_anisotropy = max;
    }
    fn set_compare_op(&mut self, op: CompareOp) {
        self.sampler_desc.compare_enable = VK_TRUE;
        self.sampler_desc.compare_op = op;
    }
    fn set_border_color(&mut self, color: BorderColor) {
        self.sampler_desc.border_color = color;
    }
    fn set_unnormalize_coordinates_enable(&mut self, enable: bool) {
        self.sampler_desc.unnormalize_coordinates = if enable { 1 } else { 0 };
    }
}
