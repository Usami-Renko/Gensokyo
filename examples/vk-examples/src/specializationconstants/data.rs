
use ash::vk;
use gsvk::prelude::common::*;
use gsvk::prelude::pipeline::*;
use gsvk::prelude::command::*;
use gsvk::prelude::descriptor::*;
use gsvk::prelude::buffer::*;
use gsvk::prelude::image::*;
use gs::prelude::*;

use gsma::{ offset_of, vk_format, vertex_rate };

use std::mem;

use nalgebra::{ Point3, Vector2, Vector3, Vector4, Matrix4 };


pub struct ModelResource {
    pub model: GsglTFEntity,
    pub repository: GsBufferRepository<Device>,
}

pub struct UniformResource {

    pub texture   : GsSampleImage,
    pub ubo_vs    : Vec<UBOVS>,
    pub ubo_set   : DescriptorSet,
    pub ubo_buffer: GsUniformBuffer,
    pub ubo_storage   : GsBufferRepository<Host>,

    pub depth_attachment: GsDSAttachment,
    pub image_storage   : GsImageRepository<Device>,

    pub desc_storage: GsDescriptorRepository,
}

pub struct PipelineResource {

    pub pipeline_set: GsPipelineSet<Graphics>,

    pub phong   : PipelineIndex,
    pub toon    : PipelineIndex,
    pub textured: PipelineIndex,

    pub phong_viewport   : CmdViewportInfo,
    pub toon_viewport    : CmdViewportInfo,
    pub textured_viewport: CmdViewportInfo,
}

// Host data to take specialization constants from
pub struct SpecializationData {
    // Sets the lighting model used in the fragment "uber" shader
    pub light_model: vkuint,
    // Parameter for the toon shading part of the fragment shader
    pub toon_desaturation_factor: vkfloat,
}

impl SpecializationData {

    pub fn specialization_map_entries() -> [vk::SpecializationMapEntry; 2] {

        // Each shader constant of a shader stage corresponds to one map entry
        [
            vk::SpecializationMapEntry {
                constant_id: 0,
                offset: offset_of!(Self, light_model) as _,
                size  : mem::size_of::<vkuint>(),
            },
            vk::SpecializationMapEntry {
                constant_id: 1,
                offset: offset_of!(Self, toon_desaturation_factor) as _,
                size  : mem::size_of::<vkfloat>(),
            },
        ]
    }
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
                    binding : 0,
                    location: 0,
                    format: vk_format!(vec3),
                    offset: offset_of!(Self, position) as _,
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
