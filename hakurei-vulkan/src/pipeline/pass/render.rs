
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use pipeline::pass::framebuffer::HaFramebuffer;
use types::{ vkuint, vkDim2D };

use std::ptr;

pub struct HaRenderPass {

    pub(crate) handle: vk::RenderPass,

    clear_values: Vec<vk::ClearValue>,
    framebuffers: Vec<HaFramebuffer>,
    framebuffer_extent: vkDim2D,
}

impl HaRenderPass {

    pub(crate) fn new(handle: vk::RenderPass, framebuffers: Vec<HaFramebuffer>, dimension: vkDim2D, clear_values: Vec<vk::ClearValue>) -> HaRenderPass {

        HaRenderPass {
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
            clear_value_count: self.clear_values.len() as vkuint,
            p_clear_values   : self.clear_values.as_ptr(),
        }
    }

    pub fn frame_count(&self) -> usize {
        self.framebuffers.len()
    }

    pub fn cleanup(&self, device: &HaDevice) {

        unsafe {
            device.handle.destroy_render_pass(self.handle, None);
        }

        self.framebuffers.iter()
            .for_each(|f| f.cleanup(device));
    }
}
