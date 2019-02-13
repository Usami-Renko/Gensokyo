
use gsvk::prelude::common::*;
use gsvk::prelude::pipeline::*;
use ash::vk;

use gsma::{ define_input, offset_of, vk_format, vertex_rate };

use nalgebra::Matrix4;

define_input! {
    #[binding = 0, rate = vertex]
    struct Vertex {
        #[location = 0, format = vec4]
        pos:   [f32; 4],
        #[location = 1, format = vec4]
        color: [f32; 4],
    }
}

//       ^ y axis
//       |
//       |  /
//       | /
//       |/
//  -----.-----------> x axis
//      /|
//     / |
//    /  |
//   /
//  < z axis
//
//    v6----- v5
//   /|      /|
//  v1------v0|
//  | |     | |
//  | |v7---|-|v4
//  |/      |/
//  v2------v3
//
//  vertex of cube

pub const VERTEX_DATA: [Vertex; 8] = [
    Vertex { pos: [ 0.6,  0.6,  0.6, 1.0], color: [1.0, 0.0, 0.0, 1.0], }, // v0
    Vertex { pos: [-0.6,  0.6,  0.6, 1.0], color: [0.0, 1.0, 0.0, 1.0], }, // v1
    Vertex { pos: [-0.6, -0.6,  0.6, 1.0], color: [0.0, 0.0, 1.0, 1.0], }, // v2
    Vertex { pos: [ 0.6, -0.6,  0.6, 1.0], color: [1.0, 1.0, 0.0, 1.0], }, // v3
    Vertex { pos: [ 0.6, -0.6, -0.6, 1.0], color: [0.0, 1.0, 1.0, 1.0], }, // v4
    Vertex { pos: [ 0.6,  0.6, -0.6, 1.0], color: [1.0, 0.0, 1.0, 1.0], }, // v5
    Vertex { pos: [-0.6,  0.6, -0.6, 1.0], color: [0.0, 0.0, 0.0, 1.0], }, // v6
    Vertex { pos: [-0.6, -0.6, -0.6, 1.0], color: [1.0, 1.0, 1.0, 1.0], }, // v7
];
pub const INDEX_DATA: [vkuint; 36] = [
    0, 1, 2,
    0, 2, 3,
    0, 3, 4,
    0, 4, 5,
    1, 6, 7,
    1, 7, 2,
    6, 5, 4,
    6, 4, 7,
    5, 6, 1,
    5, 1, 0,
    3, 2, 7,
    3, 7, 4,
];

#[derive(Debug, Clone, Copy)]
pub struct UboObject {
    pub projection: Matrix4<f32>,
    pub view      : Matrix4<f32>,
    pub model     : Matrix4<f32>,
    pub y_correction: Matrix4<f32>,
}
