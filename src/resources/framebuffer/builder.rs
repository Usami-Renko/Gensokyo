
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use resources::framebuffer::HaFramebuffer;
use resources::image::HaImageView;
use resources::error::FramebufferError;
use pipeline::pass::render_pass::HaRenderPass;

use utility::dimension::BufferDimension;

use std::ptr;

pub struct FramebufferBuilder<'i> {

    attachments: Vec<&'i HaImageView>,
    dimension: BufferDimension,
}

impl<'i> FramebufferBuilder<'i> {

    pub fn init(dimension: &BufferDimension) -> FramebufferBuilder<'i> {
        FramebufferBuilder {
            attachments: vec![],
            dimension: dimension.clone(),
        }
    }

    pub fn build(&self, device: &HaLogicalDevice, render_pass: &HaRenderPass) -> Result<HaFramebuffer, FramebufferError> {
        let attachments: Vec<vk::ImageView> = self.attachments.iter()
            .map(|a| a.handle).collect();

        let info = vk::FramebufferCreateInfo {
            s_type: vk::StructureType::FramebufferCreateInfo,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.0.82
            flags: vk::FramebufferCreateFlags::empty(),
            render_pass: render_pass.handle,
            attachment_count: attachments.len() as uint32_t,
            p_attachments:    attachments.as_ptr(),
            width : self.dimension.extent.width,
            height: self.dimension.extent.height,
            layers: self.dimension.layers,
        };

        let handle = unsafe {
            device.handle.create_framebuffer(&info, None)
                .or(Err(FramebufferError::FramebufferCreationError))?
        };

        let framebuffer = HaFramebuffer::new(handle);
        Ok(framebuffer)
    }

    pub fn set_dimension(&mut self, dimension: BufferDimension) -> &mut FramebufferBuilder<'i> {
        self.dimension = dimension;
        self
    }
    pub fn add_attachment(&mut self, attachment: &'i HaImageView) -> &mut FramebufferBuilder<'i> {
        self.attachments.push(attachment);
        self
    }
}


