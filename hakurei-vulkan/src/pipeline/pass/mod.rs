
pub use self::dependency::AccessFlag;
pub use self::builder::RenderPassBuilder;

pub(crate) use self::render::HaRenderPass;
pub(crate) use self::dependency::DependencyFlag;

mod render;
mod attachment;
mod subpass;
mod dependency;
mod builder;
