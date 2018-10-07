
use ash::vk;
use ash::vk::{ uint32_t, c_float };

use core::device::HaDevice;

use resources::image::{ ImageType, ImageTiling, ImageUsageFlag, ImageLayout, ImageViewType };
use resources::image::{ ImageDescInfo, ImageViewDescInfo, ImageViewItem };
use resources::image::ImageAspectFlag;
use resources::image::{ HaImageDescAbs, HaImageViewDescAbs, HaImageVarietyAbs };
use resources::image::{ HaSamplerDescAbs, HaSampler, SamplerDescInfo };
use resources::descriptor::{ DescriptorImageBindingInfo, ImageDescriptorType };
use resources::error::{ ImageError, AllocatorError };

use pipeline::state::SampleCountType;

use utility::marker::VulkanEnum;

pub struct SampleImageInfo {

    pub(crate) binding: uint32_t,
    pub(crate) count  : uint32_t,

    pub(crate) image_desc  : ImageDescInfo,
    pub(crate) view_desc   : ImageViewDescInfo,
    pub(crate) sampler_desc: SamplerDescInfo,
}

impl SampleImageInfo {

    pub fn new(binding: uint32_t, count: uint32_t) -> SampleImageInfo {

        let image_desc = ImageDescInfo::init(
            // TODO: Currently HaSampleImage only support
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
            binding, count, image_desc, view_desc, sampler_desc,
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

    pub fn binding_info(&self) -> Result<DescriptorImageBindingInfo, AllocatorError> {

        if let Some(_) = self.item.handles {
            let info = DescriptorImageBindingInfo {
                binding  : self.binding,
                count    : self.count,
                type_    : ImageDescriptorType::CombinedImageSampler,
                sampler  : self.sampler.handle,
                dst_layout: ImageLayout::ShaderReadOnlyOptimal,
                view_item: self.item.clone(),
            };

            Ok(info)
        } else {

            Err(ImageError::NotYetAllocateError)?
        }
    }

    pub fn cleanup(&self, device: &HaDevice) {
        self.sampler.cleanup(device);
    }
}

impl HaImageVarietyAbs for HaSampleImage {

    fn view_index(&self) -> usize {
        self.item.view_index
    }
    fn fill_handles(&mut self, image: vk::Image, view: vk::ImageView) {
        self.item.set_handles(image, view);
    }
}

impl HaImageDescAbs for SampleImageInfo {

    fn set_tiling(&mut self, tiling: ImageTiling) {
        self.image_desc.tiling = tiling.value();
    }
    fn set_initial_layout(&mut self, layout: ImageLayout) {
        self.image_desc.initial_layout = layout.value();
    }
    fn set_samples(&mut self, count: SampleCountType, mip_levels: uint32_t, array_layers: uint32_t) {
        self.image_desc.sample_count = count.value();
        self.image_desc.mip_levels   = mip_levels;
        self.image_desc.array_layers = array_layers;
    }
    fn set_share_queues(&mut self, queue_family_indices: Vec<uint32_t>) {
        self.image_desc.sharing = vk::SharingMode::Concurrent;
        self.image_desc.queue_family_indices = queue_family_indices;
    }
}

impl HaImageViewDescAbs for SampleImageInfo {

    // image view property.
    fn set_mapping_component(&mut self, r: vk::ComponentSwizzle, g: vk::ComponentSwizzle, b: vk::ComponentSwizzle, a: vk::ComponentSwizzle) {
        self.view_desc.components = vk::ComponentMapping { r, g, b, a };
    }
    fn set_subrange(&mut self, base_mip_level: uint32_t, level_count: uint32_t, base_array_layer: uint32_t, layer_count: uint32_t) {

        self.view_desc.subrange.base_mip_level   = base_mip_level;
        self.view_desc.subrange.level_count      = level_count;
        self.view_desc.subrange.base_array_layer = base_array_layer;
        self.view_desc.subrange.layer_count      = layer_count;
    }
}

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
