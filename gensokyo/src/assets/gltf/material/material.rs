
use crate::utils::types::Vector4F;

use gsvk::types::vkfloat;

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
