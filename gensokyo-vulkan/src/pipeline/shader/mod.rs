
pub use self::input::{ GsVertexInputAttribute, GsVertexInputBinding, VertexInputDescription };
pub use self::module::GsShaderInfo;

pub(super) use self::module::GsShaderModule;

pub(super) mod shaderc;

mod module;
mod input;
