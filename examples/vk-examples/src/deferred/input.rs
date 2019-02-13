
use ash::vk;

use gsvk::prelude::pipeline::*;
use gsvk::prelude::common::*;
use gsvk::prelude::descriptor::*;
use gsvk::prelude::buffer::*;
use gsvk::prelude::image::*;
use gs::prelude::*;

use gsma::{ offset_of, vk_format, vertex_rate };

use std::mem;

use nalgebra::{ Point3, Point2, Vector2, Vector3, Vector4, Matrix4 };

pub struct ResourceRepository {

    pub descriptors : GsDescriptorRepository,
    pub buffers     : GsBufferRepository<Device>,
    pub images      : GsImageRepository<Device>,
    pub uniforms    : GsBufferRepository<Host>,
}

pub struct UniformResource {

    pub texture   : GsCombinedImgSampler,
    pub ubo_vs    : Vec<UBOVS>,
    pub ubo_set   : DescriptorSet,
    pub ubo_buffer: GsUniformBuffer,
    pub ubo_storage   : GsBufferRepository<Host>,

    pub depth_attachment: GsDSAttachment,
    pub image_storage   : GsImageRepository<Device>,

    pub desc_storage: GsDescriptorRepository,
}

#[derive(Debug, Clone, Copy)]
pub struct UBOVS {
    pub projection: Matrix4<f32>,
    pub view      : Matrix4<f32>,
    pub model     : Matrix4<f32>,
    pub y_correction: Matrix4<f32>,
    pub light_pos : Vector4<f32>,
}

#[derive(Debug, Clone, Copy)]
pub struct Vertex {
    position: Point3<f32>,
    uv      : Point2<f32>,
    color   : Vector3<f32>,
    normal  : Vector3<f32>,
    tangent : Vector3<f32>,
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
                    offset: offset_of!(Self, position) as _,
                },
                GsVertexInputAttribute {
                    binding : 0,
                    location: 1,
                    format: vk_format!(vec2),
                    offset: offset_of!(Self, uv) as _,
                },
                GsVertexInputAttribute {
                    binding : 0,
                    location: 2,
                    format: vk_format!(vec3),
                    offset: offset_of!(Self, color) as _,
                },
                GsVertexInputAttribute {
                    binding : 0,
                    location: 3,
                    format: vk_format!(vec3),
                    offset: offset_of!(Self, normal) as _,
                },
                GsVertexInputAttribute {
                    binding : 0,
                    location: 4,
                    format: vk_format!(vec3),
                    offset: offset_of!(Self, tangent) as _,
                },
            ],
        }
    }
}
