
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::device::GsDevice;
use crate::core::swapchain::GsChain;

use crate::pipeline::pass::render::GsRenderPass;
use crate::pipeline::pass::attachment::RenderAttachement;
use crate::pipeline::pass::subpass::{ RenderSubpass, AttachmentType };
use crate::pipeline::pass::dependency::RenderDependency;
use crate::pipeline::pass::framebuffer::FramebufferBuilder;
use crate::pipeline::error::{ RenderPassError, PipelineError };

use crate::image::instance::depth::GsDepthStencilAttachment;

use crate::types::vkuint;

use std::ptr;

pub struct RenderPassBuilder {

    device: GsDevice,
    chain : GsChain,

    attachments : Vec<RenderAttachement>,
    subpasses   : Vec<RenderSubpass>,
    dependencies: Vec<RenderDependency>,

    depth: Option<vk::ImageView>,
}

impl RenderPassBuilder {

    pub fn new(device: &GsDevice, chain: &GsChain) -> RenderPassBuilder {

        RenderPassBuilder {
            device: device.clone(),
            chain : chain.clone(),

            attachments  : vec!(),
            subpasses    : vec!(),
            dependencies : vec!(),

            depth: None,
        }
    }

    /// create a new subpass in the RenderPass, return the index of the subpass.
    pub fn new_subpass(&mut self) -> vkuint {

        // TODO: Currently only support Graphics Subpass.
        let subpass = RenderSubpass::new(vk::PipelineBindPoint::GRAPHICS);

        let subpass_index = self.subpasses.len();
        self.subpasses.push(subpass);

        subpass_index as _
    }

    /// create a attachment and set its reference to subpass, return the index of this attachment.
    pub fn add_attachemnt(&mut self, attachment: RenderAttachement, subpass_index: vkuint) -> usize {

        let attachment_ref = vk::AttachmentReference {
            attachment: self.attachments.len() as _,
            layout: attachment.layout,
        };

        match attachment.attach_type {
            | AttachmentType::Input => {
                self.subpasses[subpass_index as usize].add_input(attachment_ref)
            },
            | AttachmentType::Color => {
                self.subpasses[subpass_index as usize].add_color(attachment_ref)
            },
            | AttachmentType::Resolve => {
                self.subpasses[subpass_index as usize].add_resolve(attachment_ref)
            },
            | AttachmentType::DepthStencil => {
                self.subpasses[subpass_index as usize].add_depth_stencil(attachment_ref)
            },
        }

        let attachment_index = self.attachments.len();
        self.attachments.push(attachment);

        attachment_index
    }

    pub fn set_attachment_preserve(&mut self, subpass_index: usize, attachment_index: usize) {
        self.subpasses[subpass_index].add_preserve(attachment_index as vkuint);
    }

    pub fn set_depth_attachment(&mut self, image: &GsDepthStencilAttachment) {
        self.depth = Some(image.entity.view)
    }

    pub fn add_dependenty(&mut self, dependency: RenderDependency) {
        self.dependencies.push(dependency);
    }

    pub fn build(self) -> Result<GsRenderPass, PipelineError> {

        let clear_values = self.attachments.iter()
            .map(|a| a.clear_value).collect();
        let attachments: Vec<vk::AttachmentDescription> = self.attachments.into_iter()
            .map(|a| a.build()).collect();
        let subpasses: Vec<vk::SubpassDescription> = self.subpasses.into_iter()
            .map(|r| r.build()).collect();
        let dependencies: Vec<vk::SubpassDependency> = self.dependencies.into_iter()
            .map(|d| d.build()).collect();

        let create_info = vk::RenderPassCreateInfo {
            s_type: vk::StructureType::RENDER_PASS_CREATE_INFO,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags: vk::RenderPassCreateFlags::empty(),
            attachment_count: attachments.len() as _,
            p_attachments   : attachments.as_ptr(),
            subpass_count   : subpasses.len() as _,
            p_subpasses     : subpasses.as_ptr(),
            dependency_count: dependencies.len() as _,
            p_dependencies  : dependencies.as_ptr(),
        };

        let handle = unsafe {
            self.device.handle.create_render_pass(&create_info, None)
                .or(Err(PipelineError::RenderPass(RenderPassError::RenderPassCreationError)))?
        };

        // generate framebuffers ---------------------------------------
        let mut framebuffers = vec![];

        for view in self.chain.views().iter() {
            let mut builder = FramebufferBuilder::new(self.chain.extent(), 1);
            builder.add_attachment(&view.handle);

            if let Some(depth) = self.depth {
                builder.add_attachment(&depth);
            }

            let framebuffer = builder.build(&self.device, handle)?;
            framebuffers.push(framebuffer);
        }
        // ------------------------------------------------------------

        let render_pass = GsRenderPass::new(handle, framebuffers, self.chain.extent(), clear_values);
        Ok(render_pass)
    }
}
