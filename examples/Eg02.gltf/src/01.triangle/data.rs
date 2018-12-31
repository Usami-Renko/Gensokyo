
use ash::vk;
use nalgebra::Matrix4;
use gsvk::pipeline::shader::*;
use gsvk::types::vkuint;
use gsma::{ define_input, offset_of, vertex_rate, vk_format };

#[derive(Debug, Clone, Copy)]
pub struct UboObject {
    pub projection: Matrix4<f32>,
    pub view      : Matrix4<f32>,
    pub model     : Matrix4<f32>,
}

define_input! {
    #[binding = 0, rate = vertex]
    struct Vertex {
        #[location = 0, format = vec3]
        pos: [f32; 3],
    }
}
