
use ash::vk;
use ash::vk::{ uint32_t, c_float };

use core::device::HaDevice;

use resources::image::{ ImageType, ImageViewType, ImageTiling, ImageUsageFlag, ImageLayout, ImageAspectFlag };
use resources::image::{ ImageDescInfo, ImageViewDescInfo, ImageViewItem };
use resources::image::{HaImageDescAbs, HaImageViewDescAbs, HaImageBranchAbs};
use resources::image::ImagePipelineStage;
use resources::image::{ HaSamplerDescAbs, HaSampler, SamplerDescInfo };
use resources::descriptor::{ DescriptorImageBindingInfo, ImageDescriptorType, DescriptorImageBindableTarget };
use resources::error::{ DescriptorError, DescriptorResourceError };

use pipeline::state::SampleCountType;
use utility::marker::VulkanEnum;

pub struct SampleImageInfo {

    pub(crate) pipeline_stage: ImagePipelineStage,

    pub(crate) binding: uint32_t,
    pub(crate) count  : uint32_t,

    pub(crate) image_desc  : ImageDescInfo,
    pub(crate) view_desc   : ImageViewDescInfo,
    pub(crate) sampler_desc: SamplerDescInfo,
}

impl SampleImageInfo {

    pub fn new(binding: uint32_t, count: uint32_t, pipeline_stage: ImagePipelineStage) -> SampleImageInfo {

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
            pipeline_stage, binding, count, image_desc, view_desc, sampler_desc,
        }
    }
}

pub struct HaSampleImage {

    sampler: HaSampler,
    binding: uint32_t,
    count  : uint32_t,

    item   : ImageViewItem,
}

impl HaSampleImage {

    pub fn uninitialize() -> HaSampleImage {
        HaSampleImage {
            sampler: HaSampler::unitialize(),
            binding: 0,
            count  : 0,

            item: ImageViewItem::from_unallocate(0),
        }
    }

    pub(crate) fn setup(sampler: HaSampler, binding: uint32_t, count: uint32_t, index: usize) -> HaSampleImage {

        HaSampleImage {
            sampler, binding, count,
            item: ImageViewItem::from_unallocate(index),
        }
    }

    pub fn cleanup(&self, device: &HaDevice) {
        self.sampler.cleanup(device);
    }
}

impl DescriptorImageBindableTarget for HaSampleImage {

    fn binding_info(&self) -> Result<DescriptorImageBindingInfo, DescriptorError> {

        if let Some(_) = self.item.handles {
            let info = DescriptorImageBindingInfo {
                type_    : ImageDescriptorType::CombinedImageSampler,
                binding  : self.binding,
                count    : self.count,
                sampler  : self.sampler.handle,
                dst_layout: ImageLayout::ShaderReadOnlyOptimal,
                view_item: self.item.clone(),
            };

            Ok(info)
        } else {

            Err(DescriptorError::Resource(DescriptorResourceError::ImageNotAllocated))?
        }
    }
}

impl_image_branch_abs!(HaSampleImage);
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
