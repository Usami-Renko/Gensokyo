
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::GsDevice;

use crate::image::instance::sampler::ci::SamplerCI;
use crate::image::instance::sampler::sampler::GsSamplerMirror;

use crate::descriptor::{ GsDescriptorType, ImageDescriptorType };
use crate::descriptor::binding::DescriptorArrayMeta;
use crate::descriptor::binding::{ DescriptorBindingImgArrayInfo, DescriptorBindingImgArrayTgt, ImgArrayBinding };

use crate::types::vkuint;
use crate::error::{ VkResult, VkError };

use std::iter::Map;
use std::slice::Iter;

pub struct SamplerArrayCI {

    descriptor: DescriptorArrayMeta,
    sampler_cis: Vec<SamplerCI>,
}

impl SamplerArrayCI {

    pub fn add_sampler(&mut self, sampler: SamplerCI) {

        self.sampler_cis.push(sampler);
        self.descriptor.count += 1;
    }

    pub(crate) fn build(self, device: &GsDevice) -> VkResult<GsSamplerArray> {

        let mut handles = Vec::with_capacity(self.sampler_cis.len());
        for sampler in self.sampler_cis.into_iter() {
           let handle = unsafe {
               device.logic.handle.create_sampler(&sampler.take_ci(), None)
                   .or(Err(VkError::create("Sampler")))?
           };
            handles.push(handle);
        }

        let result = GsSamplerArray {
            handles,
            descriptor: self.descriptor,
        };
        Ok(result)
    }
}

pub struct GsSamplerArray {

    handles: Vec<vk::Sampler>,
    descriptor: DescriptorArrayMeta,
}

impl GsSamplerArray {

    pub fn new(binding: vkuint) -> SamplerArrayCI {

        SamplerArrayCI {
            descriptor: DescriptorArrayMeta {
                binding,
                count: 0,
                descriptor_type: GsDescriptorType::Image(ImageDescriptorType::Sampler),
            },
            sampler_cis: Vec::new(),
        }
    }

    pub(crate) fn mirrors(&self) -> Map<Iter<vk::Sampler>, fn(&vk::Sampler) -> GsSamplerMirror> {
        self.handles.iter().map(|handle| GsSamplerMirror(handle.clone()))
    }

    // pub(crate) fn destroy(&self, device: &GsDevice) {
    //
    //     self.handles.iter().for_each(|&handle| {
    //         unsafe {
    //             device.logic.handle.destroy_sampler(handle, None)
    //         }
    //     })
    // }
}

impl DescriptorBindingImgArrayTgt for GsSamplerArray {

    fn binding_info(&self) -> DescriptorBindingImgArrayInfo {

        DescriptorBindingImgArrayInfo {
            meta: self.descriptor.clone(),
            content: ImgArrayBinding::MultiSamplers {
                samplers   : self.handles.clone(),
                view_handle: vk::ImageView::null(),
                dst_layout : vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            },
        }
    }
}
