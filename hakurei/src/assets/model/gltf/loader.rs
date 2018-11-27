
use assets::model::GltfEntity;
use assets::model::ModelLoadingErr;

use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;

use std::path::Path;

// An gltf model loader.
pub struct ModelGltfLoader {}

impl ModelGltfLoader {

    pub fn new() -> ModelGltfLoader {
        ModelGltfLoader {}
    }

    pub fn load_model<M: BufferMemoryTypeAbs + Copy>(&self, path: impl AsRef<Path>, typ: M) -> Result<GltfEntity<M>, ModelLoadingErr> {

        GltfEntity::load(path, typ)
    }
}
