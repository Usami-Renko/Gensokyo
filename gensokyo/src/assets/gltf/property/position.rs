
use crate::assets::gltf::property::traits::GltfPrimitiveProperty;
use crate::utils::types::{ Point3F, Matrix4F };

use gsvk::buffer::allocator::{ GsBufferAllocator, BufferBlockIndex };
use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::buffer::instance::{ VertexBlockInfo, GsVertexBlock };
use gsvk::memory::AllocatorError;
use gsvk::memory::transfer::BufferDataUploader;
use gsvk::types::vkbytes;
use gsma::data_size;

use gltf::mesh::Reader;

#[derive(Default)]
pub(crate) struct GltfPropertyPosition {

    data: Option<Vec<Point3F>>,
}

impl GltfPrimitiveProperty for GltfPropertyPosition {
    const PROPERTY_NAME: &'static str = "POSITION";

    type IndexType = BufferBlockIndex;
    type BlockType = GsVertexBlock;

    fn read<'a, 's, F>(reader: &Reader<'a, 's, F>) -> Self
        where F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]> {

        let data = reader.read_positions()
            .and_then(|pos_iter| {
                let data = pos_iter.map(Point3F::from).collect();
                Some(data)
            });

        GltfPropertyPosition { data }
    }

    fn append_allocation<M>(&self, allocator: &mut GsBufferAllocator<M>) -> Result<Option<Self::IndexType>, AllocatorError>
        where M: BufferMemoryTypeAbs {

        let block_index = if let Some(ref vertex_data) = self.data {
            let vertex_info = VertexBlockInfo::new(data_size!(vertex_data, Point3F));
            Some(allocator.append_buffer(vertex_info)?)
        } else {
            None
        };

        Ok(block_index)
    }

    fn upload<M>(&self, to: &Option<Self::BlockType>, by: &mut BufferDataUploader<M>) -> Result<(), AllocatorError>
        where M: BufferMemoryTypeAbs {

        if let Some(ref vertex_data) = self.data {
            if let Some(ref vertex_block) = to {
                let _ = by.upload(vertex_block, vertex_data)?;
            }
        }
        Ok(())
    }
}

impl GltfPropertyPosition {

    pub fn apply_transform(&self, transform: &Matrix4F) -> GltfPropertyPosition {

        let data_transformed = self.data.as_ref().and_then(|vertex_data| {
            let data = vertex_data.iter()
                .map(|pos| transform.transform_point(pos)).collect();
            Some(data)
        });

        GltfPropertyPosition { data: data_transformed }
    }

    pub fn vertex_count(&self) -> Option<usize> {

        self.data.as_ref()
            .map(|vertex_data| vertex_data.len())
    }
}
