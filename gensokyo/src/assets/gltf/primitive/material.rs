
use crate::assets::gltf::storage::GltfShareResource;
use crate::assets::gltf::material::GltfShareResourceTmp;

use gsvk::buffer::instance::GsUniformBlock;
use gsvk::memory::transfer::GsBufferDataUpdater;
use gsvk::memory::AllocatorError;

type GltfStorageIndex = usize;

#[derive(Debug, Clone)]
pub(super) struct GltfPrimitiveMaterial {

    index: Option<GltfStorageIndex>,
}

impl GltfPrimitiveMaterial {

    pub fn read(primitive: &gltf::Primitive, res: &mut GltfShareResourceTmp) -> GltfPrimitiveMaterial {

        let index = res.load_material(primitive);
        GltfPrimitiveMaterial { index }
    }

    pub fn update_uniform(&self, to: &GsUniformBlock, updater: &mut GsBufferDataUpdater, res: &GltfShareResource) -> Result<(), AllocatorError> {

        if let Some(mat_index) = self.index {
            let material = &res.materials[mat_index];
            let transfer_data = material.to_uniform_data();

            updater.update(to, &[transfer_data])?;
        }

        Ok(())
    }
}
