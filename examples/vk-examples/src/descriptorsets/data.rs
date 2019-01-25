
use ash::vk;
use gsvk::prelude::pipeline::*;
use gsvk::prelude::descriptor::*;
use gsvk::prelude::buffer::*;
use gsvk::prelude::image::*;

use gsma::{ offset_of, vk_format, vertex_rate };

use nalgebra::{ Point3, Vector2, Vector3, Matrix4 };

pub struct CubeResources {
    pub matrices: [Vec<UBOMatrices>; 2],
    pub texture : [GsSampleImage; 2],
    pub ubo_set : [DescriptorSet; 2],
    pub ubo_buffer: [GsUniformBuffer; 2],
}

#[derive(Debug, Clone, Copy)]
pub struct UBOMatrices {
    pub projection: Matrix4<f32>,
    pub model     : Matrix4<f32>,
    pub view      : Matrix4<f32>,
    pub y_correction: Matrix4<f32>,
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    position: Point3<f32>,
    normal  : Vector3<f32>,
    uv      : Vector2<f32>,
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
                GsVertexInputAttribute {
                    binding: 0,
                    location: 2,
                    format: vk_format!(vec2),
                    offset: offset_of!(Self, uv) as _,
                },
            ],
        }
    }
}
