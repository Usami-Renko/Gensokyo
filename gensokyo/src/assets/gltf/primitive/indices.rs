
use gsvk::buffer::allocator::{ GsBufferAllocator, BufferBlockIndex };
use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::buffer::instance::{ IndexBlockInfo, GsIndexBlock };
use gsvk::memory::AllocatorError;
use gsvk::memory::transfer::GsBufferDataUploader;
use gsvk::types::vkuint;
use gsma::data_size;

#[derive(Default)]
pub(super) struct GltfPrimitiveIndices {

    data: Option<Vec<vkuint>>,
}

impl GltfPrimitiveIndices {

    pub fn read<'a, 's, F>(reader: &gltf::mesh::Reader<'a, 's, F>) -> GltfPrimitiveIndices
        where F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]> {

        let data = reader.read_indices()
            .and_then(|index_iter| {
                let data = index_iter.into_u32().collect();
                Some(data)
            });

        GltfPrimitiveIndices { data }
    }

    pub fn indices_count(&self) -> Option<usize> {

        self.data.as_ref()
            .map(|vertex_data| vertex_data.len())
    }

    pub fn append_allocation<M>(&self, allocator: &mut GsBufferAllocator<M>) -> Result<Option<BufferBlockIndex>, AllocatorError>
        where M: BufferMemoryTypeAbs{

        let block_index = if let Some(ref indices_data) = self.data {
            let indices_info = IndexBlockInfo::new(data_size!(indices_data, vkuint));
            Some(allocator.append_buffer(indices_info)?)
        } else {
            None
        };

        Ok(block_index)
    }

    pub fn upload(&self, to: &Option<GsIndexBlock>, by: &mut GsBufferDataUploader) -> Result<(), AllocatorError> {

        if let Some(ref indices_data) = self.data {
            if let Some(ref index_block) = to {
                let _ = by.upload(index_block, indices_data)?;
            }
        }
        Ok(())
    }
}
