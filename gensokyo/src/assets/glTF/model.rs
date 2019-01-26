
use crate::assets::glTF::levels::GsglTFSceneEntity;
use crate::assets::glTF::data::{ GsglTFDataStorage };
use crate::assets::glTF::data::{ GsglTFVertexAllotIndex, GsglTFUniformAllotIndex };
use crate::assets::glTF::material::material::MaterialConstants;
use crate::assets::error::{ AssetsError, GltfError };
use crate::error::{ GsResult, GsError };

use gsvk::buffer::instance::{ GsVertexBuffer, GsIndexBuffer, GsUniformBuffer };
use gsvk::buffer::allocator::GsBufferDistributor;
use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;

use gsvk::pipeline::target::GsPipelineStage;
use gsvk::pipeline::layout::GsPushConstantRange;

use gsvk::command::{ GsCmdRecorder, GsCmdGraphicsApi, CmdDescriptorSetBindInfo };
use gsvk::descriptor::{ DescriptorSet, DescriptorBufferBindableTarget, DescriptorBufferBindingInfo };
use gsvk::memory::transfer::{ GsBufferDataUploader, GsBufferUploadable };

use gsvk::utils::allot::GsDistributeApi;
use gsvk::utils::phantom::{ Graphics, Host };

use gsvk::types::{ vkuint, vkbytes };
use gsvk::error::VkResult;

// ------------------------------------------------------------------------------------
pub struct GsglTFEntity {

    pub(super) scene: GsglTFSceneEntity,

    vertex : Option<GsVertexBuffer>,
    indices: Option<GsIndexBuffer>,
    uniform: Option<GsUniformBuffer>,
}

impl<'d, 's: 'd> GsglTFEntity {

    pub(super) fn new(scene: GsglTFSceneEntity) -> GsglTFEntity {
        GsglTFEntity { scene, vertex : None, indices: None, uniform: None }
    }

    pub fn acquire_vertex<M>(&mut self, vertex_index: GsglTFVertexAllotIndex, distributor: &GsBufferDistributor<M>)
        where M: BufferMemoryTypeAbs {

        self.vertex = Some(distributor.acquire(vertex_index.vertex));
        self.indices = vertex_index.indices.and_then(|indices_index| {
            Some(distributor.acquire(indices_index))
        });
    }

    pub fn acquire_uniform(&mut self, uniform_index: GsglTFUniformAllotIndex, distributor: &GsBufferDistributor<Host>) {

        self.uniform = Some(distributor.acquire(uniform_index.uniform));
    }

    pub fn vertex_upload_delegate(&'s self) -> Option<GVDUDelegate<'d>> {

        self.vertex.as_ref().and_then(|vertex_buffer| {
            let delegate = GVDUDelegate {
                vertex : &vertex_buffer,
                indices: &self.indices,
            };
            Some(delegate)
        })
    }

    pub fn uniform_upload_delegate(&'s self) -> Option<GUDUDelegate<'d>> {

        self.uniform.as_ref().and_then(|uniform_buffer| {
            Some(GUDUDelegate { uniform: uniform_buffer })
        })
    }

    pub fn record_command<'i>(&self, recorder: &GsCmdRecorder<Graphics>, gltf_set: &DescriptorSet, other_sets: &[CmdDescriptorSetBindInfo<'i>], params: Option<GsglTFRenderParams>) -> GsResult<()> {

        let render_params = params.unwrap_or(
            GsglTFRenderParams {
                is_use_vertex: self.vertex.is_some(),
                is_use_node_transform: self.uniform.is_some(),
                is_push_materials: true,
                material_stage: GsPipelineStage::FRAGMENT,
            }
        );

        // 1.bind vertex data ---------------------------------------------------------
        if render_params.is_use_vertex {
            if let Some(ref vertex_buffer) = self.vertex {
                // bind the whole vertex buffer.
                recorder.bind_vertex_buffers(0, &[vertex_buffer]);

                // bind the whole index buffer.
                if let Some(ref indices_block) = self.indices {
                    recorder.bind_index_buffer(indices_block, 0);
                }
            }
        } else {
            return Err(GsError::assets(AssetsError::Gltf(GltfError::loading("Vertex Buffer must be set(by calling `GsglTFEntity::acquire_vertex()` func) before recording command."))))
        }
        // ----------------------------------------------------------------------------

        // 2.prepare binding DescriptorSets. ------------------------------------------
        let mut binding_sets = Vec::with_capacity(other_sets.len() + 1);
        let mut dynamic_offsets = vec![];

        for set in other_sets.into_iter() {
            binding_sets.push(set.set);

            if let Some(dynamic) = set.dynamic_offset {
                dynamic_offsets.push(dynamic);
            }
        }
        binding_sets.push(gltf_set);

        let gltf_dynamics_index = if render_params.is_use_node_transform {
            let index = dynamic_offsets.len();
            dynamic_offsets.push(0);
            index
        } else {
            0
        };

        let uniform_aligned_size = if let Some(ref b) = self.uniform {
            b.aligned_size()
        } else {
            0
        };

        let mut record_info = GsglTFCmdRecordInfo {
            binding_sets, dynamic_offsets, uniform_aligned_size, gltf_dynamics_index,
        };
        // ----------------------------------------------------------------------------

        // 3.call the draw command. ---------------------------------------------------
        self.scene.record_command(recorder, &mut record_info, &render_params);
        // ----------------------------------------------------------------------------

        Ok(())
    }

    pub fn pushconst_description(&self, stage: GsPipelineStage) -> GsPushConstantRange {
        use std::mem;
        // TODO: Fix stage.
        GsPushConstantRange::new(stage, 0, mem::size_of::<MaterialConstants>() as vkuint)
    }
}

impl DescriptorBufferBindableTarget for GsglTFEntity {

    fn binding_info(&self, sub_block_indices: Option<Vec<vkuint>>) -> DescriptorBufferBindingInfo {
        if let Some(ref uniform_buffer) = self.uniform {
            uniform_buffer.binding_info(sub_block_indices)
        } else {
            panic!("Uniform Buffer must be set(GsglTFEntity::acquire_uniform()) before calling this function.")
        }
    }
}
// ------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------
pub(crate) struct GsglTFCmdRecordInfo<'i> {
    /// The descriptor sets used during the glTF model rendering.
    pub binding_sets: Vec<&'i DescriptorSet>,
    /// The dynamic offsets used in Descriptor set binding.
    pub dynamic_offsets: Vec<vkuint>,
    /// Specify `uniform_aligned_size` to None to disable binding descriptor to shader for this model.
    pub uniform_aligned_size: vkbytes,
    /// The index value of dynamic offset of this model in `dynamic_offsets`.
    pub gltf_dynamics_index: usize,
}

#[derive(Debug, Clone)]
pub struct GsglTFRenderParams {
    pub is_use_vertex: bool,
    pub is_use_node_transform: bool,
    pub is_push_materials: bool,
    pub material_stage: GsPipelineStage,
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

    fn upload_func(&self) -> Box<dyn Fn(&Self, &mut GsBufferDataUploader, &GsglTFDataStorage) -> VkResult<()>> {

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

    fn upload_func(&self) -> Box<dyn Fn(&Self, &mut GsBufferDataUploader, &GsglTFDataStorage) -> VkResult<()>> {

        let upload_func = |model: &GUDUDelegate, by: &mut GsBufferDataUploader, data: &GsglTFDataStorage| {

            // upload uniform data.
            let element_alignment = model.uniform.require_dynamic_alignment();
            data.node_transforms.data_content().upload(model.uniform, by, element_alignment)
        };
        Box::new(upload_func)
    }
}
// ------------------------------------------------------------------------------------
