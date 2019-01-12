
use crate::assets::glTF::data::{ IntermediateglTFData, GsglTFLoadingData };
use crate::assets::glTF::levels::traits::{ GsglTFLevelEntity, GsglTFArchitecture };
use crate::assets::glTF::levels::node::GsglTFNodeEntity;
use crate::assets::glTF::primitive::templates::GsglTFAttrFlag;
use crate::assets::glTF::error::GltfError;
use crate::utils::types::Matrix4F;

use gsvk::command::GsCommandRecorder;

// --------------------------------------------------------------------------------------
/// A wrapper class for scene level in glTF, containing the render parameters read from glTF file.
pub struct GsglTFSceneEntity {

    nodes: Vec<GsglTFNodeEntity>,
}

impl GsglTFSceneEntity {

    pub fn update_transforms(&mut self) {

        for node in self.nodes.iter_mut() {
            node.combine_transform(&Matrix4F::identity())
        }
    }
}

impl<'a> GsglTFLevelEntity<'a> for GsglTFSceneEntity {
    type LevelglTFType = gltf::Scene<'a>;

    fn read_architecture(level: Self::LevelglTFType) -> Result<GsglTFArchitecture<Self>, GltfError> {

        let mut attr_flag = GsglTFAttrFlag::NONE;
        let mut nodes = vec![];

        for glTF_node in level.nodes() {
            let node_arch = GsglTFNodeEntity::read_architecture(glTF_node)?;
            attr_flag |= node_arch.flag;

            nodes.push(node_arch.arch);
        }

        let target_arch = GsglTFArchitecture {
            arch: GsglTFSceneEntity { nodes },
            flag: attr_flag,
        };
        Ok(target_arch)
    }

    fn read_data(&mut self, level: Self::LevelglTFType, source: &IntermediateglTFData, data: &mut GsglTFLoadingData) -> Result<(), GltfError> {

        for (node_entity, node_level) in self.nodes.iter_mut().zip(level.nodes()) {
            node_entity.read_data(node_level, source, data)?;
        }

        Ok(())
    }

    fn record_command(&self, recorder: &GsCommandRecorder) {

        for node in self.nodes.iter() {
            node.record_command(recorder);
        }
    }
}
// --------------------------------------------------------------------------------------
