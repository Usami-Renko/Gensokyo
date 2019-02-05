
use gsvk::prelude::common::*;
use gsvk::prelude::buffer::*;
use gsvk::prelude::api::*;
use gs::prelude::*;

use gsma::data_size;

pub struct QuadAsset {

    vertex : GsVertexBuffer,
    indices: GsIndexBuffer,

    repository: GsBufferRepository<Host>,
}

struct Vertex {

    pos     : [f32; 3],
    uv      : [f32; 2],
    color   : [f32; 3],
    normal  : [f32; 3],
    tangent : [f32; 3],
}

impl QuadAsset {

    pub fn generate(initializer: &AssetInitializer) -> GsResult<QuadAsset> {

        let mut buffer_allocator = GsBufferAllocator::new(initializer, BufferStorageType::HOST);

        let vertex_data = generate_vertex_data();
        let vertex_ci = GsVertexBuffer::new(data_size!(Vertex), vertex_data.len());
        buffer_allocator.assign(vertex_ci)?;

        let indices_data = generate_indices_data();
        let indices_ci = GsIndexBuffer::new(indices_data.len());
        buffer_allocator.assign(indices_ci)?;

        let distributor = buffer_allocator.allocate()?;

        let vertex_buffer = distributor.acquire(vertex_ci);
        let index_buffer  = distributor.acquire(indices_ci);

        let mut buffer_repository = distributor.into_repository();

        buffer_repository.data_uploader()?
            .upload(&vertex_buffer, &vertex_data)?
            .upload(&index_buffer, &indices_data)?
            .finish()?;

        let result = QuadResource {
            vertex : vertex_buffer,
            indices: index_buffer,
            repository: buffer_repository,
        };
        Ok(result)
    }
}

fn generate_vertex_data() -> Vec<Vertex> {

    let mut x = 0.0_f32;
    let mut y = 0.0_f32;

    let mut vertex_data = Vec::new();
    for i in 0..3_u32 {

        // Last component of normal is used for debug display sampler index.
        vertex_data.extend([
            Vertex { pos: [x + 1.0, y + 1.0, 0.0], uv: [1.0, 1.0], color: [1.0; 3], normal: [0.0, 0.0, i as f32], tangent: [0.0; 3] },
            Vertex { pos: [x      , y + 1.0, 0.0], uv: [0.0, 1.0], color: [1.0; 3], normal: [0.0, 0.0, i as f32], tangent: [0.0; 3] },
            Vertex { pos: [x      , y      , 0.0], uv: [0.0, 0.0], color: [1.0; 3], normal: [0.0, 0.0, i as f32], tangent: [0.0; 3] },
            Vertex { pos: [x + 1.0, y      , 0.0], uv: [1.0, 0.0], color: [1.0; 3], normal: [0.0, 0.0, i as f32], tangent: [0.0; 3] },
        ]);

        x += 1.0;

        if x > 1.0 {
            x = 0.0;
            y += 1.0;
        }
    }

    vertex_data
}

fn generate_indices_data() -> Vec<u32> {

    let mut indices_data = vec![ 0,1,2, 2,3,0 ];

    for i in 0..3 {

        for &index in [0,1,2, 2,3,0].iter() {
            indices_data.push(i * 4 + index);
        }
    }

    indices_data
}
