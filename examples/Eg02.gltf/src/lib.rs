
pub mod program;

use gsvk::pipeline::shader::*;

type Mat4x4 = nalgebra::Matrix4<f32>;

pub struct FilePathConstants {
    // shader.
    pub vertex_shader  : &'static str,
    pub fragment_shader: &'static str,
    // gltf model.
    pub model_path     : &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct UboObject {
    pub projection   : Mat4x4,
    pub view         : Mat4x4,
    pub model        : Mat4x4,
    pub y_correction : Mat4x4,
}

pub trait ShaderInputDefinition {
    fn desc() -> VertexInputDescription;
}
