
use crate::assets::glTF::data::{ IntermediateglTFData, GsglTFLoadingData };
use crate::assets::glTF::levels::traits::{ GsglTFLevelEntity, GsglTFArchitecture };
use crate::assets::glTF::levels::primitive::GsglTFPrimitiveEntity;
use crate::assets::glTF::primitive::templates::GsglTFAttrFlag;
use crate::assets::glTF::error::GltfError;

use gsvk::command::GsCommandRecorder;

// --------------------------------------------------------------------------------------
/// A wrapper class for mesh level in glTF, containing the render parameters read from glTF file.
pub(super) struct GsglTFMeshEntity {

    primitives: Vec<GsglTFPrimitiveEntity>,
}

impl<'a> GsglTFLevelEntity<'a> for GsglTFMeshEntity {
    type LevelglTFType = gltf::Mesh<'a>;

    fn read_architecture(level: Self::LevelglTFType) -> Result<GsglTFArchitecture<Self>, GltfError> {

        let mut attr_flag = GsglTFAttrFlag::NONE;

        let mut primitives = vec![];
        for glTF_primitive in level.primitives() {

            let primitive_arch = GsglTFPrimitiveEntity::read_architecture(glTF_primitive)?;
            attr_flag |= primitive_arch.flag;

            primitives.push(primitive_arch.arch);
        }

        let arch_target = GsglTFArchitecture {
            arch: GsglTFMeshEntity { primitives },
            flag: attr_flag,
        };
        Ok(arch_target)
    }

    fn read_data(&mut self, level: Self::LevelglTFType, source: &IntermediateglTFData, data: &mut GsglTFLoadingData) -> Result<(), GltfError> {

        for (primitive_entity, primitive_level) in self.primitives.iter_mut().zip(level.primitives()) {
            primitive_entity.read_data(primitive_level, source, data)?;
        }

        Ok(())
    }

    fn record_command(&self, recorder: &GsCommandRecorder) {

        for primitive in self.primitives.iter() {
            primitive.record_command(recorder);
        }
    }
}
// --------------------------------------------------------------------------------------
