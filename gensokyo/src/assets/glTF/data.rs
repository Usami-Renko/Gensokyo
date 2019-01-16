
use crate::assets::glTF::importer::GsglTFEntity;
use crate::assets::glTF::levels::GsglTFNodeEntity;
use crate::assets::glTF::asset::{ GsglTFAssetLib, GsglTFPhyLimits };
use crate::assets::glTF::error::GltfError;

use crate::assets::glTF::material::material::MaterialConstants;
use crate::assets::glTF::material::sampler::GsglTFSamplerData;
use crate::assets::glTF::material::texture::GsglTFTextureData;

use crate::assets::glTF::primitive::attributes::{ GsglTFAttributesData, GsglTFAttrFlags };
use crate::assets::glTF::primitive::transforms::{ GsglTFNodesData, GsglTFNodeUniformFlags };
use crate::assets::glTF::primitive::indices::GsglTFIndicesData;

use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::buffer::allocator::{ GsBufferAllocator, GsBufferAllocatable, GsBufferDistributor };
use gsvk::buffer::instance::{ GsVertexBuffer, IVertex };
use gsvk::buffer::instance::{ GsIndexBuffer, IIndices };
use gsvk::buffer::instance::{ GsUniformBuffer, IUniform };

use gsvk::memory::transfer::{ GsBufferDataUploader, GsBufferUploadable };
use gsvk::memory::AllocatorError;

use gsvk::pipeline::target::GsPipelineStage;
use gsvk::pipeline::layout::GsPushConstantRange;

use gsvk::command::{ GsCmdRecorder, GsCmdGraphicsApi, CmdDescriptorSetBindInfo };
use gsvk::descriptor::{ DescriptorSet, DescriptorBufferBindableTarget, DescriptorBufferBindingInfo };

use gsvk::utils::assign::GsAssignIndex;
use gsvk::utils::phantom::{ Graphics, Host };

use gsvk::types::{ vkuint, vkbytes };

use std::mem;

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

    pub start_index: vkuint,
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

    attributes: GsglTFAttributesData,
    indices: GsglTFIndicesData,
    node_transforms: GsglTFNodesData,

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

impl<'d, M> GsBufferAllocatable<M, GsglTFVertexAllotIndex> for GVDADelegate<'d> where M: BufferMemoryTypeAbs {

    fn allot_func(&self) -> Box<dyn Fn(&Self, &mut GsBufferAllocator<M>) -> Result<GsglTFVertexAllotIndex, AllocatorError>> {

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

    fn allot_func(&self) -> Box<dyn Fn(&Self, &mut GsBufferAllocator<Host>) -> Result<GsglTFUniformAllotIndex, AllocatorError>> {

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
    vertex : GsAssignIndex<IVertex>,
    indices: Option<GsAssignIndex<IIndices>>,
}

pub struct GsglTFUniformAllotIndex {
    uniform: GsAssignIndex<IUniform>,
}
// ------------------------------------------------------------------------------------


// ------------------------------------------------------------------------------------
pub struct GsglTFModel {

    entity: GsglTFEntity,

    vertex : GsVertexBuffer,
    indices: Option<GsIndexBuffer>,
    uniform: GsUniformBuffer,
}

pub(crate) struct GsglTFCmdRecordInfo<'i> {
    pub binding_sets: Vec<CmdDescriptorSetBindInfo<'i>>,
    pub uniform_aligned_size: vkbytes,
    pub gltf_uniform_index: usize,
}

impl GsglTFEntity {

    pub fn assign<M>(self,

         vertex_index : GsglTFVertexAllotIndex,  vertex_distributor: &GsBufferDistributor<M>,
         uniform_index: GsglTFUniformAllotIndex, uniform_distributor: &GsBufferDistributor<Host>)

        -> GsglTFModel where M: BufferMemoryTypeAbs {

        GsglTFModel {
            entity: self,
            vertex: vertex_distributor.acquire_vertex(vertex_index.vertex),
            indices: vertex_index.indices.and_then(|indices_index| {
                Some(vertex_distributor.acquire_index(indices_index))
            }),
            uniform: uniform_distributor.acquire_uniform(uniform_index.uniform),
        }
    }
}

impl<'d, 's: 'd> GsglTFModel {

    pub fn vertex_upload_delegate(&'s self) -> GVDUDelegate<'d> {
        GVDUDelegate {
            vertex : &self.vertex,
            indices: &self.indices,
        }
    }

    pub fn uniform_upload_delegate(&'s self) -> GUDUDelegate<'d> {
        GUDUDelegate {
            uniform: &self.uniform,
        }
    }

    pub fn record_command<'i>(&self, recorder: &GsCmdRecorder<Graphics>, gltf_set: &DescriptorSet, binding_sets: Vec<CmdDescriptorSetBindInfo<'i>>) {

        // bind the whole vertex buffer.
        recorder.bind_vertex_buffers(0, &[&self.vertex]);

        // bind the whole index buffer.
        if let Some(ref indices_block) = self.indices {
            recorder.bind_index_buffer(indices_block, 0);
        }

        // Prepare binding DescriptorSets.
        let uniform_index = binding_sets.len(); // get the location of glTF descriptorSet.
        let mut binding_sets = binding_sets; // make it mutable.

        binding_sets.push(CmdDescriptorSetBindInfo {
            set: gltf_set,
            dynamic_offset: None,
        });

        let mut record_info = GsglTFCmdRecordInfo {
            binding_sets,
            uniform_aligned_size: self.uniform.alignment_size(),
            gltf_uniform_index: uniform_index,
        };

        // call the draw command.
        self.entity.scene.record_command(recorder, &mut record_info);
    }

    pub fn pushconst_description(&self) -> GsPushConstantRange {
        GsPushConstantRange::new(GsPipelineStage::FRAGMENT, 0, mem::size_of::<MaterialConstants>() as vkuint)
    }
}

impl DescriptorBufferBindableTarget for GsglTFModel {

    fn binding_info(&self, sub_block_indices: Option<Vec<vkuint>>) -> DescriptorBufferBindingInfo {
        self.uniform.binding_info(sub_block_indices)
    }
}
// ------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------
/// glTF Vertex Data Upload Delegate.
pub struct GVDUDelegate<'d> {
    vertex : &'d GsVertexBuffer,
    indices: &'d Option<GsIndexBuffer>,
}

// glTF Uniform Data Upload Delegate.
pub struct GUDUDelegate<'d> {
    uniform: &'d GsUniformBuffer,
}

impl<'d> GsBufferUploadable<GsglTFDataStorage> for GVDUDelegate<'d> {

    fn upload_func(&self) -> Box<dyn Fn(&Self, &mut GsBufferDataUploader, &GsglTFDataStorage) -> Result<(), AllocatorError>> {

        let upload_func = |model: &GVDUDelegate, by: &mut GsBufferDataUploader, data: &GsglTFDataStorage| {

            // upload vertex data.
            data.attributes.data_content().upload(model.vertex, by)?;
            // upload index data.
            data.indices.upload(model.indices, by)?;

            Ok(())
        };
        Box::new(upload_func)
    }
}

impl<'d> GsBufferUploadable<GsglTFDataStorage> for GUDUDelegate<'d> {

    fn upload_func(&self) -> Box<dyn Fn(&Self, &mut GsBufferDataUploader, &GsglTFDataStorage) -> Result<(), AllocatorError>> {

        let upload_func = |model: &GUDUDelegate, by: &mut GsBufferDataUploader, data: &GsglTFDataStorage| {

            // upload uniform data.
            let element_alignment = model.uniform.dyn_alignment().unwrap(); // unwrap() should always succeed here.
            data.node_transforms.data_content().upload(model.uniform, by, element_alignment)
        };
        Box::new(upload_func)
    }
}
// ------------------------------------------------------------------------------------
