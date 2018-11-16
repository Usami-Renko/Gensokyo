
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::framebuffer::target::HaFramebuffer;
use resources::error::FramebufferError;

use utils::types::{ vkint, vkDimension2D };

use std::ptr;

pub struct FramebufferBuilder {

    attachments: Vec<vk::ImageView>,

    // dimension
    extent: vkDimension2D,
    layers: vkint,
}

impl FramebufferBuilder {

    pub fn init(extent: vkDimension2D, layers: vkint) -> FramebufferBuilder {

        FramebufferBuilder {
            attachments: vec![],
            extent, layers,
        }
    }

    pub(crate) fn build(&self, device: &HaDevice, render_pass: vk::RenderPass) -> Result<HaFramebuffer, FramebufferError> {

        let info = vk::FramebufferCreateInfo {
            s_type: vk::StructureType::FramebufferCreateInfo,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags: vk::FramebufferCreateFlags::empty(),
            render_pass,
            attachment_count: self.attachments.len() as vkint,
            p_attachments   : self.attachments.as_ptr(),
            width : self.extent.width,
            height: self.extent.height,
            layers: self.layers,
        };

        let handle = unsafe {
            device.handle.create_framebuffer(&info, None)
                .or(Err(FramebufferError::FramebufferCreationError))?
        };

        let framebuffer = HaFramebuffer {
            handle,
        };
        Ok(framebuffer)
    }

    pub(crate) fn add_attachment(&mut self, attachment: &vk::ImageView) -> &mut FramebufferBuilder {

        self.attachments.push(attachment.clone());
        self
    }
}
