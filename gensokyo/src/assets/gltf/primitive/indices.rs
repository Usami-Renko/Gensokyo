
use crate::assets::glTF::error::GltfError;

use gsvk::buffer::instance::{ IndexBlockInfo, GsIndexBlock };
use gsvk::memory::transfer::GsBufferDataUploader;
use gsvk::memory::AllocatorError;
use gsvk::types::vkuint;

use gsma::data_size;

#[derive(Default)]
pub struct GsglTFIndicesData {

    data: Vec<vkuint>,
}

impl GsglTFIndicesData {

    pub fn extend<'a, 's, F>(&mut self, reader: &gltf::mesh::Reader<'a, 's, F>) -> Result<usize, GltfError>
        where F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]> {

        let new_indices: Vec<vkuint> = reader.read_indices()
            .and_then(|index_iter| {
                Some(index_iter.into_u32().collect())
            }).ok_or(GltfError::ModelContentMissing)?;

        let extend_count = new_indices.len();

        self.data.extend(new_indices);

        Ok(extend_count)
    }

    pub fn indices_info(&self) -> Option<IndexBlockInfo> {

        if self.is_contain_indices() {
            Some(IndexBlockInfo::new(data_size!(self.data, vkuint)))
        } else {
            None
        }
    }

    fn is_contain_indices(&self) -> bool {
        !self.data.is_empty()
    }

    pub fn indices_count(&self) -> usize {
        self.data.len()
    }

    pub fn upload(&self, to: &Option<GsIndexBlock>, by: &mut GsBufferDataUploader) -> Result<(), AllocatorError> {

        if self.is_contain_indices() {
            if let Some(ref index_block) = to {
                let _ = by.upload(index_block, &self.data)?;
            } else {
                unreachable!()
            }
        }

        Ok(())
    }
}
