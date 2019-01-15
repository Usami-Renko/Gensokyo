
use crate::assets::glTF::asset::GsglTFAsset;

pub struct GsglTFTextureData {

}

impl GsglTFAsset<Self> for GsglTFTextureData {
    const ASSET_NAME: &'static str = "Textures";

    fn into_data(self) -> GsglTFTextureData {
        self
    }
}

impl From<&gltf::Texture<'_>> for GsglTFTextureData {

    fn from(_raw_texture: &gltf::Texture) -> GsglTFTextureData {

        unimplemented!()
    }
}

impl Default for GsglTFTextureData {

    fn default() -> GsglTFTextureData {
        unimplemented!()
    }
}
