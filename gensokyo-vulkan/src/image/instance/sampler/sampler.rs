
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::GsDevice;

use crate::descriptor::DescriptorBindingContent;
use crate::descriptor::{ DescriptorImageBindingInfo, DescriptorImageBindableTarget };

pub struct GsSampler {

    pub(crate) handle : vk::Sampler,
    pub(crate) binding: DescriptorBindingContent,
}

impl GsSampler {

    // pub(crate) fn destroy(&self, device: &GsDevice) {
    //     unsafe {
    //         device.logic.handle.destroy_sampler(self.handle, None);
    //     }
    // }

    pub(crate) fn mirror(&self) -> GsSamplerMirror {
        GsSamplerMirror(self.handle.clone())
    }
}

impl DescriptorImageBindableTarget for GsSampler {

    fn binding_info(&self) -> DescriptorImageBindingInfo {

        DescriptorImageBindingInfo {
            content        : self.binding.clone(),
            sampler_handle : self.handle,
            dst_layout     : vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            view_handle    : vk::ImageView::null(),
        }
    }
}

// ----------------------------------------------------------------------------------------
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GsSamplerMirror(vk::Sampler);

impl GsSamplerMirror {

    pub(crate) fn destroy(&self, device: &GsDevice) {
        unsafe {
            device.logic.handle.destroy_sampler(self.0, None);
        }
    }
}
// ----------------------------------------------------------------------------------------
