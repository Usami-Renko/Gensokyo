
pub use self::blending::HaBlend;
pub use self::attachment::{ BlendAttachemnt, BlendAttachmentPrefab, ColorComponentFlag };
pub use self::logic_op::HaLogicalOp;

use ash;
pub type BlendFactor = ash::vk::BlendFactor;
pub type BlendOp     = ash::vk::BlendOp;

mod blending;
mod attachment;
mod logic_op;
