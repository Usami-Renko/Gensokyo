
use crate::assets::glTF::levels::{ GsglTFSceneEntity, GsglTFLevelEntity };
use crate::assets::glTF::data::{ GsglTFDataStorage, IntermediateglTFData, GsglTFLoadingData };
use crate::assets::glTF::error::GltfError;
use crate::assets::error::AssetsError;

use std::path::Path;

// ------------------------------------------------------------------------------------
pub struct GsglTFImporter;

impl GsglTFImporter {

    /// Try to load a glTF file(read to memory) with its path, and return its model data if succeed.
    pub fn load(path: impl AsRef<Path>) -> Result<(GsglTFEntity, GsglTFDataStorage), AssetsError> {

        let (doc, data_buffer, data_image) = gltf::import(path)
            .map_err(|e| AssetsError::Gltf(GltfError::Loading(e)))?;

        let intermediate_data = IntermediateglTFData {
            doc, data_buffer, data_image
        };

        // Only support loading the default scene or first scene in gltf file.
        let dst_scene = intermediate_data.doc.default_scene()
            .or(intermediate_data.doc.scenes().next())
            .ok_or(GltfError::ModelContentMissing)?;

        let arch = GsglTFSceneEntity::read_architecture(dst_scene.clone())?;
        let mut loading_data = GsglTFLoadingData::new(arch.attr_flags, arch.node_flags)?;

        let mut dst_entity = GsglTFEntity { scene: arch.arch };
        dst_entity.scene.read_data(dst_scene, &intermediate_data, &mut loading_data)?;

        Ok((dst_entity, loading_data.into_storage()))
    }
}
// ------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------
pub struct GsglTFEntity {

    pub(crate) scene: GsglTFSceneEntity,
}
// ------------------------------------------------------------------------------------
