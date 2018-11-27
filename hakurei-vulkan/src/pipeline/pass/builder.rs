
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaDevice;
use core::swapchain::HaSwapchain;

use pipeline::pass::render::HaRenderPass;
use pipeline::pass::attachment::RenderAttachement;
use pipeline::pass::subpass::{ RenderSubpass, AttachmentType };
use pipeline::pass::dependency::RenderDependency;
use pipeline::pass::framebuffer::{ HaFramebuffer, FramebufferBuilder };
use pipeline::error::{ RenderPassError, PipelineError };

//use resources::image::HaDepthStencilImage;

use types::vkuint;

use std::ptr;

pub struct RenderPassBuilder {

    device: HaDevice,
    attachments : Vec<RenderAttachement>,
    subpasses   : Vec<RenderSubpass>,
    dependencies: Vec<RenderDependency>,

    // TODO: Remove the following field.
    depth_handle: Option<vk::ImageView>,
}

impl RenderPassBuilder {

    pub fn new(device: &HaDevice) -> RenderPassBuilder {

        RenderPassBuilder {
            device: device.clone(),
            attachments  : vec!(),
            subpasses    : vec!(),
            dependencies : vec!(),
            depth_handle : None,
        }
    }

    /// create a new subpass in the RenderPass, return the index of the subpass.
    pub fn new_subpass(&mut self) -> vkuint {

        // TODO: Currently only support Graphics Subpass.
        let subpass = RenderSubpass::new(vk::PipelineBindPoint::GRAPHICS);

        let subpass_index = self.subpasses.len();
        self.subpasses.push(subpass);

        subpass_index as vkuint
    }

    /// create a attachment and set its reference to subpass, return the index of this attachment.
    pub fn add_attachemnt(&mut self, attachment: RenderAttachement, subpass_index: vkuint, typ: AttachmentType) -> usize {

        let attachment_ref = vk::AttachmentReference {
            attachment: self.attachments.len() as vkuint,
            layout: attachment.layout,
        };

        match typ {
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

    pub fn add_dependenty(&mut self, dependency: RenderDependency) {
        self.dependencies.push(dependency);
    }

    // TODO: Fix this function
//    pub fn set_depth_attachment(&mut self, depth_view: &HaDepthStencilImage) {
//        self.depth_handle = Some(depth_view.get_item().view_handle);
//    }

    pub fn build(self, swapchain: &HaSwapchain) -> Result<HaRenderPass, PipelineError> {

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
            attachment_count: attachments.len() as vkuint,
            p_attachments   : attachments.as_ptr(),
            subpass_count   : subpasses.len() as vkuint,
            p_subpasses     : subpasses.as_ptr(),
            dependency_count: dependencies.len() as vkuint,
            p_dependencies  : dependencies.as_ptr(),
        };

        let handle = unsafe {
            self.device.handle.create_render_pass(&create_info, None)
                .or(Err(PipelineError::RenderPass(RenderPassError::RenderPassCreationError)))?
        };

        let framebuffers = generate_framebuffers(&self.device, swapchain, handle, &self.depth_handle)
            .map_err(|e| PipelineError::RenderPass(e))?;

        let render_pass = HaRenderPass::new(handle, framebuffers, swapchain.extent(), clear_values);
        Ok(render_pass)
    }
}

// TODO: Redesign this function, since this function is for temporarily used.
fn generate_framebuffers(device: &HaDevice, swapchain: &HaSwapchain, render_pass: vk::RenderPass, depth: &Option<vk::ImageView>)
    -> Result<Vec<HaFramebuffer>, RenderPassError> {

    let mut framebuffers = vec![];

    if let Some(depth_view) = depth {

        for view in swapchain.views().iter() {
            let mut builder = FramebufferBuilder::new(swapchain.extent(), 1);
            builder.add_attachment(&view.handle);
            builder.add_attachment(depth_view);
            let framebuffer = builder.build(device, render_pass)?;
            framebuffers.push(framebuffer);
        }
    } else {

        for view in swapchain.views().iter() {
            let mut builder = FramebufferBuilder::new(swapchain.extent(), 1);
            builder.add_attachment(&view.handle);
            let framebuffer = builder.build(device, render_pass)?;
            framebuffers.push(framebuffer);
        }
    }

    Ok(framebuffers)
}
