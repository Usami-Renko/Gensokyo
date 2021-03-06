
use ash::vk;

use crate::types::vkuint;

use std::ptr;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AttachmentRawType {

    #[allow(dead_code)]
    Input,
    Color,
    #[allow(dead_code)]
    Resolve,
    DepthStencil,
}

#[derive(Debug, Default)]
pub struct RenderSubpass {

    /// bind_point specifies whether this is a compute or graphics subpass.
    bind_point: vk::PipelineBindPoint,
    /// inputs lists which of the render pass’s attachments can be read in the fragment shader stage during the subpass, and what layout each attachment will be in during the subpass.
    ///
    /// Each element of the array corresponds to an input attachment unit number in the shader.
    ///
    /// i.e. if the shader declares an input variable layout(input_attachment_index=X, set=Y, binding=Z) then it uses the attachment provided in pInputAttachments[X].
    ///
    /// Input attachments must also be bound to the pipeline with a descriptor set, with the input attachment descriptor written in the location (set=Y, binding=Z).
    ///
    /// Fragment shaders can use subpass input variables to access the contents of an input attachment at the fragment’s (x, y, layer) framebuffer coordinates.
    pub(super) inputs: Vec<vk::AttachmentReference>,
    /// colors lists which of the render pass’s attachments will be used as color attachments in the subpass, and what layout each attachment will be in during the subpass.
    ///
    /// Each element of the array corresponds to a fragment shader output location.
    ///
    /// i.e. if the shader declared an output variable layout(location=X) then it uses the attachment provided in pColorAttachments[X].
    pub(super) colors: Vec<vk::AttachmentReference>,
    /// resolves lists which of the render pass’s attachments are resolved to at the end of the subpass, and what layout each attachment will be in during the multisample resolve operation.
    ///
    /// If pResolveAttachments is not NULL, each of its elements corresponds to a color attachment (the element in pColorAttachments at the same index), and a multisample resolve operation is defined for each attachment.
    ///
    /// At the end of each subpass, multisample resolve operations read the subpass’s color attachments, and resolve the samples for each pixel to the same pixel location in the corresponding resolve attachments, unless the resolve attachment index is VK_ATTACHMENT_UNUSED.
    ///
    /// If the first use of an attachment in a render pass is as a resolve attachment, then the loadOp is effectively ignored as the resolve is guaranteed to overwrite all pixels in the render area.
    pub(super) resolves: Vec<vk::AttachmentReference>,
    /// depth_stencils lists which attachment will be used for depth/stencil data and the layout it will be in during the subpass.
    ///
    /// Setting the attachment index to VK_ATTACHMENT_UNUSED or leaving this pointer as NULL indicates that no depth/stencil attachment will be used in the subpass.
    pub(super) depth_stencils: Vec<vk::AttachmentReference>,
    /// preserves is an array of render pass attachment indices describing the attachments that are not used by a subpass, but whose contents must be preserved throughout the subpass.
    pub(super) preserves: Vec<vkuint>,
}

impl RenderSubpass {

    pub fn new(bind_point: vk::PipelineBindPoint) -> RenderSubpass {
        RenderSubpass {
            bind_point,
            ..Default::default()
        }
    }

    #[inline]
    pub(super) fn build(&self) -> vk::SubpassDescription {

        // Here p_resolve_attachments and p_depth_stencil_attachment may cause crash if use a empty vec pointer.
        vk::SubpassDescription {
            // The value of the flags is currently provided by extension.
            flags: vk::SubpassDescriptionFlags::empty(),
            pipeline_bind_point       : self.bind_point,
            input_attachment_count    : self.inputs.len() as _,
            p_input_attachments       : self.inputs.as_ptr(),
            color_attachment_count    : self.colors.len() as _,
            p_color_attachments       : self.colors.as_ptr(),
            p_resolve_attachments     : if self.resolves.is_empty() { ptr::null() } else { self.resolves.as_ptr() },
            p_depth_stencil_attachment: if self.depth_stencils.is_empty() { ptr::null() } else { self.depth_stencils.as_ptr() },
            preserve_attachment_count : self.preserves.len() as _,
            p_preserve_attachments    : self.preserves.as_ptr(),
        }
    }
}
