
use ash::vk;

use crate::pipeline::pass::subpass::AttachmentType;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RenderAttachementPrefab {
    PresentAttachment,
    DepthAttachment,
}

/// Wrapper class of vk::Attachement.
pub struct RenderAttachement {

    attachment: vk::AttachmentDescription,

    pub(crate) attach_type: AttachmentType,
    pub(crate) layout: vk::ImageLayout,
    pub(crate) clear_value: vk::ClearValue,
}

impl RenderAttachement {

    /// `format` is a vk::Format value specifying the format of the image view that will be used for the attachment.
    pub fn setup(prefab: RenderAttachementPrefab, format: vk::Format) -> RenderAttachement {

        let mut attachment = vk::AttachmentDescription {
            flags            : vk::AttachmentDescriptionFlags::empty(),
            format,
            samples          : vk::SampleCountFlags::TYPE_1,
            load_op          : vk::AttachmentLoadOp::CLEAR,
            store_op         : vk::AttachmentStoreOp::STORE,
            stencil_load_op  : vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op : vk::AttachmentStoreOp::DONT_CARE,
            initial_layout   : vk::ImageLayout::UNDEFINED,
            final_layout     : vk::ImageLayout::UNDEFINED,
        };

        let (clear_value, layout, attach_type) = match prefab {
            | RenderAttachementPrefab::PresentAttachment => {

                attachment.final_layout = vk::ImageLayout::PRESENT_SRC_KHR;

                let clear_value = vk::ClearValue {
                    color: vk::ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 1.0],
                    }
                };

                let layout = vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL;
                (clear_value, layout, AttachmentType::Color)
            },
            | RenderAttachementPrefab::DepthAttachment => {

                attachment.final_layout = vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL;

                let clear_value = vk::ClearValue {
                    depth_stencil: vk::ClearDepthStencilValue {
                        depth: 1.0,
                        stencil: 0,
                    }
                };

                let layout = vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL;
                (clear_value, layout, AttachmentType::DepthStencil)
            },
        };

        RenderAttachement {
            attachment, attach_type,
            layout, clear_value,
        }
    }

    // TODO: Add configuration for vk::AttachmentDescriptionFlags.
    /// `flags` specifyies additional properties of the attachment.
    pub fn with_flags(mut self, flags: vk::AttachmentDescriptionFlags) -> RenderAttachement {
        self.attachment.flags = flags;
        self
    }

    /// `count` the number of samples of the image.
    pub fn sample(mut self, count: vk::SampleCountFlags) -> RenderAttachement {
        self.attachment.samples = count;
        self
    }

    /// `load` is a AttachmentLoadOp value specifying how the contents of color and depth components of the attachment are treated at the beginning of the subpass where it is first used.
    ///
    /// `store` is a AttachmentStoreOp value specifying how the contents of color and depth components of the attachment are treated at the end of the subpass where it is last used.
    pub fn op(mut self, load: vk::AttachmentLoadOp, store: vk::AttachmentStoreOp) -> RenderAttachement {
        self.attachment.load_op = load;
        self.attachment.store_op = store;
        self
    }

    /// `load` is a AttachmentStoreOp value specifying how the contents of stencil components of the attachment are treated at the beginning of the subpass where it is first used.
    ///
    /// `store` is a AttachmentStoreOp value specifying how the contents of stencil components of the attachment are treated at the end of the last subpass where it is used.
    pub fn stencil_op(mut self, load: vk::AttachmentLoadOp, store: vk::AttachmentStoreOp) -> RenderAttachement {
        self.attachment.stencil_load_op = load;
        self.attachment.stencil_store_op = store;
        self
    }

    /// `initial` is the layout the attachment image subresource will be in when a render pass instance begins.
    ///
    /// `transition` specifying the layout the attachment uses during the subpass.
    ///
    /// `final_layout` is the layout the attachment image subresource will be transitioned to when a render pass instance ends.
    ///
    /// During a render pass instance, an attachment can use a different layout in each subpass, if desired.
    pub fn layout(mut self, initial: vk::ImageLayout, transition: vk::ImageLayout, fin: vk::ImageLayout) -> RenderAttachement {
        self.attachment.initial_layout = initial;
        self.layout = transition;
        self.attachment.final_layout = fin;
        self
    }

    /// `clear_value` the clear value for each attachment.
    pub fn clear_value(mut self, color: vk::ClearValue) -> RenderAttachement {
        self.clear_value = color;
        self
    }

    pub(crate) fn build(self) -> vk::AttachmentDescription {
        self.attachment
    }
}
