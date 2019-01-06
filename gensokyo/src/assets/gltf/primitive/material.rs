
use crate::assets::gltf::primitive::traits::GltfPrimitiveProperty;
use crate::assets::gltf::error::GltfError;

use gsvk::types::vkfloat;

pub(crate) struct GltfPrimitiveMaterial {

    pbr: Option<PbrMetallicRoughness>,
    emissive_factor: [vkfloat; 3],
}

struct PbrMetallicRoughness {

    base_color_factor: [vkfloat; 4],
    metallic_factor: vkfloat,
}

impl GltfPrimitiveProperty for GltfPrimitiveMaterial {
    const PROPERTY_NAME: &'static str = "material";

    fn read<'a, 's, F>(primitive: &gltf::Primitive, _reader: &gltf::mesh::Reader<'a, 's, F>) -> Result<Self, GltfError>
        where F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]> {

        let raw_material = primitive.material();
        let material = if raw_material.index().is_some() {

            let raw_pbr = raw_material.pbr_metallic_roughness();

            GltfPrimitiveMaterial {
                pbr: Some(
                    PbrMetallicRoughness {
                        base_color_factor: raw_pbr.base_color_factor(),
                        metallic_factor  : raw_pbr.metallic_factor(),
                    }
                ),
                emissive_factor: raw_material.emissive_factor(),
            }
        } else {

            GltfPrimitiveMaterial {
                pbr: None,
                emissive_factor: [0.0; 3],
            }
        };

        Ok(material)
    }
}

impl GltfPrimitiveMaterial {

    pub fn is_contain_material(&self) -> bool {
        self.pbr.is_some()
    }
}
