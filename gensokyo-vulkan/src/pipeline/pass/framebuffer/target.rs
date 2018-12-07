
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::device::GsDevice;

pub struct GsFramebuffer {

    pub(crate) handle: vk::Framebuffer,
}

impl GsFramebuffer {

    pub fn cleanup(&self, device: &GsDevice) {
        unsafe {
            device.handle.destroy_framebuffer(self.handle, None);
        }
    }
}
