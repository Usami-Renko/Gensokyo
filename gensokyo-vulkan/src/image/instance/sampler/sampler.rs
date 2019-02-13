
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::GsDevice;

use crate::descriptor::binding::DescriptorMeta;
use crate::descriptor::binding::{ DescriptorBindingImgInfo, DescriptorBindingImgTgt };

pub struct GsSampler {

    pub(crate) handle : vk::Sampler,
    pub(crate) descriptor: DescriptorMeta,
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

impl DescriptorBindingImgTgt for GsSampler {

    fn binding_info(&self) -> DescriptorBindingImgInfo {

        DescriptorBindingImgInfo {
            meta           : self.descriptor.clone(),
            sampler_handle : self.handle,
            dst_layout     : vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
            view_handle    : vk::ImageView::null(),
        }
    }
}

// ----------------------------------------------------------------------------------------
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct GsSamplerMirror(pub(super) vk::Sampler);

impl GsSamplerMirror {

    pub(crate) fn discard(&self, device: &GsDevice) {
        unsafe {
            device.logic.handle.destroy_sampler(self.0, None);
        }
    }
}
// ----------------------------------------------------------------------------------------
