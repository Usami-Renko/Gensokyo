
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

pub struct HaImageView {

    pub(crate) handle: vk::ImageView,
}

impl HaImageView {

    pub fn cleanup(&self, device: &HaLogicalDevice) {
        unsafe {
            device.handle.destroy_image_view(self.handle, None);
        }
    }
}
