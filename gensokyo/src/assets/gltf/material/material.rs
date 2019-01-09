
use crate::assets::gltf::material::{ GsGltfTexture, GsGltfSampler };
use crate::assets::gltf::storage::GltfShareResource;
use crate::utils::types::Vector4F;

use gsvk::types::vkfloat;

use std::collections::HashMap;

type GltfReferenceIndex = usize;
type GltfStorageIndex   = usize;


pub struct GsGltfMaterial {

    pbr: PbrMetallicRoughness,
    emissive_factor: [vkfloat; 3],
}

struct PbrMetallicRoughness {

    base_color_factor: [vkfloat; 4],
    metallic_factor: vkfloat,
}

impl GsGltfMaterial {

    pub fn new(raw_material: &gltf::Material) -> GsGltfMaterial {

        let raw_pbr = raw_material.pbr_metallic_roughness();

        GsGltfMaterial {
            pbr: PbrMetallicRoughness {
                base_color_factor: raw_pbr.base_color_factor(),
                metallic_factor  : raw_pbr.metallic_factor(),
            },
            emissive_factor: raw_material.emissive_factor(),
        }
    }

    pub fn to_uniform_data(&self) -> GltfPbrUniform {

        GltfPbrUniform {
            base_color_factor: Vector4F::from(self.pbr.base_color_factor),
            metallic_factor: self.pbr.metallic_factor,
        }
    }
}

// TODO: Test aligment.
#[derive(Debug, Clone, Copy)]
pub struct GltfPbrUniform {
    base_color_factor: Vector4F,
    metallic_factor  : vkfloat,
}


// ------------------------------------------------------------------------------------
#[derive(Default)]
pub struct GltfShareResourceTmp {

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
// ------------------------------------------------------------------------------------
