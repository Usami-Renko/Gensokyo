
use ash::vk;
use gsvk::prelude::pipeline::*;

use gsma::{ offset_of, vk_format, vertex_rate };

use nalgebra::{ Point3, Vector3, Vector4, Matrix4 };

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
pub struct UBOMatrices {
    pub projection: Matrix4<f32>,
    pub model     : Matrix4<f32>,
    pub view      : Matrix4<f32>,
    pub y_correction: Matrix4<f32>,
    pub camera_pos: Point3<f32>,
}

#[derive(Debug, Clone, Copy)]
pub struct UboParams {

    lights: [Vector4<f32>; 4],
}

/// The data structure of push constant block define in pbr.vert.glsl
#[derive(Default, Clone, Serialize)]
pub struct ObjPosPushBlock {
    pub pos: [f32; 3],
}

/// The data structure of push constant block defined in pbr.frag.glsl.
#[derive(Default, Clone, Serialize)]
pub struct MaterialPushBlock {

    pub roughness: f32,
    pub metallic : f32,
    pub rgb: [f32; 3],
}

lazy_static! {

    pub static ref MATERIAL_DATA: Vec<MaterialPushBlock> = vec![
        MaterialPushBlock { roughness: 0.1, metallic: 1.0, rgb: [     1.0, 0.765557, 0.336057] }, // Gold
        MaterialPushBlock { roughness: 0.1, metallic: 1.0, rgb: [0.955008, 0.637427, 0.538163] }, // Copper
        MaterialPushBlock { roughness: 0.1, metallic: 1.0, rgb: [0.549585, 0.556114, 0.554256] }, // Chromium
        MaterialPushBlock { roughness: 0.1, metallic: 1.0, rgb: [0.659777, 0.608679, 0.525649] }, // Nickel
        MaterialPushBlock { roughness: 0.1, metallic: 1.0, rgb: [0.541931, 0.496791, 0.449419] }, // Titanium
        MaterialPushBlock { roughness: 0.1, metallic: 1.0, rgb: [0.662124, 0.654864, 0.633732] }, // Cobalt
        MaterialPushBlock { roughness: 0.1, metallic: 1.0, rgb: [0.672411, 0.637331, 0.585456] }, // Platinum

        MaterialPushBlock { roughness: 0.1, metallic: 1.0, rgb: [1.0, 1.0, 1.0] }, // White
        MaterialPushBlock { roughness: 0.1, metallic: 1.0, rgb: [1.0, 0.0, 0.0] }, // Red
        MaterialPushBlock { roughness: 0.1, metallic: 1.0, rgb: [0.0, 0.0, 1.0] }, // Blue
        MaterialPushBlock { roughness: 0.1, metallic: 1.0, rgb: [0.0, 0.0, 0.0] }, // Black
    ];
}
