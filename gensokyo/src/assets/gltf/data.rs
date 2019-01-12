
use crate::assets::glTF::importer::GsglTFEntity;
use crate::assets::glTF::levels::GsglTFLevelEntity;
use crate::assets::glTF::error::GltfError;

use crate::assets::glTF::material::material::GsglTFMaterialData;
use crate::assets::glTF::material::sampler::GsglTFSamplerData;
use crate::assets::glTF::material::texture::GsglTFTextureData;

use crate::assets::glTF::primitive::attributes::GsglTFAttributesData;
use crate::assets::glTF::primitive::indices::GsglTFIndicesData;
use crate::assets::glTF::primitive::templates::GsglTFAttrFlag;

use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::buffer::allocator::{ GsBufferAllocator, GsBufferAllocatable, BufferBlockIndex };
use gsvk::buffer::allocator::GsBufferDistributor;
use gsvk::buffer::instance::{ GsVertexBlock, GsIndexBlock };
use gsvk::memory::transfer::{ GsBufferDataUploader, GsBufferUploadable };
use gsvk::memory::AllocatorError;
use gsvk::command::GsCommandRecorder;
use gsvk::types::{ vkuint, vkbytes };

// ------------------------------------------------------------------------------------
pub(crate) struct IntermediateglTFData {
    pub doc: gltf::Document,
    pub data_buffer: Vec<gltf::buffer::Data>,
    pub data_image : Vec<gltf::image::Data>,
}
// ------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------
pub(crate) struct GsglTFLoadingData {

    attributes: GsglTFAttributesData,
    indices: GsglTFIndicesData,

    materials: Vec<GsglTFMaterialData>,
    textures: Vec<GsglTFTextureData>,
    samplers: Vec<GsglTFSamplerData>,
}

pub(crate) struct AttrExtendInfo {

    pub start_index: vkuint,
    pub extend_vertex_count: vkuint,
    pub start_offset: vkbytes,
}

pub(crate) struct IndicesExtendInfo {

    pub start_index: vkuint,
    pub extend_indices_count: vkuint,
}

impl GsglTFLoadingData {

    pub fn new(attr_flag: GsglTFAttrFlag) -> Result<GsglTFLoadingData, GltfError> {

        let loading_data = GsglTFLoadingData {
            attributes: GsglTFAttributesData::new(attr_flag)?,
            indices   : GsglTFIndicesData::default(),

            materials: vec![],
            textures : vec![],
            samplers : vec![],
        };
        Ok(loading_data)
    }

    pub fn extend_attributes(&mut self, primitive: &gltf::Primitive, source: &IntermediateglTFData) -> Result<AttrExtendInfo, GltfError> {

        let offset = self.attributes.data_size(); // the data offset from the beginning of vertex buffer.
        let start_vertex_index = self.attributes.content.data_length();

        // perform data reading.
        let extend_count = self.attributes.content.extend(primitive, source)?;

        let extend_info = AttrExtendInfo {
            start_index: start_vertex_index as _,
            extend_vertex_count: extend_count as _,
            start_offset: offset,
        };
        Ok(extend_info)
    }

    pub fn extend_indices<'a, 's, F>(&mut self, reader: &gltf::mesh::Reader<'a, 's, F>) -> Result<IndicesExtendInfo, GltfError>
        where F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]> {

        let start_indices_index = self.indices.indices_count();
        let extend_count = self.indices.extend(reader)?;

        let extend_info = IndicesExtendInfo {
            start_index: start_indices_index as _,
            extend_indices_count: extend_count as _,
        };
        Ok(extend_info)
    }

    pub fn into_storage(self) -> GsglTFDataStorage {

        GsglTFDataStorage {
            attributes: self.attributes,
            indices   : self.indices,
            materials : self.materials,
            textures  : self.textures,
            samplers  : self.samplers,
        }
    }
}
// ------------------------------------------------------------------------------------

// -----------------------------------------------------------------------------------
pub struct GsglTFDataStorage {

    attributes: GsglTFAttributesData,
    indices   : GsglTFIndicesData,

    materials: Vec<GsglTFMaterialData>,
    textures : Vec<GsglTFTextureData>,
    samplers : Vec<GsglTFSamplerData>,
}

impl<M> GsBufferAllocatable<M, GsglTFAllotIndex> for GsglTFDataStorage where M: BufferMemoryTypeAbs {

    fn allot_func(&self) -> Box<dyn Fn(&Self, &mut GsBufferAllocator<M>) -> Result<GsglTFAllotIndex, AllocatorError>> {

        let func = |data_storage: &GsglTFDataStorage, allocator: &mut GsBufferAllocator<M>| {

            let vertex_info = data_storage.attributes.vertex_info();
            let vertex_index = allocator.append_buffer(vertex_info)?;

            let indices_index = if let Some(indices_info) = data_storage.indices.indices_info() {
                let indices_index = allocator.append_buffer(indices_info)?;
                Some(indices_index)
            } else {
                None
            };

            let allot_index = GsglTFAllotIndex {
                vertex : vertex_index,
                indices: indices_index,
            };
            Ok(allot_index)
        };
        Box::new(func)
    }
}
// ------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------
pub struct GsglTFAllotIndex {

    vertex : BufferBlockIndex,
    indices: Option<BufferBlockIndex>,
}
// ------------------------------------------------------------------------------------


// ------------------------------------------------------------------------------------
pub struct GsglTFModel {

    entity: GsglTFEntity,

    vertex : GsVertexBlock,
    indices: Option<GsIndexBlock>,
}

impl GsglTFEntity {

    pub fn assign<M>(self, at: GsglTFAllotIndex, by: &GsBufferDistributor<M>) -> GsglTFModel
        where M: BufferMemoryTypeAbs {

        let indices = at.indices.and_then(|indices_index| {
            Some(by.acquire_index(indices_index))
        });

        GsglTFModel {
            entity: self,
            vertex: by.acquire_vertex(at.vertex),
            indices,
        }
    }
}

impl GsBufferUploadable<GsglTFDataStorage> for GsglTFModel {

    fn upload_func(&self) -> Box<dyn Fn(&Self, &mut GsBufferDataUploader, &GsglTFDataStorage) -> Result<(), AllocatorError>> {

        let upload_func = |model: &GsglTFModel, by: &mut GsBufferDataUploader, data: &GsglTFDataStorage| {

            // upload vertex data.
            data.attributes.content.upload(&model.vertex, by)?;
            // upload index data.
            data.indices.upload(&model.indices, by)?;

            Ok(())
        };
        Box::new(upload_func)
    }
}

impl GsglTFModel {

    pub fn record_command(&self, recorder: &GsCommandRecorder) {

        // bind the whole vertex buffer.
        recorder.bind_vertex_buffers(0, &[&self.vertex]);

        // bind the whole index buffer.
        if let Some(ref indices_block) = self.indices {
            recorder.bind_index_buffer(indices_block, 0);
        }

        // call the draw command.
        self.entity.scene.record_command(recorder);
    }
}
// ------------------------------------------------------------------------------------
