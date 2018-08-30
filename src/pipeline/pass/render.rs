
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

pub struct HaRenderPass {

    pub(crate) handle: vk::RenderPass,
    pub(crate) clear_values: Vec<vk::ClearValue>,
}

impl HaRenderPass {

    pub fn cleanup(&self, device: &HaLogicalDevice) {
        unsafe {
            device.handle.destroy_render_pass(self.handle, None);}
    }
}
