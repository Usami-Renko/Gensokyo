
use ash::vk;
use gsvk::prelude::common::*;
use gsvk::prelude::pipeline::*;

use gsma::{ offset_of, vk_format, vertex_rate };

use nalgebra::{ Point3, Point2, Vector3, Point4, Matrix4 };

#[derive(Debug, Clone, Copy)]
pub struct UBOVS {
    pub projection  : Matrix4<f32>,
    pub view        : Matrix4<f32>,
    pub model       : Matrix4<f32>,
    pub y_correction: Matrix4<f32>,
    pub view_pos    : Point4<f32>,
    pub lod_bias    : f32,
    pub sampler_index: vksint,
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    pos: Point3<f32>,
    normal: Vector3<f32>,
    uv : Point2<f32>,
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
                    binding : 0,
                    location: 0,
                    format: vk_format!(vec3),
                    offset: offset_of!(Self, pos) as _,
                },
                GsVertexInputAttribute {
                    binding : 0,
                    location: 1,
                    format: vk_format!(vec3),
                    offset: offset_of!(Self, normal) as _,
                },
                GsVertexInputAttribute {
                    binding : 0,
                    location: 2,
                    format: vk_format!(vec2),
                    offset: offset_of!(Self, uv) as _,
                },
            ],
        }
    }
}
