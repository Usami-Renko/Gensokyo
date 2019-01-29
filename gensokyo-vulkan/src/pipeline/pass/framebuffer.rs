
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::GsDevice;

use crate::error::{ VkResult, VkError };
use crate::types::{ vkuint, vkDim2D };

use std::ptr;

#[derive(Clone)]
pub struct GsFramebuffer {

    pub(crate) handle: vk::Framebuffer,
}

impl GsFramebuffer {

    pub fn destroy(&self, device: &GsDevice) {
        unsafe {
            device.logic.handle.destroy_framebuffer(self.handle, None);
        }
    }
}

pub struct FramebufferBuilder {

    attachments: Vec<vk::ImageView>,

    // dimension
    extent: vkDim2D,
    layers: vkuint,
}

impl FramebufferBuilder {

    pub fn new(extent: vkDim2D, layers: vkuint) -> FramebufferBuilder {

        FramebufferBuilder {
            attachments: vec![],
            extent, layers,
        }
    }

    pub fn build(self, device: &GsDevice, render_pass: vk::RenderPass) -> VkResult<GsFramebuffer> {

        let framebuffer_ci = vk::FramebufferCreateInfo {
            s_type: vk::StructureType::FRAMEBUFFER_CREATE_INFO,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags: vk::FramebufferCreateFlags::empty(),
            render_pass,
            attachment_count: self.attachments.len() as _,
            p_attachments   : self.attachments.as_ptr(),
            width : self.extent.width,
            height: self.extent.height,
            layers: self.layers,
        };

        let handle = unsafe {
            device.logic.handle.create_framebuffer(&framebuffer_ci, None)
                .or(Err(VkError::create("FrameBuffer")))?
        };

        let framebuffer = GsFramebuffer { handle };
        Ok(framebuffer)
    }

    pub(crate) fn add_attachment(&mut self, attachment: vk::ImageView) {
        self.attachments.push(attachment);
    }
}
