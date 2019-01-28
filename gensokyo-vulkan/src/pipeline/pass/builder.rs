
use ash::vk;
use ash::version::DeviceV1_0;

use crate::core::device::GsDevice;
use crate::core::swapchain::GsChain;

use crate::pipeline::pass::render::GsRenderPass;
use crate::pipeline::pass::attachment::{ RenderAttachment, RenderAttType, AttachmentView };
use crate::pipeline::pass::subpass::{ RenderSubpass, AttachmentRawType };
use crate::pipeline::pass::dependency::RenderDependency;
use crate::pipeline::pass::framebuffer::FramebufferBuilder;

use crate::error::{ VkResult, VkError };
use crate::types::vkuint;

use std::ptr;

pub struct RenderPassBuilder {

    device: GsDevice,
    chain : GsChain,

    attachments : Vec<vk::AttachmentDescription>,
    frame_views : Vec<AttachmentView>,
    clear_values: Vec<vk::ClearValue>,

    subpasses   : Vec<RenderSubpass>,
    dependencies: Vec<RenderDependency>,
}

impl RenderPassBuilder {

    pub fn new(device: &GsDevice, chain: &GsChain) -> RenderPassBuilder {

        RenderPassBuilder {
            device: device.clone(),
            chain : chain.clone(),

            attachments : vec!(),
            frame_views : vec![],
            clear_values: vec![],
            subpasses   : vec!(),
            dependencies: vec!(),
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

    /// create a attachment and set its reference to subpass.
    pub fn add_attachment<A>(&mut self, attachment: RenderAttachment<A>, subpass_index: vkuint)
        where
            A: RenderAttType {

        // `attachment_index` is the index of attachments used in a specific render pass.
        let attachment_index = self.attachments.len();
        let attachment_ref = attachment.reference(attachment_index);

        match A::RAW_TYPE {
            | AttachmentRawType::Input => {
                self.subpasses[subpass_index as usize].inputs.push(attachment_ref)
            },
            | AttachmentRawType::Color => {
                self.subpasses[subpass_index as usize].colors.push(attachment_ref)
            },
            | AttachmentRawType::Resolve => {
                self.subpasses[subpass_index as usize].resolves.push(attachment_ref)
            },
            | AttachmentRawType::DepthStencil => {
                self.subpasses[subpass_index as usize].depth_stencils.push(attachment_ref)
            },
        }

        let (attachment, frame_view, clear_value) = attachment.take();
        self.attachments.push(attachment);
        self.frame_views.push(frame_view);
        self.clear_values.push(clear_value);
    }

    pub fn add_dependency(&mut self, dependency: RenderDependency) {
        self.dependencies.push(dependency);
    }

    pub fn build(self) -> VkResult<GsRenderPass> {

        let subpasses: Vec<vk::SubpassDescription> = self.subpasses.iter()
            .map(|r| r.build()).collect();
        let dependencies: Vec<vk::SubpassDependency> = self.dependencies.into_iter()
            .map(|d| d.take()).collect();

        let render_pass_ci = vk::RenderPassCreateInfo {
            s_type: vk::StructureType::RENDER_PASS_CREATE_INFO,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags: vk::RenderPassCreateFlags::empty(),
            attachment_count: self.attachments.len() as _,
            p_attachments   : self.attachments.as_ptr(),
            subpass_count   : subpasses.len() as _,
            p_subpasses     : subpasses.as_ptr(),
            dependency_count: dependencies.len() as _,
            p_dependencies  : dependencies.as_ptr(),
        };

        let handle = unsafe {
            self.device.handle.create_render_pass(&render_pass_ci, None)
                .or(Err(VkError::create("Render Pass")))?
        };

        // generate framebuffers ---------------------------------------

        let framebuffer_count = self.chain.image_count();
        let mut framebuffers = Vec::with_capacity(framebuffer_count);

        for i in 0..framebuffer_count {
            let mut builder = FramebufferBuilder::new(self.chain.dimension(), 1);

            for frame_view in self.frame_views.iter() {
                match frame_view {
                    | AttachmentView::Present => builder.add_attachment(self.chain.view_at(i)),
                    | AttachmentView::DepthStencil(view) => builder.add_attachment(view.clone()),
                }
            }

            let framebuffer = builder.build(&self.device, handle)?;
            framebuffers.push(framebuffer);
        }
        // ------------------------------------------------------------

        let render_pass = GsRenderPass::new(handle, framebuffers, self.chain.dimension(), self.clear_values);
        Ok(render_pass)
    }
}
