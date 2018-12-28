
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::device::GsDevice;

use crate::image::error::ImageError;

use crate::types::{ vkfloat, VK_TRUE, VK_FALSE };

use std::ptr;

pub struct GsSampler {

    pub(crate) handle: vk::Sampler,
}

impl GsSampler {

    pub fn unitialize() -> GsSampler {
        GsSampler {
            handle: vk::Sampler::null(),
        }
    }

    pub fn destroy(&self, device: &GsDevice) {
        unsafe {
            device.handle.destroy_sampler(self.handle, None);
        }
    }
}


#[derive(Debug, Clone)]
pub struct SamplerDescInfo(vk::SamplerCreateInfo);

impl SamplerDescInfo {

    pub fn new() -> SamplerDescInfo {

        Default::default()
    }

    /// `mag` specifies the magnification filter to apply to lookups.
    ///
    /// `min` specifies the minification filter to apply to lookups.
    pub fn filter(&mut self, mag: vk::Filter, min: vk::Filter) {

        self.0.mag_filter = mag;
        self.0.min_filter = min;
    }

    /// `mode` specifies the mipmap filter to apply to lookups.
    ///
    /// `u`, `v` and `w` specifies the addressing mode for outside [0..1] range for U, V, W coordinate.
    pub fn mipmap(&mut self, mode: vk::SamplerMipmapMode, u: vk::SamplerAddressMode, v: vk::SamplerAddressMode, w: vk::SamplerAddressMode) {

        self.0.mipmap_mode = mode;
        self.0.address_mode_u = u;
        self.0.address_mode_v = v;
        self.0.address_mode_w = w;
    }

    /// `mip_bias` is the bias to be added to mipmap LOD (level-of-detail) calculation and bias provided by image sampling functions in SPIR-V.
    ///
    /// `min` used to clamp the minimum computed LOD value, as described in the Level-of-Detail Operation section.
    ///
    /// `max` used to clamp the maxinum computed LOD value, as described in the Level-of-Detail Operation section.
    pub fn lod(&mut self, mip_bias: vkfloat, min: vkfloat, max: vkfloat) {

        self.0.mip_lod_bias = mip_bias;
        self.0.min_lod = min;
        self.0.max_lod = max;
    }

    /// This function needs to enable an physical feature named 'sampler_anisotropy'.
    ///
    /// `max` is the anisotropy value clamp used by the sampler.
    ///
    /// If `max` is None, anisotropy will be disabled.
    pub fn anisotropy(&mut self, max: Option<vkfloat>) {

        if let Some(max) = max {
            self.0.anisotropy_enable = VK_TRUE;
            self.0.max_anisotropy = max;
        } else {
            self.0.anisotropy_enable = VK_FALSE;
        }
    }

    /// `op` specifies the comparison function to apply to fetched data before filtering
    /// as described in the Depth Compare Operation section.
    ///
    /// Set `op` to some value to enable comparison.
    ///
    /// If `op` is None, the compare function will be disabled.
    pub fn compare_op(&mut self, op: Option<vk::CompareOp>) {

        if let Some(op) = op  {
            self.0.compare_enable = VK_TRUE;
            self.0.compare_op = op;
        } else {
            self.0.compare_enable = VK_FALSE;
        }
    }

    /// `border_color` specifies the predefined border color to use.
    pub fn border_color(&mut self, color: vk::BorderColor) {

        self.0.border_color = color;
    }

    /// `unnormalize_coordinates_enable` controls whether to use unnormalized or normalized texel coordinates to address texels of the image.
    ///
    /// When set to true, the range of the image coordinates used to lookup the texel is in the range of zero
    /// to the image dimensions for x, y and z.
    ///
    /// When set to false, the range of image coordinates is zero to one.
    pub fn unnormalize_coordinates_enable(&mut self, enable: bool) {

        if enable {
            self.0.unnormalized_coordinates = VK_TRUE;
        } else {
            self.0.unnormalized_coordinates = VK_FALSE;
        }
    }

    pub fn build(&self, device: &GsDevice) -> Result<GsSampler, ImageError> {

        let handle = unsafe {
            device.handle.create_sampler(&self.0, None)
                .or(Err(ImageError::SamplerCreationError))?
        };

        let sampler = GsSampler { handle };
        Ok(sampler)
    }
}

impl Default for SamplerDescInfo {

    fn default() -> SamplerDescInfo {

        let info = vk::SamplerCreateInfo {
            s_type            : vk::StructureType::SAMPLER_CREATE_INFO,
            p_next            : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags             : vk::SamplerCreateFlags::empty(),
            mag_filter        : vk::Filter::LINEAR,
            min_filter        : vk::Filter::LINEAR,
            mipmap_mode       : vk::SamplerMipmapMode::LINEAR,
            address_mode_u    : vk::SamplerAddressMode::REPEAT,
            address_mode_v    : vk::SamplerAddressMode::REPEAT,
            address_mode_w    : vk::SamplerAddressMode::REPEAT,
            mip_lod_bias      : 0.0,
            anisotropy_enable : VK_FALSE,
            max_anisotropy    : 1.0,
            compare_enable    : VK_FALSE,
            compare_op        : vk::CompareOp::ALWAYS,
            min_lod           : 0.0,
            max_lod           : 0.0,
            border_color      : vk::BorderColor::INT_OPAQUE_BLACK,
            unnormalized_coordinates : VK_FALSE,
        };

        SamplerDescInfo(info)
    }
}
