
use ash::vk;
use gsvk::pipeline::shader::*;
use gsma::{ offset_of, vertex_rate, vk_format };

use example02::ShaderInputDefinition;
use nalgebra::{ Point3, Vector3 };

//define_input! {
//    #[binding = 0, rate = vertex]
//    struct Vertex {
//        #[location = 0, format = vec3]
//        pos: [f32; 3],
//    }
//}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pos: Point3<f32>,
    normal: Vector3<f32>,
}

impl ShaderInputDefinition for Vertex {

    fn desc() -> VertexInputDescription {
        use std::mem;
        VertexInputDescription {
            bindings: vec![
                GsVertexInputBinding {
                    binding: 0,
                    stride: mem::size_of::<Self>() as _,
                    rate: vertex_rate!(vertex),
                },
            ],
            attributes: vec![
                GsVertexInputAttribute {
                    binding: 0,
                    location: 0,
                    format: vk_format!(vec3),
                    offset: offset_of!(Self, pos) as _,
                },
                GsVertexInputAttribute {
                    binding: 0,
                    location: 1,
                    format: vk_format!(vec3),
                    offset: offset_of!(Self, normal) as _,
                }
            ],
        }
    }
}
