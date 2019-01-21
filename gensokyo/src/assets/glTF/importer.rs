
use crate::assets::glTF::levels::{ GsglTFSceneEntity, GsglTFLevelEntity };
use crate::assets::glTF::data::{ GsglTFDataStorage, IntermediateglTFData, GsglTFLoadingData };
use crate::assets::glTF::model::GsglTFEntity;
use crate::assets::glTF::asset::GsglTFPhyLimits;
use crate::assets::error::{ GltfError, AssetsError };
use crate::error::GsResult;

use gsvk::core::physical::GsPhysicalDevice;

use std::path::Path;

// ------------------------------------------------------------------------------------
pub struct GsglTFImporter<'a> {
    pub(crate) physical: &'a GsPhysicalDevice,
}

impl<'a> GsglTFImporter<'a> {

    /// Try to load a glTF file(read to memory) with its path, and return its model data if succeed.
    pub fn load(&self, path: impl AsRef<Path>) -> GsResult<(GsglTFEntity, GsglTFDataStorage)> {

        let (doc, data_buffer, data_image) = gltf::import(path)
            .map_err(|e| AssetsError::Gltf(GltfError::Reading(e)))?;
        let intermediate_data = IntermediateglTFData {
            doc, data_buffer, data_image,
            limits: GsglTFPhyLimits::from(self.physical),
        };

        // Only support loading the default scene or first scene in gltf file.
        let dst_scene = intermediate_data.doc.default_scene()
            .or(intermediate_data.doc.scenes().next())
            .ok_or(AssetsError::Gltf(GltfError::loading("There is no model scene in this gltf file.")))?;

        let arch = GsglTFSceneEntity::read_architecture(dst_scene.clone())
            .map_err(|e| AssetsError::Gltf(e))?;
        let mut loading_data = GsglTFLoadingData::new(arch.attr_flags, arch.node_flags)
            .map_err(|e| AssetsError::Gltf(e))?;

        let mut dst_entity = GsglTFEntity::new(arch.arch);
        dst_entity.scene.read_data(dst_scene, &intermediate_data, &mut loading_data)
            .map_err(|e| AssetsError::Gltf(e))?;

        Ok((dst_entity, loading_data.into_storage()))
    }
}
// ------------------------------------------------------------------------------------
