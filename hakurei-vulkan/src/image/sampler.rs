
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use image::error::ImageError;

use types::{ vkfloat, vkbool, VK_TRUE, VK_FALSE };

use std::ptr;

pub struct HaSampler {

    pub(crate) handle: vk::Sampler,
}

impl HaSampler {

    pub fn unitialize() -> HaSampler {
        HaSampler {
            handle: vk::Sampler::null(),
        }
    }

    pub fn cleanup(&self, device: &HaDevice) {
        unsafe {
            device.handle.destroy_sampler(self.handle, None);
        }
    }
}


#[derive(Debug, Clone)]
pub struct SamplerDescInfo {

    /// mag_filter specifies the magnification filter to apply to lookups.
    mag_filter: vk::Filter,
    /// min_filter specifies the minification filter to apply to lookups.
    min_filter: vk::Filter,
    /// mipmap_mode specifies the mipmap filter to apply to lookups.
    mipmap_mode: vk::SamplerMipmapMode,
    /// address_u specifies the addressing mode for outside [0..1] range for U coordinate.
    address_u: vk::SamplerAddressMode,
    /// address_v specifies the addressing mode for outside [0..1] range for V coordinate.
    address_v: vk::SamplerAddressMode,
    /// address_w specifies the addressing mode for outside [0..1] range for W coordinate.
    address_w: vk::SamplerAddressMode,
    /// mip_lod_bias is the bias to be added to mipmap LOD (level-of-detail) calculation
    /// and bias provided by image sampling functions in SPIR-V.
    mip_lod_bias: vkfloat,
    /// set anisotropy_enable vk::VK_TRUE to enable anisotropic filtering or vk::VK_FALSE to disable it.
    ///
    /// This function needs to enable an physical feature named 'sampler_anisotropy'.
    anisotropy_enable: vkbool,
    /// max_anisotropy is the anisotropy value clamp used by the sampler when anisotropy_enable is true.
    ///
    /// If anisotropy_enable is vk::VK_FALSE, max_anisotropy is ignored.
    max_anisotropy: vkfloat,
    /// compare_enable is vk::VK_TRUE to enable comparison against a reference value during lookups, or vk::VK_FALSE otherwise.
    compare_enable: vkbool,
    /// compare_op specifies the comparison function to apply to fetched data before filtering as described in the Depth Compare Operation section.
    compare_op: vk::CompareOp,
    /// min_lod used to clamp the minimum computed LOD value, as described in the Level-of-Detail Operation section.
    min_lod: vkfloat,
    /// max_lod used to clamp the maxinum computed LOD value, as described in the Level-of-Detail Operation section.
    max_lod: vkfloat,
    /// border_color specifies the predefined border color to use.
    border_color: vk::BorderColor,
    /// unnormalize_coordinates controls whether to use unnormalized or normalized texel coordinates to address texels of the image.
    ///
    /// When set to vk::VK_TRUE, the range of the image coordinates used to lookup the texel is in the range of zero
    /// to the image dimensions for x, y and z. See specification for more requirement detail.
    ///
    /// When set to vk::VK_FALSE the range of image coordinates is zero to one.
    unnormalize_coordinates: vkbool,
}

impl SamplerDescInfo {

    pub fn new() -> SamplerDescInfo {

        Default::default()
    }

    pub fn build(&self, device: &HaDevice) -> Result<HaSampler, ImageError> {

        let info = vk::SamplerCreateInfo {
            s_type            : vk::StructureType::SAMPLER_CREATE_INFO,
            p_next            : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags             : vk::SamplerCreateFlags::empty(),
            mag_filter        : self.mag_filter,
            min_filter        : self.min_filter,
            mipmap_mode       : self.mipmap_mode,
            address_mode_u    : self.address_u,
            address_mode_v    : self.address_v,
            address_mode_w    : self.address_w,
            mip_lod_bias      : self.mip_lod_bias,
            anisotropy_enable : self.anisotropy_enable,
            max_anisotropy    : self.max_anisotropy,
            compare_enable    : self.compare_enable,
            compare_op        : self.compare_op,
            min_lod           : self.min_lod,
            max_lod           : self.max_lod,
            border_color      : self.border_color,
            unnormalized_coordinates : self.unnormalize_coordinates,
        };

        let handle = unsafe {
            device.handle.create_sampler(&info, None)
                .or(Err(ImageError::SamplerCreationError))?
        };

        let sampler = HaSampler { handle };
        Ok(sampler)
    }

    pub fn set_filter(&mut self, mag: vk::Filter, min: vk::Filter) {

        self.mag_filter = mag;
        self.min_filter = min;
    }

    pub fn set_mipmap(&mut self, mode: vk::SamplerMipmapMode, u: vk::SamplerAddressMode, v: vk::SamplerAddressMode, w: vk::SamplerAddressMode) {

        self.mipmap_mode = mode;
        self.address_u = u;
        self.address_v = v;
        self.address_w = w;
    }

    pub fn set_lod(&mut self, mip_bias: vkfloat, min: vkfloat, max: vkfloat) {

        self.mip_lod_bias = mip_bias;
        self.min_lod = min;
        self.max_lod = max;
    }

    pub fn set_anisotropy(&mut self, max: Option<vkfloat>) {

        if let Some(max) = max {
            self.anisotropy_enable = VK_TRUE;
            self.max_anisotropy = max;
        } else {
            self.anisotropy_enable = VK_FALSE;
        }
    }

    pub fn set_compare_op(&mut self, op: Option<vk::CompareOp>) {

        if let Some(op) = op  {
            self.compare_enable = VK_TRUE;
            self.compare_op = op;
        } else {
            self.compare_enable = VK_FALSE;
        }
    }

    pub fn set_border_color(&mut self, color: vk::BorderColor) {

        self.border_color = color;
    }

    pub fn set_unnormalize_coordinates_enable(&mut self, enable: bool) {

        if enable {
            self.unnormalize_coordinates = VK_TRUE;
        } else {
            self.unnormalize_coordinates = VK_FALSE;
        }
    }
}

impl Default for SamplerDescInfo {

    fn default() -> SamplerDescInfo {

        SamplerDescInfo {
            mag_filter        : vk::Filter::LINEAR,
            min_filter        : vk::Filter::LINEAR,
            mipmap_mode       : vk::SamplerMipmapMode::LINEAR,
            address_u         : vk::SamplerAddressMode::REPEAT,
            address_v         : vk::SamplerAddressMode::REPEAT,
            address_w         : vk::SamplerAddressMode::REPEAT,
            mip_lod_bias      : 0.0,
            anisotropy_enable : VK_FALSE,
            max_anisotropy    : 1.0,
            compare_enable    : VK_FALSE,
            compare_op        : vk::CompareOp::ALWAYS,
            min_lod           : 0.0,
            max_lod           : 0.0,
            border_color      : vk::BorderColor::INT_OPAQUE_BLACK,
            unnormalize_coordinates: VK_FALSE,
        }
    }
}
