
use hakurei::prelude::*;
use hakurei::prelude::pipeline::*;

use cgmath::Matrix4;

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

pub const VERTEX_DATA: [Vertex; 16] = [
    // cube 1
    Vertex { pos: [ 0.0,  0.0,  0.0, 1.0], color: [1.0, 1.0, 1.0, 1.0], }, // v0
    Vertex { pos: [-0.8,  0.0,  0.0, 1.0], color: [1.0, 1.0, 1.0, 1.0], }, // v1
    Vertex { pos: [-0.8, -0.8,  0.0, 1.0], color: [1.0, 1.0, 1.0, 1.0], }, // v2
    Vertex { pos: [ 0.0, -0.8,  0.0, 1.0], color: [1.0, 1.0, 1.0, 1.0], }, // v3
    Vertex { pos: [ 0.0, -0.8, -0.8, 1.0], color: [1.0, 1.0, 1.0, 1.0], }, // v4
    Vertex { pos: [ 0.0,  0.0, -0.8, 1.0], color: [1.0, 1.0, 1.0, 1.0], }, // v5
    Vertex { pos: [-0.8,  0.0, -0.8, 1.0], color: [1.0, 1.0, 1.0, 1.0], }, // v6
    Vertex { pos: [-0.8, -0.8, -0.8, 1.0], color: [1.0, 1.0, 1.0, 1.0], }, // v7

    // cube 2
    Vertex { pos: [ 0.6,  0.6,  0.6, 1.0], color: [1.0, 1.0, 1.0, 1.0], }, // v0
    Vertex { pos: [-0.2,  0.6,  0.6, 1.0], color: [1.0, 1.0, 1.0, 1.0], }, // v1
    Vertex { pos: [-0.2, -0.2,  0.6, 1.0], color: [1.0, 1.0, 1.0, 1.0], }, // v2
    Vertex { pos: [ 0.6, -0.2,  0.6, 1.0], color: [1.0, 1.0, 1.0, 1.0], }, // v3
    Vertex { pos: [ 0.6, -0.2, -0.2, 1.0], color: [1.0, 1.0, 1.0, 1.0], }, // v4
    Vertex { pos: [ 0.6,  0.6, -0.2, 1.0], color: [1.0, 1.0, 1.0, 1.0], }, // v5
    Vertex { pos: [-0.2,  0.6, -0.2, 1.0], color: [1.0, 1.0, 1.0, 1.0], }, // v6
    Vertex { pos: [-0.2, -0.2, -0.2, 1.0], color: [1.0, 1.0, 1.0, 1.0], }, // v7
];
pub const INDEX_DATA: [uint32_t; 72] = [
    // cube 1
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

    // cube 2
    8, 9, 10,
    8, 10, 11,
    8, 11, 12,
    8, 12, 13,
    9, 14, 15,
    9, 15, 10,
    14, 13, 12,
    14, 12, 15,
    13, 14, 9,
    13, 9, 8,
    11, 10, 15,
    11, 15, 12,
];

#[derive(Debug, Clone, Copy)]
pub struct UboObject {
    pub projection: Matrix4<f32>,
    pub view      : Matrix4<f32>,
    pub model     : Matrix4<f32>,
}
