
use crate::assets::glTF::levels::GsglTFNodeEntity;
use crate::assets::glTF::asset::{ GsglTFAssetLib, GsglTFPhyLimits };
use crate::assets::error::GltfError;

use crate::assets::glTF::material::sampler::GsglTFSamplerData;
use crate::assets::glTF::material::texture::GsglTFTextureData;

use crate::assets::glTF::primitive::attributes::{ GsglTFAttributesData, GsglTFAttrFlags };
use crate::assets::glTF::primitive::transforms::{ GsglTFNodesData, GsglTFNodeUniformFlags };
use crate::assets::glTF::primitive::indices::GsglTFIndicesData;

use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::buffer::allocator::{ GsBufferAllocator, GsBufferAllocatable };
use gsvk::buffer::instance::{ IVertex, IIndices, IUniform };

use gsvk::types::{ vkuint, vkbytes };
use gsvk::utils::allot::{ GsAssignIndex, GsAllocatorApi };
use gsvk::utils::phantom::Host;

use gsvk::error::VkResult;

// ------------------------------------------------------------------------------------
pub(crate) struct IntermediateglTFData {
    pub doc: gltf::Document,
    pub data_buffer: Vec<gltf::buffer::Data>,
    pub data_image : Vec<gltf::image::Data>,
    pub limits: GsglTFPhyLimits,
}
// ------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------
pub(crate) struct GsglTFLoadingData {

    attributes: GsglTFAttributesData,
    indices: GsglTFIndicesData,
    node_transforms: GsglTFNodesData,

    textures : GsglTFAssetLib<GsglTFTextureData, GsglTFTextureData>,
    samplers : GsglTFAssetLib<GsglTFSamplerData, GsglTFSamplerData>,
}

pub(crate) struct AttrExtendInfo {

    pub start_vertex: vkuint,
    pub extend_vertex_count: vkuint,
    pub start_offset: vkbytes,
}

pub(crate) struct IndicesExtendInfo {

    pub start_index: vkuint,
    pub extend_indices_count: vkuint,
}

impl GsglTFLoadingData {

    pub fn new(attr_flag: GsglTFAttrFlags, node_flag: GsglTFNodeUniformFlags) -> Result<GsglTFLoadingData, GltfError> {

        let loading_data = GsglTFLoadingData {
            attributes: GsglTFAttributesData::new(attr_flag)?,
            indices: GsglTFIndicesData::default(),
            node_transforms: GsglTFNodesData::new(node_flag)?,

            textures : Default::default(),
            samplers : Default::default(),
        };
        Ok(loading_data)
    }

    pub fn extend_attributes(&mut self, primitive: &gltf::Primitive, source: &IntermediateglTFData) -> Result<AttrExtendInfo, GltfError> {

        let offset = self.attributes.data_size(); // the data offset from the beginning of vertex buffer.
        let data_content = self.attributes.data_content_mut();
        let start_vertex_index = data_content.data_length();

        // perform data reading.
        let extend_count = data_content.extend(primitive, source)?;

        let extend_info = AttrExtendInfo {
            start_vertex: start_vertex_index as _,
            extend_vertex_count: extend_count as _,
            start_offset: offset,
        };
        Ok(extend_info)
    }

    pub fn extend_indices(&mut self, primitive: &gltf::Primitive, source: &IntermediateglTFData) -> Result<IndicesExtendInfo, GltfError> {

        let start_indices_index = self.indices.indices_count();
        let extend_count = self.indices.extend(primitive, source)?;

        let extend_info = IndicesExtendInfo {
            start_index: start_indices_index as _,
            extend_indices_count: extend_count as _,
        };
        Ok(extend_info)
    }

    pub fn extend_transforms(&mut self, node: &GsglTFNodeEntity) {

        let data_content = self.node_transforms.data_content_mut();
        data_content.extend(node);
    }

    pub fn into_storage(self) -> GsglTFDataStorage {

        GsglTFDataStorage {
            attributes: self.attributes,
            indices: self.indices,
            node_transforms: self.node_transforms,
            textures : self.textures.into_data(),
            samplers : self.samplers.into_data(),
        }
    }
}
// ------------------------------------------------------------------------------------

// -----------------------------------------------------------------------------------
pub struct GsglTFDataStorage {

    pub(super) attributes: GsglTFAttributesData,
    pub(super) indices: GsglTFIndicesData,
    pub(super) node_transforms: GsglTFNodesData,

    #[allow(dead_code)]
    textures : Vec<GsglTFTextureData>,
    #[allow(dead_code)]
    samplers : Vec<GsglTFSamplerData>,
}

/// glTF Vertex Data Allocation Delegate.
pub struct GVDADelegate<'d> {
    attributes: &'d GsglTFAttributesData,
    indices: &'d GsglTFIndicesData,
}

/// glTF Uniform Data Allocation Delegate.
pub struct GUDADelegate<'d> {
    uniform_binding: vkuint,
    node_transforms: &'d GsglTFNodesData,
}

impl<'d, 's: 'd> GsglTFDataStorage {

    pub fn vertex_allot_delegate(&'s self) -> GVDADelegate<'d> {
       GVDADelegate {
           attributes: &self.attributes,
           indices: &self.indices,
       }
    }

    pub fn uniform_allot_delegate(&'s self, uniform_binding: vkuint) -> GUDADelegate<'d> {
        GUDADelegate {
            uniform_binding,
            node_transforms: &self.node_transforms,
        }
    }
}

impl<'d, M> GsBufferAllocatable<M, GsglTFVertexAllotIndex> for GVDADelegate<'d>
    where
        M: BufferMemoryTypeAbs {

    fn allot_func(&self) -> Box<dyn Fn(&Self, &mut GsBufferAllocator<M>) -> VkResult<GsglTFVertexAllotIndex>> {

        let func = |data_storage: &GVDADelegate, allocator: &mut GsBufferAllocator<M>| {

            let vertex_info = data_storage.attributes.vertex_info();
            let vertex_index = allocator.assign(vertex_info)?;

            let indices_index = if let Some(indices_info) = data_storage.indices.indices_info() {
                let indices_index = allocator.assign(indices_info)?;
                Some(indices_index)
            } else {
                None
            };

            let allot_index = GsglTFVertexAllotIndex {
                vertex : vertex_index,
                indices: indices_index,
            };
            Ok(allot_index)
        };
        Box::new(func)
    }
}

impl<'d> GsBufferAllocatable<Host, GsglTFUniformAllotIndex> for GUDADelegate<'d> {

    fn allot_func(&self) -> Box<dyn Fn(&Self, &mut GsBufferAllocator<Host>) -> VkResult<GsglTFUniformAllotIndex>> {

        let func = |data_storage: &GUDADelegate, allocator: &mut GsBufferAllocator<Host>| {

            let uniform_info = data_storage.node_transforms.uniform_info(data_storage.uniform_binding);
            let uniform_index = allocator.assign(uniform_info)?;

            let allot_index = GsglTFUniformAllotIndex {
                uniform: uniform_index,
            };
            Ok(allot_index)
        };
        Box::new(func)
    }
}
// ------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------
pub struct GsglTFVertexAllotIndex {
    pub(super) vertex : GsAssignIndex<IVertex>,
    pub(super) indices: Option<GsAssignIndex<IIndices>>,
}

pub struct GsglTFUniformAllotIndex {
    pub(super) uniform: GsAssignIndex<IUniform>,
}
// ------------------------------------------------------------------------------------

