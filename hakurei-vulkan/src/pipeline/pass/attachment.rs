
use ash::vk;

use pipeline::state::multisample::SampleCountType;
use resources::image::ImageLayout;
use utils::marker::{ VulkanFlags, VulkanEnum };

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RenderAttachementPrefab {
    BackColorAttachment,
    DepthAttachment,
}
impl RenderAttachementPrefab {

    fn generate(&self) -> RenderAttachement {

        match self {
            | RenderAttachementPrefab::BackColorAttachment => RenderAttachement {
                ..Default::default()
            },
            | RenderAttachementPrefab::DepthAttachment => RenderAttachement {
                store_op: AttachmentStoreOp::DontCare,
                layout: ImageLayout::DepthStencilAttachmentOptimal,
                clear_value: vk::ClearValue { depth: vk::ClearDepthStencilValue { depth: 1.0, stencil: 0, } },
                ..Default::default()
            },
        }
    }
}

/// Wrapper class of vk::Attachement.
pub struct RenderAttachement {

    /// flags a set of AttachmentDescFlag specifying additional properties of the attachment.
    flags            : Vec<AttachmentDescFlag>,
    /// format is a vk::Format value specifying the format of the image view that will be used for the attachment.
    format           : vk::Format,
    /// sample_count the number of samples of the image.
    sample_count     : SampleCountType,
    /// load_op is a AttachmentLoadOp value specifying how the contents of color and depth components of the attachment are treated at the beginning of the subpass where it is first used.
    load_op          : AttachmentLoadOp,
    /// store_op is a AttachmentStoreOp value specifying how the contents of color and depth components of the attachment are treated at the end of the subpass where it is last used.
    store_op         : AttachmentStoreOp,
    /// stencil_load_op is a AttachmentStoreOp value specifying how the contents of stencil components of the attachment are treated at the beginning of the subpass where it is first used.
    stencil_load_op  : AttachmentLoadOp,
    /// stencil_store_op is a AttachmentStoreOp value specifying how the contents of stencil components of the attachment are treated at the end of the last subpass where it is used.
    stencil_store_op : AttachmentStoreOp,
    /// initial_layout is the layout the attachment image subresource will be in when a render pass instance begins.
    initial_layout   : ImageLayout,
    /// final_layout is the layout the attachment image subresource will be transitioned to when a render pass instance ends.
    ///
    /// During a render pass instance, an attachment can use a different layout in each subpass, if desired.
    final_layout     : ImageLayout,
    // TODO: Remove pub statement.
    /// layout specifying the layout the attachment uses during the subpass.
    pub layout           : ImageLayout,
    // TODO: Currently clear_value is not configuratable.
    // TODO: Remove pub statement.
    /// the clear value for each attachment.
    pub clear_value: vk::ClearValue,
}

impl RenderAttachement {

    pub fn setup(prefab: RenderAttachementPrefab, format: vk::Format) -> RenderAttachement {
        RenderAttachement {
            format,
            ..prefab.generate()
        }
    }

    pub(super) fn desc(&self) -> vk::AttachmentDescription {
        vk::AttachmentDescription {
            flags  : self.flags.flags(),
            format : self.format,
            samples: self.sample_count.value(),
            load_op         : self.load_op.value(),
            store_op        : self.store_op.value(),
            stencil_load_op : self.stencil_load_op.value(),
            stencil_store_op: self.stencil_store_op.value(),
            initial_layout  : self.initial_layout.value(),
            final_layout    : self.final_layout.value(),
        }
    }

    pub fn set_flags(&mut self, flags: &[AttachmentDescFlag]) {
        self.flags = flags.to_vec();
    }
    pub fn set_format(&mut self, format: vk::Format) {
        self.format = format;
    }
    pub fn set_samples(&mut self, sample_count: SampleCountType) {
        self.sample_count = sample_count;
    }
    pub fn set_op(&mut self, load: AttachmentLoadOp, store: AttachmentStoreOp) {
        self.load_op  = load;
        self.store_op = store;
    }
    pub fn set_stencil_op(&mut self, load: AttachmentLoadOp, store: AttachmentStoreOp) {
        self.stencil_load_op  = load;
        self.stencil_store_op = store;
    }
    pub fn set_image_layout(&mut self, layout: ImageLayout, initial_layout: ImageLayout, final_layout: ImageLayout) {
        self.layout         = layout;
        self.initial_layout = initial_layout;
        self.final_layout   = final_layout;
    }
}

impl Default for RenderAttachement {

    fn default() -> RenderAttachement {
        RenderAttachement {
            flags            : vec![],
            format           : vk::Format::B8g8r8a8Unorm,
            sample_count     : SampleCountType::Count1Bit,
            load_op          : AttachmentLoadOp::Clear,
            store_op         : AttachmentStoreOp::Store,
            stencil_load_op  : AttachmentLoadOp::DontCare,
            stencil_store_op : AttachmentStoreOp::DontCare,
            initial_layout   : ImageLayout::Undefined,
            final_layout     : ImageLayout::PresentSrcKHR,
            layout           : ImageLayout::ColorAttachmentOptimal,
            clear_value      : vk::ClearValue { color: vk::ClearColorValue { float32: [0.0, 0.0, 0.0, 1.0], } }
        }
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AttachmentDescFlag {
    /// MayAliasBit specifies that the attachment aliases the same device memory as other attachments.
    MayAliasBit,
}

impl VulkanFlags for [AttachmentDescFlag] {
    type FlagType = vk::AttachmentDescriptionFlags;

    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::AttachmentDescriptionFlags::empty(), |acc, flag| {
            match flag {
                | AttachmentDescFlag::MayAliasBit => acc | vk::ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT,
            }
        })
    }
}

// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AttachmentLoadOp {

    /// Load specifies that the previous contents of the image within the render area will be preserved.
    ///
    /// For attachments with a depth/stencil format, this uses the access type VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT.
    ///
    /// For attachments with a color format, this uses the access type VK_ACCESS_COLOR_ATTACHMENT_READ_BIT.
    Load,
    /// Clear specifies that the contents within the render area will be cleared to a uniform value, which is specified when a render pass instance is begun.
    ///
    /// For attachments with a depth/stencil format, this uses the access type VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT.
    ///
    /// For attachments with a color format, this uses the access type VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT
    Clear,
    /// DontCare specifies that the previous contents within the area need not be preserved; the contents of the attachment will be undefined inside the render area.
    ///
    /// For attachments with a depth/stencil format, this uses the access type VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT.
    ///
    /// For attachments with a color format, this uses the access type VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT.
    DontCare,
}

impl VulkanEnum for AttachmentLoadOp {
    type EnumType = vk::AttachmentLoadOp;

    fn value(&self) -> Self::EnumType {
        match self {
            | AttachmentLoadOp::Load     => vk::AttachmentLoadOp::Load,
            | AttachmentLoadOp::Clear    => vk::AttachmentLoadOp::Clear,
            | AttachmentLoadOp::DontCare => vk::AttachmentLoadOp::DontCare,
        }
    }
}

// TODO: Map to raw value
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum AttachmentStoreOp {
    /// Store specifies the contents generated during the render pass and within the render area are written to memory.
    ///
    /// For attachments with a depth/stencil format, this uses the access type VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT.
    ///
    /// For attachments with a color format, this uses the access type VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT.
    Store,
    /// DontCare specifies the contents within the render area are not needed after rendering, and may be discarded; the contents of the attachment will be undefined inside the render area.
    ///
    /// For attachments with a depth/stencil format, this uses the access type VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT.
    ///
    /// For attachments with a color format, this uses the access type VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT
    DontCare,
}

impl VulkanEnum for AttachmentStoreOp {
    type EnumType = vk::AttachmentStoreOp;

    fn value(&self) -> Self::EnumType {
        match self {
            | AttachmentStoreOp::Store    => vk::AttachmentStoreOp::Store,
            | AttachmentStoreOp::DontCare => vk::AttachmentStoreOp::DontCare,
        }
    }
}
