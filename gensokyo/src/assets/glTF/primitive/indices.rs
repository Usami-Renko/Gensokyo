
use crate::assets::glTF::data::IntermediateglTFData;
use crate::assets::error::GltfError;

use gsvk::buffer::instance::{ GsBufIndicesInfo, GsIndexBuffer };
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

        // TODO: Support other integer type.
        let new_indices = reader.read_indices()
            .and_then(|index_iter| {

                let result = index_iter.into_u32()
                    .map(|index_element| index_element + self.start_index).collect();
                Some(result)
            }).unwrap_or(Vec::new());

        let extend_count = new_indices.len();
        // println!("indices: {:?}", new_indices);

        self.start_index += indices_range as u32;
        self.data.extend(new_indices);

        Ok(extend_count)
    }

    pub fn indices_info(&self) -> Option<GsBufIndicesInfo> {

        if self.is_contain_indices() {
            Some(GsBufIndicesInfo::new(self.data.len() as _))
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
