
pub use self::builder::RenderPassBuilder;
pub use self::framebuffer::{ HaFramebuffer, FramebufferBuilder };
pub use self::attachment::{ RenderAttachement, RenderAttachementPrefab };
pub use self::subpass::AttachmentType;
pub use self::dependency::RenderDependency;

pub(crate) use self::render::HaRenderPass;


mod render;
mod attachment;
mod subpass;
mod dependency;
mod builder;
mod framebuffer;
