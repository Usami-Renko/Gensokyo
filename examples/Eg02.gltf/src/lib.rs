
// TODO: Rename crate in Cargo.toml.
extern crate gensokyo as gs;
extern crate gensokyo_vulkan as gsvk;
extern crate gensokyo_macros as gsma;

pub mod program;

use gsvk::pipeline::shader::*;

pub struct FilePathConstants {
    // shader.
    pub vertex_shader  : &'static str,
    pub framment_shader: &'static str,
    // gltf model.
    pub model_path     : &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct UboObject {
    pub projection: nalgebra::Matrix4<f32>,
    pub view      : nalgebra::Matrix4<f32>,
    pub model     : nalgebra::Matrix4<f32>,
}

pub trait ShaderInputDefination {
    fn desc() -> VertexInputDescription;
}
