
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::framebuffer::HaFramebuffer;
use resources::image::HaImageView;
use resources::error::FramebufferError;

use utility::dimension::BufferDimension;

use std::ptr;

pub struct FramebufferBuilder {

    attachments: Vec<vk::ImageView>,
    dimension  : BufferDimension,
}

impl FramebufferBuilder {

    pub fn init(dimension: &BufferDimension) -> FramebufferBuilder {
        FramebufferBuilder {
            attachments: vec![],
            dimension: dimension.clone(),
        }
    }

    pub fn build(&self, device: &HaDevice, render_pass: vk::RenderPass) -> Result<HaFramebuffer, FramebufferError> {

        let info = vk::FramebufferCreateInfo {
            s_type: vk::StructureType::FramebufferCreateInfo,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags: vk::FramebufferCreateFlags::empty(),
            render_pass,
            attachment_count: self.attachments.len() as uint32_t,
            p_attachments:    self.attachments.as_ptr(),
            width : self.dimension.extent.width,
            height: self.dimension.extent.height,
            layers: self.dimension.layers,
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

    #[allow(dead_code)]
    pub fn set_dimension(&mut self, dimension: BufferDimension) -> &mut FramebufferBuilder {
        self.dimension = dimension;
        self
    }
    pub(crate) fn add_attachment(&mut self, attachment: &vk::ImageView) -> &mut FramebufferBuilder {
        self.attachments.push(attachment.clone());
        self
    }
}
