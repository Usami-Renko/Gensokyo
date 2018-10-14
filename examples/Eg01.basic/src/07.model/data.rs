
use hakurei::prelude::*;
use hakurei::prelude::pipeline::*;

use hakurei::prelude::utility::*;

use cgmath::Matrix4;

define_input! {
    #[binding = 0, rate = vertex]
    struct Vertex {
        #[location = 0, format = vec4]
        pos: [f32; 4],
        #[location = 1, format = vec2]
        tex_coord: [f32; 2],
    }
}

pub struct ModelData {

    pub vertices: Vec<Vertex>,
    pub indices : Vec<uint32_t>,
}

impl ModelData {

    pub fn empty() -> ModelData {
        ModelData {
            vertices: vec![],
            indices : vec![],
        }
    }
}

impl ObjDataEntity for ModelData {

    fn init_vertices_capacity(&mut self, vertex_amount: usize) {
        self.vertices = Vec::with_capacity(vertex_amount);
    }

    fn fill_vertex(&mut self, pos_x: f32, pos_y: f32, pos_z: f32, tex_x: f32, tex_y: f32) {

        let vertex = Vertex {
            pos: [pos_x, pos_y, pos_z, 1.0],
            tex_coord: [tex_x, 1.0 - tex_y],
        };
        self.vertices.push(vertex);
    }

    fn fill_indices(&mut self, indices: Vec<u32>) {

        self.indices = indices;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UboObject {
    pub projection: Matrix4<f32>,
    pub view      : Matrix4<f32>,
    pub model     : Matrix4<f32>,
}
