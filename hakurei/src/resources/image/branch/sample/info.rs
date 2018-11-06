
use ash::vk;
use ash::vk::{ uint32_t, c_float };

use config::resources::ImageLoadConfig;
use core::device::HaDevice;
use core::physical::HaPhyDevice;

use resources::image::HaImage;
use resources::image::{ ImageDescInfo, ImageViewDescInfo, SamplerDescInfo, HaImageDescAbs, HaImageViewDescAbs };
use resources::image::{ ImageType, ImageViewType, ImageTiling, ImageLayout, ImageUsageFlag, ImageAspectFlag };
use resources::image::ImagePipelineStage;
use resources::image::{ ImageBranchInfoAbs, ImageBranchType, ImageStorageInfo };
use resources::image::{ HaSampler, HaSamplerDescAbs };
use resources::allocator::ImageAllocateInfo;
use resources::error::ImageError;
use pipeline::state::SampleCountType;

use utility::marker::VulkanEnum;

use std::path::PathBuf;

pub struct SampleImageInfo {

    path: PathBuf,
    pipeline_stage: ImagePipelineStage,

    pub(crate) binding: uint32_t,
    pub(crate) count  : uint32_t,

    image_desc  : ImageDescInfo,
    view_desc   : ImageViewDescInfo,
    sampler_desc: SamplerDescInfo,

    allocate_index: Option<usize>,
}

impl SampleImageInfo {

    pub fn new(binding: uint32_t, count: uint32_t, path: impl Into<PathBuf>, pipeline_stage: ImagePipelineStage) -> SampleImageInfo {

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

        SampleImageInfo {
            path: path.into(),
            pipeline_stage, binding, count, sampler_desc, image_desc, view_desc,
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

    fn set_filter(&mut self, mag: vk::Filter, min: vk::Filter) {
        self.sampler_desc.mag_filter = mag;
        self.sampler_desc.min_filter = min;
    }
    fn set_mipmap(&mut self, mode: vk::SamplerMipmapMode, u: vk::SamplerAddressMode, v: vk::SamplerAddressMode, w: vk::SamplerAddressMode) {
        self.sampler_desc.mipmap_mode = mode;
        self.sampler_desc.address_u = u;
        self.sampler_desc.address_v = v;
        self.sampler_desc.address_w = w;
    }
    fn set_lod(&mut self, min: c_float, max: c_float, mip_bias: c_float) {
        self.sampler_desc.min_lod = min;
        self.sampler_desc.max_lod = max;
        self.sampler_desc.mip_lod_bias = mip_bias;
    }
    fn set_anisotropy(&mut self, max: c_float) {
        self.sampler_desc.anisotropy_enable = vk::VK_TRUE;
        self.sampler_desc.max_anisotropy = max;
    }
    fn set_compare_op(&mut self, op: vk::CompareOp) {
        self.sampler_desc.compare_enable = vk::VK_TRUE;
        self.sampler_desc.compare_op = op;
    }
    fn set_border_color(&mut self, color: vk::BorderColor) {
        self.sampler_desc.border_color = color;
    }
    fn set_unnormalize_coordinates_enable(&mut self, enable: bool) {
        self.sampler_desc.unnormalize_coordinates = if enable { 1 } else { 0 };
    }
}
