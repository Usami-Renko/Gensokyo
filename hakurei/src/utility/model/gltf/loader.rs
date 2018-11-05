
use utility::model::GltfEntity;
use utility::model::ModelLoadingErr;

use std::path::Path;

// An gltf model loader.
pub struct ModelGltfLoader {}

impl ModelGltfLoader {

    pub fn new() -> ModelGltfLoader {
        ModelGltfLoader {}
    }

    pub fn load_model(&self, path: &impl AsRef<Path>) -> Result<GltfEntity, ModelLoadingErr> {

        GltfEntity::load(path)
    }
}
