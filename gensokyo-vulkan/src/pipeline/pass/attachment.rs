
use ash::vk;

use crate::pipeline::pass::subpass::AttachmentRawType;
use crate::types::format::Format;

pub trait RenderAttType: Sized {
    const IMAGE_LAYOUT: vk::ImageLayout;
    const CLEAR_VALUE: vk::ClearValue;
    const RAW_TYPE: AttachmentRawType;

    fn build() -> vk::AttachmentDescription;
    fn frame_view(self) -> AttachmentView;
}

#[derive(Debug, Clone)]
pub enum AttachmentView {
    Present,
    DepthStencil(vk::ImageView),
}

pub struct Present;
pub struct DepthStencil(pub(crate) vk::ImageView);

/// Wrapper class of vk::Attachment.
pub struct RenderAttachmentCI<T>
    where
        T: RenderAttType {

    phantom: T,
    content: vk::AttachmentDescription,

    layout: vk::ImageLayout,
    clear_value: vk::ClearValue,
}

impl<T> RenderAttachmentCI<T>
    where
        T: RenderAttType {

    /// `format` is a vk::Format value specifying the format of the image view that will be used for the attachment.
    pub fn create(att_type: T, attachment_format: Format) -> RenderAttachmentCI<T> {

        let mut attachment = T::build();
        attachment.format = attachment_format.0;

        RenderAttachmentCI {
            phantom    : att_type,
            content    : attachment,
            layout     : T::IMAGE_LAYOUT,
            clear_value: T::CLEAR_VALUE,
        }
    }

    // TODO: Add configuration for vk::AttachmentDescriptionFlags.
    /// `flags` specifies additional properties of the attachment.
    pub fn with_flags(mut self, flags: vk::AttachmentDescriptionFlags) -> RenderAttachmentCI<T> {
        self.content.flags = flags;
        self
    }

    /// `count` the number of samples of the image.
    pub fn sample(mut self, count: vk::SampleCountFlags) -> RenderAttachmentCI<T> {
        self.content.samples = count;
        self
    }

    /// `load` is a AttachmentLoadOp value specifying how the contents of color and depth components of the attachment are treated at the beginning of the subpass where it is first used.
    ///
    /// `store` is a AttachmentStoreOp value specifying how the contents of color and depth components of the attachment are treated at the end of the subpass where it is last used.
    pub fn op(mut self, load: vk::AttachmentLoadOp, store: vk::AttachmentStoreOp) -> RenderAttachmentCI<T> {
        self.content.load_op  = load;
        self.content.store_op = store;
        self
    }

    /// `load` is a AttachmentStoreOp value specifying how the contents of stencil components of the attachment are treated at the beginning of the subpass where it is first used.
    ///
    /// `store` is a AttachmentStoreOp value specifying how the contents of stencil components of the attachment are treated at the end of the last subpass where it is used.
    pub fn stencil_op(mut self, load: vk::AttachmentLoadOp, store: vk::AttachmentStoreOp) -> RenderAttachmentCI<T> {
        self.content.stencil_load_op  = load;
        self.content.stencil_store_op = store;
        self
    }

    /// `initial` is the layout the attachment image subresource will be in when a render pass instance begins.
    ///
    /// `transition` specifying the layout the attachment uses during the subpass.
    ///
    /// `final_layout` is the layout the attachment image subresource will be transitioned to when a render pass instance ends.
    ///
    /// During a render pass instance, an attachment can use a different layout in each subpass, if desired.
    pub fn layout(mut self, initial: vk::ImageLayout, transition: vk::ImageLayout, fin: vk::ImageLayout) -> RenderAttachmentCI<T> {
        self.content.initial_layout = initial;
        self.layout = transition;
        self.content.final_layout = fin;
        self
    }

    /// `clear_value` the clear value for each attachment.
    pub fn clear_value(mut self, color: vk::ClearValue) -> RenderAttachmentCI<T> {
        self.clear_value = color;
        self
    }

    pub(super) fn reference(&self, at_index: usize) -> vk::AttachmentReference {

        vk::AttachmentReference {
            attachment: at_index as u32,
            layout    : self.layout,
        }
    }

    pub(super) fn take(self) -> (vk::AttachmentDescription, AttachmentView, vk::ClearValue) {
        (self.content, self.phantom.frame_view(), self.clear_value)
    }
}


impl RenderAttType for Present {
    const IMAGE_LAYOUT: vk::ImageLayout = vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL;
    const CLEAR_VALUE: vk::ClearValue = vk::ClearValue { color: vk::ClearColorValue { float32: [0.0, 0.0, 0.0, 1.0] } };
    const RAW_TYPE: AttachmentRawType = AttachmentRawType::Color;

    fn build() -> vk::AttachmentDescription {

        vk::AttachmentDescription {
            flags            : vk::AttachmentDescriptionFlags::empty(),
            format           : vk::Format::UNDEFINED,
            samples          : vk::SampleCountFlags::TYPE_1,
            load_op          : vk::AttachmentLoadOp::CLEAR,
            store_op         : vk::AttachmentStoreOp::STORE,
            stencil_load_op  : vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op : vk::AttachmentStoreOp::DONT_CARE,
            initial_layout   : vk::ImageLayout::UNDEFINED,
            final_layout     : vk::ImageLayout::PRESENT_SRC_KHR,
        }
    }

    fn frame_view(self) -> AttachmentView {
        AttachmentView::Present
    }
}

impl RenderAttType for DepthStencil {
    const IMAGE_LAYOUT: vk::ImageLayout = vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL;
    const CLEAR_VALUE: vk::ClearValue = vk::ClearValue { depth_stencil: vk::ClearDepthStencilValue { depth: 1.0, stencil: 0 } };
    const RAW_TYPE: AttachmentRawType = AttachmentRawType::DepthStencil;

    fn build() -> vk::AttachmentDescription {

        vk::AttachmentDescription {
            flags            : vk::AttachmentDescriptionFlags::empty(),
            format           : vk::Format::UNDEFINED,
            samples          : vk::SampleCountFlags::TYPE_1,
            load_op          : vk::AttachmentLoadOp::CLEAR,
            store_op         : vk::AttachmentStoreOp::DONT_CARE,
            stencil_load_op  : vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op : vk::AttachmentStoreOp::DONT_CARE,
            initial_layout   : vk::ImageLayout::UNDEFINED,
            final_layout     : vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL,
        }
    }

    fn frame_view(self) -> AttachmentView {
        AttachmentView::DepthStencil(self.0)
    }
}
