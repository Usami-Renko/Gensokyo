
use lazy_static::lazy_static;

use ash::vk;
use gsvk::prelude::common::*;
use gsvk::prelude::pipeline::*;

use gsma::{ offset_of, vk_format, vertex_rate };

use nalgebra::{ Point3, Vector3, Matrix4 };
use super::program::OBJECT_INSTANCE;

lazy_static! {

    pub static ref VERTEX_DATA: [Vertex; 8] = [
        Vertex { position: Point3::new(-1.0, -1.0,  1.0), color: Vector3::new(1.0, 0.0, 0.0), }, // v0
        Vertex { position: Point3::new( 1.0, -1.0,  1.0), color: Vector3::new(0.0, 1.0, 0.0), }, // v1
        Vertex { position: Point3::new( 1.0,  1.0,  1.0), color: Vector3::new(0.0, 0.0, 1.0), }, // v2
        Vertex { position: Point3::new(-1.0,  1.0,  1.0), color: Vector3::new(0.0, 0.0, 0.0), }, // v3
        Vertex { position: Point3::new(-1.0, -1.0, -1.0), color: Vector3::new(1.0, 0.0, 0.0), }, // v4
        Vertex { position: Point3::new( 1.0, -1.0, -1.0), color: Vector3::new(0.0, 1.0, 0.0), }, // v5
        Vertex { position: Point3::new( 1.0,  1.0, -1.0), color: Vector3::new(0.0, 0.0, 1.0), }, // v6
        Vertex { position: Point3::new(-1.0,  1.0, -1.0), color: Vector3::new(0.0, 0.0, 0.0), }, // v7
    ];

    pub static ref INDEX_DATA: [vkuint; 36] = [
        0,1,2, 2,3,0, 1,5,6, 6,2,1, 7,6,5, 5,4,7, 4,0,3, 3,7,4, 4,5,1, 1,0,4, 3,2,6, 6,7,3,
    ];
}

#[derive(Debug, Clone, Copy)]
pub struct UboView {
    pub projection: Matrix4<f32>,
    pub view      : Matrix4<f32>,
    pub y_correction: Matrix4<f32>,
}

pub struct RotationData {
    pub rotations    : [Vector3<f32>; OBJECT_INSTANCE],
    pub rotate_speeds: [Vector3<f32>; OBJECT_INSTANCE],
}

impl RotationData {

    pub fn new_by_rng() -> RotationData {

        let mut data = RotationData {
            rotations    : [nalgebra::zero(); OBJECT_INSTANCE],
            rotate_speeds: [nalgebra::zero(); OBJECT_INSTANCE],
        };

        use rand::distributions::Distribution;
        let rnd_dist = rand::distributions::Uniform::from(-1.0..1.0_f32);
        let mut rnd_engine = rand::thread_rng();

        for i in 0..OBJECT_INSTANCE {
            data.rotations[i] = Vector3::new(
                rnd_dist.sample(&mut rnd_engine), // generate a random float between -1.0 ~ 1.0.
                rnd_dist.sample(&mut rnd_engine),
                rnd_dist.sample(&mut rnd_engine),
            );
            data.rotate_speeds[i] = Vector3::new(
                rnd_dist.sample(&mut rnd_engine),
                rnd_dist.sample(&mut rnd_engine),
                rnd_dist.sample(&mut rnd_engine),
            );
        }

        data
    }
}

pub struct UboDataDynamic {
    pub model: [Matrix4<f32>; OBJECT_INSTANCE],
}

impl UboDataDynamic {

    pub fn identity() -> UboDataDynamic {
        UboDataDynamic {
            model: [Matrix4::identity(); OBJECT_INSTANCE],
        }
    }

    pub fn update(&mut self, rotations: &mut RotationData, delta_time: f32) {

        // Dynamic ubo with per-object model matrices indexed by offsets in the command buffer
        let dim: usize = (OBJECT_INSTANCE as f32).powf(1.0 / 3.0) as usize;
        let offset = Vector3::<f32>::new(5.0, 5.0, 5.0);

        for x in 0..dim {
            for y in 0..dim {
                for z in 0..dim {

                    let dim_f = dim as f32;

                    let index = x * dim * dim + y * dim + z;
                    // update rotations
                    rotations.rotations[index] += delta_time * rotations.rotate_speeds[index];

                    let pos = Vector3::new(
                        -((dim_f * offset.x) / 2.0) + offset.x / 2.0 + (x as f32) * offset.x,
                        -((dim_f * offset.y) / 2.0) + offset.y / 2.0 + (y as f32) * offset.y,
                        -((dim_f * offset.z) / 2.0) + offset.z / 2.0 + (z as f32) * offset.z,
                    );
                    let translate = Matrix4::new_translation(&pos);
                    let rotate = Matrix4::new_rotation(rotations.rotations[index]);

                    self.model[index] = translate * rotate;
                }
            }
        }
    }
}


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