
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::GsDevice;

use crate::pipeline::pass::framebuffer::GsFramebuffer;
use crate::types::vkDim2D;

use std::ptr;

#[derive(Clone)]
pub struct GsRenderPass {

    pub(crate) handle: vk::RenderPass,

    clear_values: Vec<vk::ClearValue>,
    framebuffers: Vec<GsFramebuffer>,
    framebuffer_extent: vkDim2D,
}

impl GsRenderPass {

    pub(crate) fn new(handle: vk::RenderPass, framebuffers: Vec<GsFramebuffer>, dimension: vkDim2D, clear_values: Vec<vk::ClearValue>) -> GsRenderPass {

        GsRenderPass {
            handle,
            framebuffers,
            framebuffer_extent: dimension,
            clear_values,
        }
    }

    pub fn begin_info(&self, framebuffer_index: usize) -> vk::RenderPassBeginInfo {

        vk::RenderPassBeginInfo {
            s_type: vk::StructureType::RENDER_PASS_BEGIN_INFO,
            p_next: ptr::null(),
            render_pass: self.handle,
            framebuffer: self.framebuffers[framebuffer_index].handle,
            render_area: vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: self.framebuffer_extent,
            },
            clear_value_count: self.clear_values.len() as _,
            p_clear_values   : self.clear_values.as_ptr(),
        }
    }

    pub fn frame_count(&self) -> usize {
        self.framebuffers.len()
    }

    pub fn destroy(&self, device: &GsDevice) {

        unsafe {
            device.logic.handle.destroy_render_pass(self.handle, None);
        }

        self.framebuffers.iter()
            .for_each(|f| f.destroy(device));
    }
}
