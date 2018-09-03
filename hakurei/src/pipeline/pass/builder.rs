
use ash::vk;
use ash::vk::uint32_t;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;
use core::swapchain::chain::HaSwapchain;

use pipeline::pass::render::HaRenderPass;
use pipeline::pass::attachment::RenderAttachement;
use pipeline::pass::subpass::{ RenderSubpass, AttachmentType, SubpassType };
use pipeline::pass::dependency::RenderDependency;
use pipeline::error::{ RenderPassError, PipelineError };

use resources::framebuffer::{ HaFramebuffer, FramebufferBuilder };

use utility::dimension::BufferDimension;
use utility::marker::VulkanEnum;
use constant::swapchain::FRAMEBUFFER_LAYERS;

use std::ptr;

pub struct RenderPassBuilder {

    attachments : Vec<RenderAttachement>,
    subpasses   : Vec<RenderSubpass>,
    dependencies: Vec<RenderDependency>,

    clear_values: Vec<vk::ClearValue>,
}

impl RenderPassBuilder {

    pub fn new() -> RenderPassBuilder {
        RenderPassBuilder {
            attachments : vec![],
            subpasses   : vec![],
            dependencies: vec![],

            clear_values:  vec![
                vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 1.0],
                    }
                }
            ],
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

    pub fn build(&self, device: &HaLogicalDevice, swapchain: &HaSwapchain) -> Result<HaRenderPass, PipelineError> {

        let attachments : Vec<vk::AttachmentDescription> = self.attachments.iter().map(|a| a.desc()).collect();
        let subpasses   : Vec<vk::SubpassDescription>    = self.subpasses.iter().map(|r| r.desc()).collect();
        let dependencies: Vec<vk::SubpassDependency>     = self.dependencies.iter().map(|d| d.desc()).collect();

        let create_info = vk::RenderPassCreateInfo {
            s_type: vk::StructureType::RenderPassCreateInfo,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.0.82.
            flags: vk::RenderPassCreateFlags::empty(),
            attachment_count: attachments.len() as uint32_t,
            p_attachments   : attachments.as_ptr(),
            subpass_count   : subpasses.len() as uint32_t,
            p_subpasses     : subpasses.as_ptr(),
            dependency_count: dependencies.len() as uint32_t,
            p_dependencies  : dependencies.as_ptr(),
        };

        let handle = unsafe {
            device.handle.create_render_pass(&create_info, None)
                .or(Err(PipelineError::RenderPass(RenderPassError::RenderPassCreationError)))?
        };

        let framebuffers = generate_framebuffers(device, swapchain, handle)
            .map_err(|e| PipelineError::RenderPass(e))?;

        let render_pass = HaRenderPass {
            handle,
            clear_values: self.clear_values.clone(),

            framebuffers,
            framebuffer_extent: swapchain.extent,
        };
        Ok(render_pass)
    }
}

fn generate_framebuffers(device: &HaLogicalDevice, swapchain: &HaSwapchain, render_pass: vk::RenderPass)
    -> Result<Vec<HaFramebuffer>, RenderPassError> {

    let dimension = BufferDimension::init(swapchain.extent, FRAMEBUFFER_LAYERS);

    let mut framebuffers = vec![];
    for view in swapchain.views.iter() {
        let mut builder = FramebufferBuilder::init(&dimension);
        builder.add_attachment(view);
        let framebuffer = builder.build(device, render_pass)
            .map_err(|e| RenderPassError::Framebuffer(e))?;
        framebuffers.push(framebuffer);
    }

    Ok(framebuffers)
}

