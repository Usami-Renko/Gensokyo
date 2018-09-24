
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaDevice;

use resources::framebuffer::HaFramebuffer;
use resources::image::{ HaImageView, ImageViewItem };
use resources::repository::HaImageRepository;
use resources::error::FramebufferError;

use utility::dimension::BufferDimension;

use std::ptr;

pub struct FramebufferBuilder<'i> {

    attachments: Vec<&'i HaImageView>,
    dimension  : BufferDimension,
}

impl<'i> FramebufferBuilder<'i> {

    pub fn init(dimension: &BufferDimension) -> FramebufferBuilder<'i> {
        FramebufferBuilder {
            attachments: vec![],
            dimension: dimension.clone(),
        }
    }

    pub fn build(&self, device: &HaDevice, render_pass: vk::RenderPass) -> Result<HaFramebuffer, FramebufferError> {
        let attachments: Vec<vk::ImageView> = self.attachments.iter()
            .map(|a| a.handle).collect();

        let info = vk::FramebufferCreateInfo {
            s_type: vk::StructureType::FramebufferCreateInfo,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags: vk::FramebufferCreateFlags::empty(),
            render_pass,
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

        let framebuffer = HaFramebuffer {
            handle,
        };
        Ok(framebuffer)
    }

    #[allow(dead_code)]
    pub fn set_dimension(&mut self, dimension: BufferDimension) -> &mut FramebufferBuilder<'i> {
        self.dimension = dimension;
        self
    }
    pub(crate) fn add_attachment_inner(&mut self, attachment: &'i HaImageView) -> &mut FramebufferBuilder<'i> {
        self.attachments.push(attachment);
        self
    }
    #[allow(dead_code)]
    pub fn add_attachment(&mut self, repository: &'i HaImageRepository, item: &ImageViewItem) -> &mut FramebufferBuilder<'i> {
        let view = repository.view_at(item);
        self.attachments.push(view);
        self
    }
}
