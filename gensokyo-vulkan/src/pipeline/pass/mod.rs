
pub use self::builder::RenderPassBuilder;
pub use self::framebuffer::{ GsFramebuffer, FramebufferBuilder };
pub use self::attachment::{ RenderAttachmentCI, RenderAttType, Present, DepthStencil };
pub use self::dependency::{ RenderDependencyCI, SubpassStage };
pub use self::render::GsRenderPass;

mod render;
mod attachment;
mod subpass;
mod dependency;
mod builder;
mod framebuffer;
