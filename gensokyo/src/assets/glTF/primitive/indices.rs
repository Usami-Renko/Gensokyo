
use crate::assets::glTF::data::IntermediateglTFData;
use crate::assets::error::GltfError;

use gsvk::buffer::instance::{ GsIndexBuffer, IndicesBufferCI };
use gsvk::memory::transfer::GsBufferDataUploader;
use gsvk::types::vkuint;
use gsvk::error::VkResult;

pub(crate) struct GsglTFIndicesData {

    start_index: u32,
    data: Vec<vkuint>,
}

impl GsglTFIndicesData {

    pub fn extend(&mut self, primitive: &gltf::Primitive, source: &IntermediateglTFData) -> Result<usize, GltfError> {

        let reader = primitive.reader(|b| Some(&source.data_buffer[b.index()]));
        let indices_range = get_indices_range(primitive)?;
        let start_index = self.start_index.clone();

        // TODO: Support other integer type.
        let index_iter = reader.read_indices()
            .ok_or(GltfError::loading("Missing indices property in glTF primitive."))?
            .into_u32()
            .map(move |index_element| index_element + start_index);
        let (extend_count, _) = index_iter.size_hint();

        self.data.extend(index_iter);
        self.start_index += indices_range as u32;

        Ok(extend_count)
    }

    pub fn indices_info(&self) -> Option<IndicesBufferCI> {

        if self.is_contain_indices() {
            Some(GsIndexBuffer::new(self.data.len() as _))
        } else {
            None
        }
    }

    #[inline]
    fn is_contain_indices(&self) -> bool {
        !self.data.is_empty()
    }

    #[inline]
    pub fn indices_count(&self) -> usize {
        self.data.len()
    }

    pub fn upload(&self, to: &Option<GsIndexBuffer>, by: &mut GsBufferDataUploader) -> VkResult<()> {

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

impl Default for GsglTFIndicesData {

    fn default() -> GsglTFIndicesData {
        GsglTFIndicesData {
            start_index: 0,
            data: Vec::new(),
        }
    }
}

fn get_indices_range(primitive: &gltf::Primitive) -> Result<u64, GltfError> {

    let indices_accessor = primitive.indices()
        .ok_or(GltfError::loading("Failed to get indices property of gltf::Primitive"))?;

    // Get the maximum index of this primitive.
    let index_max = get_index(indices_accessor.max())?;
    let index_min = get_index(indices_accessor.min())?;
    let indices_range = index_max - index_min + 1;

    Ok(indices_range)
}

fn get_index(value: Option<gltf::json::Value>) -> Result<u64, GltfError> {

    value
        .and_then(|v| v.as_array().cloned())
        .and_then(|v| v.first().cloned())
        .and_then(|v| v.as_u64())
        .ok_or(GltfError::loading(""))
}
