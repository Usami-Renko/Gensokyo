
use ash;

pub use pipeline::pass::render::HaRenderPass;
pub use pipeline::pass::attachment::{ RenderAttachement, RenderAttachementPrefab };
pub use pipeline::pass::attachment::{ AttachmentDescFlag, AttachmentLoadOp, AttachmentStoreOp };
pub use pipeline::pass::subpass::{ AttachmentType, SubpassType };
pub use pipeline::pass::dependency::{ RenderDependency, RenderDependencyPrefab };
pub use pipeline::pass::dependency::{ AccessFlag, DependencyFlag };
pub use pipeline::pass::builder::RenderPassBuilder;

pub const SUBPASS_EXTERAL: ash::vk::uint32_t = ash::vk::VK_SUBPASS_EXTERNAL;
