
pub use self::builder::RenderPassBuilder;
pub use self::framebuffer::{ GsFramebuffer, FramebufferBuilder };
pub use self::attachment::{ RenderAttachement, RenderAttachementPrefab };
pub use self::dependency::{ RenderDependency, SubpassStage };
pub use self::render::GsRenderPass;

mod render;
mod attachment;
mod subpass;
mod dependency;
mod builder;
mod framebuffer;
