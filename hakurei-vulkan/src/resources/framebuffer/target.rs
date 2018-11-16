
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

pub struct HaFramebuffer {

    pub(crate) handle: vk::Framebuffer,
}

impl HaFramebuffer {

    pub fn cleanup(&self, device: &HaDevice) {
        unsafe {
            device.handle.destroy_framebuffer(self.handle, None);
        }
    }
}
