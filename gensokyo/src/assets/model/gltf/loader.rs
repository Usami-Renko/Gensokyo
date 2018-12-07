
use crate::assets::model::GltfEntity;
use crate::assets::model::ModelLoadingError;

use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;

use std::path::Path;

// An gltf model loader.
pub struct ModelGltfLoader {}

impl ModelGltfLoader {

    pub fn new() -> ModelGltfLoader {
        ModelGltfLoader {}
    }

    pub fn load_model<M: BufferMemoryTypeAbs + Copy>(&self, path: impl AsRef<Path>, typ: M) -> Result<GltfEntity<M>, ModelLoadingError> {

        GltfEntity::load(path, typ)
    }
}
