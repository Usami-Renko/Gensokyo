
use crate::assets::gltf::property::traits::GltfPrimitiveProperty;

use gsvk::buffer::allocator::{ GsBufferAllocator, BufferBlockIndex };
use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::buffer::instance::{ IndexBlockInfo, GsIndexBlock };
use gsvk::memory::AllocatorError;
use gsvk::memory::transfer::BufferDataUploader;
use gsvk::types::{ vkuint, vkbytes };
use gsma::data_size;

use gltf::mesh::Reader;

#[derive(Default)]
pub(crate) struct GltfPropertyIndices {

    data: Option<Vec<vkuint>>,
}

impl GltfPrimitiveProperty for GltfPropertyIndices {
    const PROPERTY_NAME: &'static str = "indices";

    type IndexType = BufferBlockIndex;
    type BlockType = GsIndexBlock;

    fn read<'a, 's, F>(reader: &Reader<'a, 's, F>) -> Self
        where F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]> {

        let data = reader.read_indices()
            .and_then(|index_iter| {
                let data = index_iter.into_u32().collect();
                Some(data)
            });

        GltfPropertyIndices { data }
    }

    fn append_allocation<M>(&self, allocator: &mut GsBufferAllocator<M>) -> Result<Option<Self::IndexType>, AllocatorError>
        where M: BufferMemoryTypeAbs {

        let block_index = if let Some(ref indices_data) = self.data {
            let indices_info = IndexBlockInfo::new(data_size!(indices_data, vkuint));
            Some(allocator.append_buffer(indices_info)?)
        } else {
            None
        };

        Ok(block_index)
    }

    fn upload<M>(&self, to: &Option<Self::BlockType>, by: &mut BufferDataUploader<M>) -> Result<(), AllocatorError>
        where M: BufferMemoryTypeAbs {

        if let Some(ref indices_data) = self.data {
            if let Some(ref index_block) = to {
                let _ = by.upload(index_block, indices_data)?;
            }
        }
        Ok(())
    }
}

impl GltfPropertyIndices {

    pub fn indices_count(&self) -> Option<usize> {

        self.data.as_ref()
            .map(|vertex_data| vertex_data.len())
    }
}
