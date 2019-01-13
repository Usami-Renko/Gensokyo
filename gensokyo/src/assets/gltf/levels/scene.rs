
use crate::assets::glTF::data::{ IntermediateglTFData, GsglTFLoadingData };
use crate::assets::glTF::levels::traits::{ GsglTFLevelEntity, GsglTFArchitecture };
use crate::assets::glTF::levels::node::GsglTFNodeEntity;
use crate::assets::glTF::primitive::attributes::GsglTFAttrFlags;
use crate::assets::glTF::primitive::transforms::GsglTFNodeUniformFlags;
use crate::assets::glTF::error::GltfError;
use crate::utils::types::Matrix4F;

use gsvk::command::GsCommandRecorder;

// --------------------------------------------------------------------------------------
/// A wrapper class for scene level in glTF, containing the render parameters read from glTF file.
pub struct GsglTFSceneEntity {

    nodes: Vec<GsglTFNodeEntity>,
}

impl<'a> GsglTFLevelEntity<'a> for GsglTFSceneEntity {
    type LevelglTFMessage = gltf::Scene<'a>;
    type LevelglTFData    = gltf::Scene<'a>;

    fn read_architecture(level: Self::LevelglTFMessage) -> Result<GsglTFArchitecture<Self>, GltfError> {

        let mut attr_flag = GsglTFAttrFlags::NONE;
        let mut node_flag = GsglTFNodeUniformFlags::NONE;

        let mut nodes = vec![];
        let mut node_draw_order = 0_usize;

        for glTF_node in level.nodes() {
            let node_arch = GsglTFNodeEntity::read_architecture((glTF_node, &mut node_draw_order))?;
            attr_flag |= node_arch.attr_flags;
            node_flag |= node_arch.node_flags;

            nodes.push(node_arch.arch);
        }

        let target_arch = GsglTFArchitecture {
            arch: GsglTFSceneEntity { nodes },
            attr_flags: attr_flag,
            node_flags: node_flag,
        };
        Ok(target_arch)
    }

    fn read_data(&mut self, level: Self::LevelglTFData, source: &IntermediateglTFData, data: &mut GsglTFLoadingData) -> Result<(), GltfError> {

        for (node_entity, node_level) in self.nodes.iter_mut().zip(level.nodes()) {

            // update node's transformation.
            node_entity.combine_transform(&Matrix4F::identity());

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
