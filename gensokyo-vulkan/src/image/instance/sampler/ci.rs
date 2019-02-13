
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::GsDevice;

use crate::image::instance::sampler::sampler::GsSampler;

use crate::descriptor::binding::DescriptorMeta;
use crate::descriptor::{ GsDescriptorType, ImageDescriptorType };

use crate::types::{ vkuint, vkfloat, VK_TRUE, VK_FALSE };
use crate::error::{ VkResult, VkError };

#[derive(Debug, Clone)]
pub struct SamplerCI {

    descriptor: Option<DescriptorMeta>,
    ci: vk::SamplerCreateInfo,
}

impl GsSampler {

    pub fn new() -> SamplerCI {

        SamplerCI {
            descriptor: None,
            ..SamplerCI::inner_default()
        }
    }

    pub fn new_descriptor(binding: vkuint) -> SamplerCI {

        let descriptor = DescriptorMeta {
            binding,
            descriptor_type: GsDescriptorType::Image(ImageDescriptorType::Sampler),
        };
        
        SamplerCI {
            descriptor: Some(descriptor),
            ..SamplerCI::inner_default()
        }
    }
}

impl SamplerCI {

    fn inner_default() -> SamplerCI {

        use std::ptr;

        SamplerCI {
            descriptor: None,
            ci: vk::SamplerCreateInfo {
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
            },
        }
    }

    pub(crate) fn build(self, device: &GsDevice) -> VkResult<GsSampler> {

        let descriptor = self.descriptor
            .ok_or(VkError::other("Descriptor binding must be set before creating vk::Sampler."))?;

        let handle = unsafe {
            device.logic.handle.create_sampler(&self.ci, None)
                .or(Err(VkError::create("Sampler")))?
        };

        let sampler = GsSampler { handle, descriptor };
        Ok(sampler)
    }

    pub(crate) fn reset_descriptor(&mut self, descriptor: DescriptorMeta) {
        self.descriptor = Some(descriptor);
    }

    pub(crate) fn reset_ci(&mut self, sampler_ci: SamplerCI) {
        self.ci = sampler_ci.ci;
    }

    pub(crate) fn take_ci(self) -> vk::SamplerCreateInfo {
        self.ci
    }
}

impl SamplerCI {

    /// `mag` specifies the magnification filter to apply to lookups.
    ///
    /// `min` specifies the minification filter to apply to lookups.
    pub fn filter(mut self, mag: vk::Filter, min: vk::Filter) -> SamplerCI {

        self.ci.mag_filter = mag;
        self.ci.min_filter = min;
        self
    }

    /// `mode` specifies the mipmap filter to apply to lookups.
    ///
    /// `u`, `v` and `w` specifies the addressing mode for outside [0..1] range for U, V, W coordinate.
    pub fn mipmap(mut self, mode: vk::SamplerMipmapMode, u: vk::SamplerAddressMode, v: vk::SamplerAddressMode, w: vk::SamplerAddressMode) -> SamplerCI {

        self.ci.mipmap_mode = mode;
        self.ci.address_mode_u = u;
        self.ci.address_mode_v = v;
        self.ci.address_mode_w = w;

        self
    }

    /// `mip_bias` is the bias to be added to mipmap LOD (level-of-detail) calculation and bias provided by image sampling functions in SPIR-V.
    ///
    /// `min` used to clamp the minimum computed LOD value, as described in the Level-of-Detail Operation section.
    ///
    /// `max` used to clamp the maximum computed LOD value, as described in the Level-of-Detail Operation section.
    pub fn lod(mut self, mip_bias: vkfloat, min: vkfloat, max: vkfloat) -> SamplerCI {

        self.ci.mip_lod_bias = mip_bias;
        self.ci.min_lod = min;
        self.ci.max_lod = max;

        self
    }

    /// This function needs to enable an physical feature named 'sampler_anisotropy'.
    ///
    /// `max` is the anisotropy value clamp used by the sampler.
    ///
    /// If `max` is None, anisotropy will be disabled.
    pub fn anisotropy(mut self, max: Option<vkfloat>) -> SamplerCI {

        if let Some(max) = max {
            self.ci.anisotropy_enable = VK_TRUE;
            self.ci.max_anisotropy = max;
        } else {
            self.ci.anisotropy_enable = VK_FALSE;
        }

        self
    }

    /// `op` specifies the comparison function to apply to fetched data before filtering
    /// as described in the Depth Compare Operation section.
    ///
    /// Set `op` to some value to enable comparison.
    ///
    /// If `op` is None, the compare function will be disabled.
    pub fn compare_op(mut self, op: Option<vk::CompareOp>) -> SamplerCI {

        if let Some(op) = op  {
            self.ci.compare_enable = VK_TRUE;
            self.ci.compare_op = op;
        } else {
            self.ci.compare_enable = VK_FALSE;
        }

        self
    }

    /// `border_color` specifies the predefined border color to use.
    pub fn border_color(mut self, color: vk::BorderColor) -> SamplerCI {

        self.ci.border_color = color;
        self
    }

    /// `unnormalize_coordinates_enable` controls whether to use unnormalized or normalized texel coordinates to address texels of the image.
    ///
    /// When set to true, the range of the image coordinates used to lookup the texel is in the range of zero
    /// to the image dimensions for x, y and z.
    ///
    /// When set to false, the range of image coordinates is zero to one.
    pub fn unnormalize_coordinates_enable(mut self, enable: bool) -> SamplerCI {

        if enable {
            self.ci.unnormalized_coordinates = VK_TRUE;
        } else {
            self.ci.unnormalized_coordinates = VK_FALSE;
        }

        self
    }
}
