
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

pub(crate) struct HaBuffer {

    pub(crate) handle : vk::Buffer,
    pub(crate) requirement : vk::MemoryRequirements,
}

impl HaBuffer {

    pub fn cleanup(&self, device: &HaLogicalDevice) {
        unsafe {
            device.handle.destroy_buffer(self.handle, None);
        }
    }
}
