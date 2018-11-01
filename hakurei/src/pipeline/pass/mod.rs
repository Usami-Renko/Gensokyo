
pub use self::render::HaRenderPass;
pub use self::attachment::{ RenderAttachement, RenderAttachementPrefab };
pub use self::attachment::{ AttachmentDescFlag, AttachmentLoadOp, AttachmentStoreOp };
pub use self::subpass::AttachmentType;
pub use self::dependency::{ RenderDependency, RenderDependencyPrefab };
pub use self::dependency::{ AccessFlag, DependencyFlag };
pub use self::builder::RenderPassBuilder;

use ash;
pub const SUBPASS_EXTERAL: ash::vk::uint32_t = ash::vk::VK_SUBPASS_EXTERNAL;

mod render;
mod attachment;
mod subpass;
mod dependency;
mod builder;
