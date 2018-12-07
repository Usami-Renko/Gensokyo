
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::GsDevice;

use pipeline::pass::framebuffer::target::GsFramebuffer;
use pipeline::error::RenderPassError;

use types::{ vkuint, vkDim2D };

use std::ptr;

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

    pub fn build(self, device: &GsDevice, render_pass: vk::RenderPass) -> Result<GsFramebuffer, RenderPassError> {

        let info = vk::FramebufferCreateInfo {
            s_type: vk::StructureType::FRAMEBUFFER_CREATE_INFO,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags: vk::FramebufferCreateFlags::empty(),
            render_pass,
            attachment_count: self.attachments.len() as vkuint,
            p_attachments   : self.attachments.as_ptr(),
            width : self.extent.width,
            height: self.extent.height,
            layers: self.layers,
        };

        let handle = unsafe {
            device.handle.create_framebuffer(&info, None)
                .or(Err(RenderPassError::FramebufferCreationError))?
        };

        let framebuffer = GsFramebuffer {
            handle,
        };
        Ok(framebuffer)
    }

    pub(crate) fn add_attachment(&mut self, attachment: &vk::ImageView) -> &mut FramebufferBuilder {

        self.attachments.push(attachment.clone());
        self
    }
}
