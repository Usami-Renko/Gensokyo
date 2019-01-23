
use ash::vk;
use gsvk::prelude::pipeline::*;

use gsma::{ offset_of, vk_format, vertex_rate };

use nalgebra::{ Point3, Vector3, Matrix4 };

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    position: Point3<f32>,
    normal  : Vector3<f32>,
}

impl Vertex {

    pub fn input_description() -> VertexInputDescription {

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
                    offset: offset_of!(Self, position) as _,
                },
                GsVertexInputAttribute {
                    binding: 0,
                    location: 1,
                    format: vk_format!(vec3),
                    offset: offset_of!(Self, normal) as _,
                },
            ],
        }
    }
}


#[derive(Debug, Clone, Copy)]
pub struct UBOVS {
    pub projection: Matrix4<f32>,
    pub model     : Matrix4<f32>,
    pub view      : Matrix4<f32>,
    pub y_correction: Matrix4<f32>,
}

/// The data structure of push constant block define in lights.vert
#[derive(Clone, Serialize)]
#[repr(C)]
pub struct PushConstants {
    pub lights: [[f32; 4]; 6],
}
