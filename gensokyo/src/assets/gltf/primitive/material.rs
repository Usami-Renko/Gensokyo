
use crate::assets::gltf::material::storage::{ GltfShareResource, GltfShareResourceTmp };

use gsvk::buffer::instance::GsUniformBlock;
use gsvk::memory::transfer::GsBufferDataUpdater;
use gsvk::memory::AllocatorError;

type GltfStorageIndex = usize;

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
            let material = res.material(mat_index);
            let transfer_data = material.to_uniform_data();

            updater.update(to, &[transfer_data])?;
        }

        Ok(())
    }
}
