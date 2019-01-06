
use gsvk::types::vkfloat;

pub(crate) struct GltfPropertyMaterial {

    pbr: Option<PbrMetallicRoughness>,
    emissive_factor: [vkfloat; 3],
}

struct PbrMetallicRoughness {

    base_color_factor: [vkfloat; 4],
    metallic_factor: vkfloat,
}

impl GltfPropertyMaterial {

    pub fn load(from: &gltf::Primitive) -> GltfPropertyMaterial {

        if from.indices().is_some() { // material property is defined in glTF.

            let raw_material = from.material();
            let raw_pbr = raw_material.pbr_metallic_roughness();

            GltfPropertyMaterial {
                pbr: Some(
                    PbrMetallicRoughness {
                        base_color_factor: raw_pbr.base_color_factor(),
                        metallic_factor  : raw_pbr.metallic_factor(),
                    }
                ),
                emissive_factor: raw_material.emissive_factor(),
            }
        } else { // material property is not defined in glTF.

            GltfPropertyMaterial {
                pbr: None,
                emissive_factor: [0.0; 3],
            }
        }
    }

    pub fn is_contain_material(&self) -> bool {
        self.pbr.is_some()
    }
}
