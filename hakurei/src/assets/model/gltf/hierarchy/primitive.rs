
use gltf;
use ash::vk;

use gsvk::types::{ vkuint, vkbytes };

use gsvk::pipeline::shader::HaVertexInputAttribute;
use gsvk::pipeline::shader::HaVertexInputBinding;
use gsvk::pipeline::shader::VertexInputDescription;
use gsvk::memory::AllocatorError;

use gsvk::buffer::instance::{ VertexBlockInfo, HaVertexBlock };
use gsvk::buffer::instance::{ IndexBlockInfo, HaIndexBlock };
use gsvk::memory::transfer::BufferDataUploader;

use assets::model::GltfRawData;
use assets::model::{ ModelGltfLoadingError, GltfAttributeMissing };

pub(crate) struct GltfPrimitive {

    vertexs: Vec<Vertex>,
    indices: Vec<vkuint>,
}

impl GltfPrimitive {

    pub fn from_hierarchy(hierarchy: gltf::Primitive, data: &GltfRawData) -> Result<GltfPrimitive, ModelGltfLoadingError> {

        let buffer_data = &data.buffers;
        let reader = hierarchy.reader(|buffer: gltf::Buffer|
            Some(&buffer_data[buffer.index()])
        );

        // TODO: Reconsider allow missing position.
        let mut positions = reader.read_positions()
            .ok_or(ModelGltfLoadingError::AttriMissing(GltfAttributeMissing::Position))?;

        let vertex_count = positions.len();
        let mut vertexs = Vec::with_capacity(vertex_count);

        for _ in 0..positions.len() {
            use self::ModelGltfLoadingError::AttributeElementCountNotMatch as AecnmError;

            let vertex = Vertex {
                position: positions.next().ok_or(AecnmError)?,
            };

            vertexs.push(vertex);
        }

        let indices = reader.read_indices()
            .ok_or(ModelGltfLoadingError::AttriMissing(GltfAttributeMissing::Index))?
            .into_u32().collect::<Vec<_>>();

        let primitive = GltfPrimitive {
            vertexs,
            indices,
        };

        Ok(primitive)
    }

    #[inline]
    pub fn block_info(&self) -> VertexBlockInfo {
        let data_size = data_size!(self.vertexs, Vertex);
        VertexBlockInfo::new(data_size)
    }

    #[inline]
    pub fn upload_vertex_data(&self, block: &HaVertexBlock, uploader: &mut BufferDataUploader) -> Result<(), AllocatorError> {
        uploader.upload(block, &self.vertexs)?;
        Ok(())
    }

    #[inline]
    pub fn index_info(&self) -> IndexBlockInfo {
        let data_size = data_size!(self.indices, vkuint);
        IndexBlockInfo::new(data_size)
    }

    #[inline]
    pub fn upload_index_data(&self, block: &HaIndexBlock, uploader: &mut BufferDataUploader) -> Result<(), AllocatorError> {
        uploader.upload(block, &self.indices)?;
        Ok(())
    }

    #[inline]
    pub fn index_count(&self) -> usize {
        self.indices.len()
    }
}

define_input! {
    #[binding = 0, rate = vertex]
    struct Vertex {
        #[location = 0, format = vec3]
        position : [f32; 3],
    }
}
