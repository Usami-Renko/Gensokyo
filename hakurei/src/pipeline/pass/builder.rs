
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaDevice;
use core::swapchain::HaSwapchain;

use pipeline::pass::render::HaRenderPass;
use pipeline::pass::attachment::RenderAttachement;
use pipeline::pass::subpass::{ RenderSubpass, AttachmentType, SubpassType };
use pipeline::pass::dependency::RenderDependency;
use pipeline::error::{ RenderPassError, PipelineError };

use resources::image::HaDepthStencilImage;
use resources::framebuffer::{ HaFramebuffer, FramebufferBuilder };

use utility::dimension::BufferDimension;
use utility::marker::VulkanEnum;

use std::ptr;

pub struct RenderPassBuilder {

    device: HaDevice,
    attachments : Vec<RenderAttachement>,
    subpasses   : Vec<RenderSubpass>,
    dependencies: Vec<RenderDependency>,

    // TODO: Remove the following two fields.
    depth_handle: Option<vk::ImageView>,
}

impl RenderPassBuilder {

    pub(crate) fn new(device: &HaDevice) -> RenderPassBuilder {
        RenderPassBuilder {
            device: device.clone(),
            attachments : vec![],
            subpasses   : vec![],
            dependencies: vec![],

            depth_handle: None,
        }
    }

    /// create a new subpass in the RenderPass, return the index of the subpass.
    pub fn new_subpass(&mut self, type_: SubpassType) -> uint32_t {
        let mut subpass = RenderSubpass::empty();
        subpass.set_bind_point(type_.bind_point());

        self.subpasses.push(subpass);
        (self.subpasses.len() - 1) as uint32_t
    }

    /// create a attachment and set its reference to subpass, return the index of this attachment.
    pub fn add_attachemnt(&mut self, attachment: RenderAttachement, subpass_index: uint32_t, type_: AttachmentType) -> usize {
        let attachment_reference = vk::AttachmentReference {
            attachment: self.attachments.len() as uint32_t,
            layout: attachment.layout.value(),
        };

        match type_ {
            | AttachmentType::Input        => self.subpasses[subpass_index as usize].add_input(attachment_reference),
            | AttachmentType::Color        => self.subpasses[subpass_index as usize].add_color(attachment_reference),
            | AttachmentType::Resolve      => self.subpasses[subpass_index as usize].add_resolve(attachment_reference),
            | AttachmentType::DepthStencil => self.subpasses[subpass_index as usize].add_depth_stencil(attachment_reference),
        }

        self.attachments.push(attachment);
        self.attachments.len() - 1
    }

    pub fn set_attachment_preserve(&mut self, subpass_index: usize, attachment_index: usize) {
        self.subpasses[subpass_index].add_preserve(attachment_index as uint32_t);
    }

    pub fn add_dependenty(&mut self, dependency: RenderDependency) {
        self.dependencies.push(dependency);
    }

    pub fn set_depth_attachment(&mut self, depth_view: &HaDepthStencilImage) {
        self.depth_handle = depth_view.get_view_handle();
    }

    pub fn build(&self, swapchain: &HaSwapchain) -> Result<HaRenderPass, PipelineError> {

        let attachments = self.attachments.iter().map(|a| a.desc()).collect::<Vec<_>>();
        let subpasses = self.subpasses.iter().map(|r| r.desc()).collect::<Vec<_>>();
        let dependencies = self.dependencies.iter().map(|d| d.desc()).collect::<Vec<_>>();

        let create_info = vk::RenderPassCreateInfo {
            s_type: vk::StructureType::RenderPassCreateInfo,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags: vk::RenderPassCreateFlags::empty(),
            attachment_count: attachments.len() as uint32_t,
            p_attachments   : attachments.as_ptr(),
            subpass_count   : subpasses.len() as uint32_t,
            p_subpasses     : subpasses.as_ptr(),
            dependency_count: dependencies.len() as uint32_t,
            p_dependencies  : dependencies.as_ptr(),
        };

        let handle = unsafe {
            self.device.handle.create_render_pass(&create_info, None)
                .or(Err(PipelineError::RenderPass(RenderPassError::RenderPassCreationError)))?
        };

        let framebuffers = generate_framebuffers(&self.device, swapchain, handle, &self.depth_handle)
            .map_err(|e| PipelineError::RenderPass(e))?;
        let clear_values = self.attachments.iter()
            .map(|a| a.clear_value).collect::<Vec<_>>();

        let render_pass = HaRenderPass {
            handle,
            clear_values,

            framebuffers,
            framebuffer_extent: swapchain.extent,
        };
        Ok(render_pass)
    }
}

// TODO: Redesign this function, since this function is for temporarily used.
fn generate_framebuffers(device: &HaDevice, swapchain: &HaSwapchain, render_pass: vk::RenderPass, depth: &Option<vk::ImageView>)
    -> Result<Vec<HaFramebuffer>, RenderPassError> {

    // TODO: Make layers property configurate
    let dimension = BufferDimension::new(swapchain.extent, 1);

    let mut framebuffers = vec![];

    if let Some(depth_view) = depth {

        for view in swapchain.views.iter() {
            let mut builder = FramebufferBuilder::init(&dimension);
            builder.add_attachment(&view.handle);
            builder.add_attachment(depth_view);
            let framebuffer = builder.build(device, render_pass)
                .map_err(|e| RenderPassError::Framebuffer(e))?;
            framebuffers.push(framebuffer);
        }
    } else {

        for view in swapchain.views.iter() {
            let mut builder = FramebufferBuilder::init(&dimension);
            builder.add_attachment(&view.handle);
            let framebuffer = builder.build(device, render_pass)
                .map_err(|e| RenderPassError::Framebuffer(e))?;
            framebuffers.push(framebuffer);
        }
    }

    Ok(framebuffers)
}
