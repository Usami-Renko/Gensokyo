
mod builder;

pub use self::builder::FramebufferBuilder;

use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

pub struct HaFramebuffer {

    pub(crate) handle: vk::Framebuffer,
}

impl HaFramebuffer {

    pub fn new(handle: vk::Framebuffer) -> HaFramebuffer {
        HaFramebuffer { handle }
    }

    pub fn cleanup(&self, device: &HaLogicalDevice) {
        unsafe {
            device.handle.destroy_framebuffer(self.handle, None);
        }
    }
}
