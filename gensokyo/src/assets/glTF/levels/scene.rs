
use crate::assets::glTF::data::{ IntermediateglTFData, GsglTFLoadingData };
use crate::assets::glTF::model::{ GsglTFCmdRecordInfo, GsglTFRenderParams };
use crate::assets::glTF::levels::traits::{ GsglTFLevelEntity, GsglTFArchitecture };
use crate::assets::glTF::levels::node::GsglTFNodeEntity;
use crate::assets::glTF::primitive::attributes::GsglTFAttrFlags;
use crate::assets::glTF::primitive::transforms::GsglTFNodeUniformFlags;
use crate::assets::error::GltfError;
use crate::utils::types::Matrix4F;

use gsvk::command::GsCmdRecorder;
use gsvk::utils::phantom::Graphics;

// --------------------------------------------------------------------------------------
/// A wrapper class for scene level in glTF, containing the render parameters read from glTF file.
pub struct GsglTFSceneEntity {

    nodes: Vec<GsglTFNodeEntity>,
}

impl<'a> GsglTFLevelEntity<'a> for GsglTFSceneEntity {
    type GltfArchLevel = gltf::Scene<'a>;
    type GltfDataLevel = gltf::Scene<'a>;

    fn read_architecture(level: Self::GltfArchLevel) -> Result<GsglTFArchitecture<Self>, GltfError> {

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

    fn read_data(&mut self, level: Self::GltfDataLevel, source: &IntermediateglTFData, data: &mut GsglTFLoadingData) -> Result<(), GltfError> {

        for (node_entity, node_level) in self.nodes.iter_mut().zip(level.nodes()) {

            // first, update node's transformation.
            node_entity.combine_transform(&Matrix4F::identity());
            // then read the transform data. The order is important.
            node_entity.read_data(node_level, source, data)?;
        }

        Ok(())
    }
}

impl GsglTFSceneEntity {

    pub(crate) fn record_command(&self, recorder: &GsCmdRecorder<Graphics>, mess: &mut GsglTFCmdRecordInfo, params: &GsglTFRenderParams) {

        for node in self.nodes.iter() {
            node.record_command(recorder, mess, params);
        }
    }
}
// --------------------------------------------------------------------------------------
