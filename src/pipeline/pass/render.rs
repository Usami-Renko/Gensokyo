
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use resources::framebuffer::HaFramebuffer;

pub struct HaRenderPass {

    pub(crate) handle: vk::RenderPass,
    pub(crate) clear_values: Vec<vk::ClearValue>,
    pub(crate) framebuffers: Vec<HaFramebuffer>,

    pub(crate) framebuffer_extent: vk::Extent2D,
}

impl HaRenderPass {

    pub fn uninitialize() -> HaRenderPass {
        HaRenderPass {
            handle: vk::RenderPass::null(),
            clear_values: vec![],
            framebuffers: vec![],

            framebuffer_extent: vk::Extent2D { width: 0, height: 0 },
        }
    }

    pub(crate) fn cleanup(&self, device: &HaLogicalDevice) {
        unsafe {
            device.handle.destroy_render_pass(self.handle, None);
        }

        self.framebuffers.iter().for_each(|f| f.cleanup(device));
    }
}
