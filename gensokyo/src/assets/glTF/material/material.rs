
use crate::assets::glTF::asset::GsglTFPhyLimits;
use crate::assets::glTF::error::GltfError;

use gsvk::types::vkfloat;

// ------------------------------------------------------------------------------------
pub struct GsglTFMaterialData {

    pbr: PbrMetallicRoughness,
    emissive_factor: [vkfloat; 3],
}

struct PbrMetallicRoughness {

    base_color_factor: [vkfloat; 4],
    metallic_factor  : vkfloat,
}

impl From<&'_ gltf::Material<'_>> for GsglTFMaterialData {

    fn from(raw_material: &gltf::Material) -> GsglTFMaterialData {

        let raw_pbr = raw_material.pbr_metallic_roughness();

        GsglTFMaterialData {
            pbr: PbrMetallicRoughness {
                base_color_factor: raw_pbr.base_color_factor(),
                metallic_factor  : raw_pbr.metallic_factor(),
            },
            emissive_factor: raw_material.emissive_factor(),
        }
    }
}

impl GsglTFMaterialData {

    pub(crate) fn into_data(self, limits: &GsglTFPhyLimits) -> Result<Vec<u8>, GltfError> {

        let data = MaterialConstants {
            base_color_factor: self.pbr.base_color_factor,
            metallic_factor  : self.pbr.metallic_factor,
            emissive_factor  : self.emissive_factor,
        };

        let bytes_data = bincode::serialize(&data)
            .map_err(|e| GltfError::Convert(e))?;

        if bytes_data.len() > limits.max_push_constant_size as _ {
            Err(GltfError::MaterialReachMaxSize)
        } else {
            Ok(bytes_data)
        }
    }
}
// ------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------
#[derive(Debug, Clone, Copy, Serialize)]
pub struct MaterialConstants {
    base_color_factor: [vkfloat; 4],
    emissive_factor  : [vkfloat; 3],
    metallic_factor  : vkfloat,
}
// ------------------------------------------------------------------------------------
