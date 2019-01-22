
use ash::vk;
use gsvk::prelude::common::*;
use gsvk::prelude::pipeline::*;

use gsma::{ offset_of, vk_format, vertex_rate };

use nalgebra::{ Point3, Vector3, Matrix4 };

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    position: Point3<f32>,
    color   : Vector3<f32>,
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
                    offset: offset_of!(Self, color) as _,
                },
            ],
        }
    }
}

lazy_static! {

    pub static ref VERTEX_DATA: [Vertex; 3] = [
        Vertex { position: Point3::new( 1.0,  1.0,  0.0), color: Vector3::new(1.0, 0.0, 0.0), }, // v0
        Vertex { position: Point3::new(-1.0,  1.0,  0.0), color: Vector3::new(0.0, 1.0, 0.0), }, // v1
        Vertex { position: Point3::new( 1.0, -1.0,  0.0), color: Vector3::new(0.0, 0.0, 1.0), }, // v2
    ];
    pub static ref INDEX_DATA: [vkuint; 3] = [0, 1, 2];
}

#[derive(Debug, Clone, Copy)]
pub struct UboObject {
    pub projection: Matrix4<f32>,
    pub view      : Matrix4<f32>,
    pub model     : Matrix4<f32>,
    pub y_correction: Matrix4<f32>,
}
