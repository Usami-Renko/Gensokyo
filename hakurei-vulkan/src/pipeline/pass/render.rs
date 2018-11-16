
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::framebuffer::HaFramebuffer;
use utils::types::vkDimension2D;

pub struct HaRenderPass {

    pub(crate) handle: vk::RenderPass,

    pub(crate) clear_values: Vec<vk::ClearValue>,
    pub framebuffers: Vec<HaFramebuffer>,

    pub framebuffer_extent: vkDimension2D,
}

impl HaRenderPass {

    pub fn uninitialize() -> HaRenderPass {

        HaRenderPass {
            handle: vk::RenderPass::null(),
            clear_values: vec![],
            framebuffers: vec![],

            framebuffer_extent: vkDimension2D { width: 0, height: 0 },
        }
    }

    pub(crate) fn cleanup(&self, device: &HaDevice) {

        unsafe {
            device.handle.destroy_render_pass(self.handle, None);
        }

        self.framebuffers.iter()
            .for_each(|f| f.cleanup(device));
    }
}
