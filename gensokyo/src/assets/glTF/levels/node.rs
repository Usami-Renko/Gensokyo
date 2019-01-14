
use crate::assets::glTF::data::{ IntermediateglTFData, GsglTFLoadingData, GsglTFCmdRecordInfo };
use crate::assets::glTF::levels::traits::{ GsglTFLevelEntity, GsglTFArchitecture };
use crate::assets::glTF::levels::mesh::GsglTFMeshEntity;
use crate::assets::glTF::primitive::attributes::GsglTFAttrFlags;
use crate::assets::glTF::primitive::transforms::GsglTFNodeUniformFlags;
use crate::assets::glTF::error::GltfError;
use crate::utils::types::Matrix4F;

use gsvk::command::{ GsCmdRecorder, GsCmdGraphicsApi };
use gsvk::utils::phantom::Graphics;
use gsvk::types::vkuint;

// --------------------------------------------------------------------------------------
/// A wrapper class for node level in glTF, containing the render parameters read from glTF file.
pub(crate) struct GsglTFNodeEntity {

    local_mesh: Option<GsglTFMeshEntity>,
    children: Vec<Box<GsglTFNodeEntity>>,

    // the drawing order of node hierarchy.
    // only node with `local_mesh` contains draw_order property.
    draw_order: Option<usize>,
    /// the transform property of current node.
    pub transform: Matrix4F,
}

impl GsglTFNodeEntity {

    /// Apply parent node's transformation to current node level.
    pub(super) fn combine_transform(&mut self, parent_transform: &Matrix4F) {
        self.transform = self.transform * parent_transform;

        for child_node in self.children.iter_mut() {
            child_node.combine_transform(&self.transform);
        }
    }
}

impl<'a> GsglTFLevelEntity<'a> for GsglTFNodeEntity {
    type GltfArchLevel = (gltf::Node<'a>, &'a mut usize);
    type GltfDataLevel = gltf::Node<'a>;

    fn read_architecture(level: Self::GltfArchLevel) -> Result<GsglTFArchitecture<Self>, GltfError> {

        let mut attr_flag = GsglTFAttrFlags::NONE;
        let mut node_flag = GsglTFNodeUniformFlags::NONE;

        // read transform.
        let transform = Matrix4F::from(level.0.transform().matrix());
        node_flag |= GsglTFNodeUniformFlags::TRANSFORM_MATRIX;

        // first, read the mesh referenced by current node.
        let (local_mesh, draw_order) = if let Some(gltf_mesh) = level.0.mesh() {

            // Record the draw order of current node.
            let draw_order = level.1.clone();
            // Update the draw order index whenever it read a new node recursively.
            (*level.1) += 1;

            let mesh_arch = GsglTFMeshEntity::read_architecture(gltf_mesh)?;
            attr_flag |= mesh_arch.attr_flags;

            (Some(mesh_arch.arch), Some(draw_order))
        } else {
            (None, None)
        };

        // and then, read the child nodes of current node recursively.
        let mut children = vec![];
        for glTF_node in level.0.children() {

            let node_arch = GsglTFNodeEntity::read_architecture((glTF_node, level.1))?;
            attr_flag |= node_arch.attr_flags;
            node_flag |= node_arch.node_flags;

            children.push(Box::new(node_arch.arch));
        }

        let target_arch = GsglTFArchitecture {
            arch: GsglTFNodeEntity { local_mesh, children, draw_order, transform },
            attr_flags: attr_flag,
            node_flags: node_flag,
        };
        Ok(target_arch)
    }

    fn read_data(&mut self, level: Self::GltfDataLevel, source: &IntermediateglTFData, data: &mut GsglTFLoadingData) -> Result<(), GltfError> {

        // read the Mesh data referred by current Node.
        if let Some(ref mut mesh) = self.local_mesh {
            // here unwrap() must not panic.
            mesh.read_data(level.mesh().unwrap(), source, data)?;
        }

        if self.local_mesh.is_some() {
            data.extend_transforms(self);
        }

        // Recursive read the data of child Node.
        for (child_node, child_level) in self.children.iter_mut().zip(level.children()) {
            child_node.read_data(child_level, source, data)?;
        }

        Ok(())
    }
}

impl GsglTFNodeEntity {

    pub(super) fn record_command(&self, recorder: &GsCmdRecorder<Graphics>, mess: &mut GsglTFCmdRecordInfo) {

        if let Some(ref mesh) = self.local_mesh {

            // recalculate the dynamic offset.
            let dyn_offset = (mess.uniform_alignment as vkuint) * (self.draw_order.unwrap() as vkuint);
            mess.binding_sets[mess.gltf_uniform_index].dynamic_offset = Some(dyn_offset);
            // rebind the DescriptorSets.
            recorder.bind_descriptor_sets(0, &mess.binding_sets);

            mesh.record_command(recorder);
        }

        for child_node in self.children.iter() {
            child_node.record_command(recorder, mess);
        }
    }
}
// --------------------------------------------------------------------------------------
