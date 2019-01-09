
use crate::assets::gltf::material::{ GsGltfMaterial, GsGltfTexture, GsGltfSampler };

use std::collections::HashMap;

type GltfReferenceIndex = usize;
type GltfStorageIndex   = usize;

// ------------------------------------------------------------------------------------
#[derive(Default)]
pub(crate) struct GltfShareResourceTmp {

    materials: Vec<GsGltfMaterial>,
    textures : Vec<GsGltfTexture>,
    samplers : Vec<GsGltfSampler>,

    material_indices: HashMap<GltfReferenceIndex, GltfStorageIndex>,
    texture_indices : HashMap<GltfReferenceIndex, GltfStorageIndex>,
    sampler_indices : HashMap<GltfReferenceIndex, GltfStorageIndex>,
}

impl GltfShareResourceTmp {

    pub fn load_material(&mut self, primitive: &gltf::Primitive) -> Option<GltfStorageIndex> {

        let raw_material = primitive.material();
        let gltf_index = raw_material.index()?;

        self.material_indices.get(&gltf_index)
            .cloned()
            .or_else(|| {
                let dst_material = GsGltfMaterial::new(&raw_material);

                let res_index = self.materials.len();
                self.material_indices.insert(gltf_index, res_index);
                self.materials.push(dst_material);
                Some(res_index)
            })
    }

    pub fn into_resource(self) -> GltfShareResource {
        GltfShareResource {
            materials: self.materials,
            textures : self.textures,
            samplers : self.samplers,
        }
    }
}

#[derive(Default)]
pub(crate) struct GltfShareResource {

    materials: Vec<GsGltfMaterial>,
    textures : Vec<GsGltfTexture>,
    samplers : Vec<GsGltfSampler>,
}

impl GltfShareResource {

    pub fn material(&self, at: GltfStorageIndex) -> &GsGltfMaterial {
        &self.materials[at]
    }

    pub fn texture(&self, at: GltfStorageIndex) -> &GsGltfTexture {
        &self.textures[at]
    }

    pub fn sampler(&self, at: GltfStorageIndex) -> &GsGltfSampler {
        &self.samplers[at]
    }
}
// ------------------------------------------------------------------------------------
