
use crate::assets::glTF::data::{ IntermediateglTFData, GsglTFLoadingData };
use crate::assets::glTF::levels::traits::{ GsglTFLevelEntity, GsglTFArchitecture };
use crate::assets::glTF::levels::mesh::GsglTFMeshEntity;
use crate::assets::glTF::primitive::templates::GsglTFAttrFlag;
use crate::assets::glTF::error::GltfError;
use crate::utils::types::Matrix4F;

use gsvk::command::GsCommandRecorder;

// --------------------------------------------------------------------------------------
/// A wrapper class for node level in glTF, containing the render parameters read from glTF file.
pub(super) struct GsglTFNodeEntity {

    local_mesh: Option<GsglTFMeshEntity>,
    children: Vec<Box<GsglTFNodeEntity>>,

    transform: Matrix4F,
}

impl GsglTFNodeEntity {

    /// Apply parent node's transformation to current node level.
    pub fn combine_transform(&mut self, parent_transform: &Matrix4F) {
        self.transform = self.transform * parent_transform;

        for child_node in self.children.iter_mut() {
            child_node.combine_transform(&self.transform);
        }
    }
}

impl<'a> GsglTFLevelEntity<'a> for GsglTFNodeEntity {
    type LevelglTFType = gltf::Node<'a>;

    fn read_architecture(level: Self::LevelglTFType) -> Result<GsglTFArchitecture<Self>, GltfError> {

        let mut attr_flag = GsglTFAttrFlag::NONE;
        let transform = Matrix4F::from(level.transform().matrix());

        let mut children = vec![];
        for glTF_node in level.children() {

            let node_arch = GsglTFNodeEntity::read_architecture(glTF_node)?;
            attr_flag |= node_arch.flag;

            children.push(Box::new(node_arch.arch));
        }

        let local_mesh = if let Some(gltf_mesh) = level.mesh() {

            let mesh_arch = GsglTFMeshEntity::read_architecture(gltf_mesh)?;
            attr_flag |= mesh_arch.flag;

            Some(mesh_arch.arch)
        } else {
            None
        };

        let target_arch = GsglTFArchitecture {
            arch: GsglTFNodeEntity { local_mesh, children, transform },
            flag: attr_flag,
        };
        Ok(target_arch)
    }

    fn read_data(&mut self, level: Self::LevelglTFType, source: &IntermediateglTFData, data: &mut GsglTFLoadingData) -> Result<(), GltfError> {

        // read the Mesh data referred by current Node.
        if let Some(ref mut mesh) = self.local_mesh {
            // here unwrap() must not panic.
            mesh.read_data(level.mesh().unwrap(), source, data)?;
        }

        // Recursive read the data of child Node.
        for (child_node, child_level) in self.children.iter_mut().zip(level.children()) {
            child_node.read_data(child_level, source, data)?;
        }

        Ok(())
    }

    fn record_command(&self, recorder: &GsCommandRecorder) {

        if let Some(ref mesh) = self.local_mesh {
            mesh.record_command(recorder);
        }

        for child_node in self.children.iter() {
            child_node.record_command(recorder);
        }
    }
}
// --------------------------------------------------------------------------------------
