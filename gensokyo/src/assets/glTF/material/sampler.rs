
use crate::assets::glTF::asset::GsglTFAsset;

pub struct GsglTFSamplerData {

}

impl GsglTFAsset<Self> for GsglTFSamplerData {
    const ASSET_NAME: &'static str = "Samplers";

    fn into_data(self) -> GsglTFSamplerData {
        self
    }
}

impl From<&gltf::texture::Sampler<'_>> for GsglTFSamplerData {

    fn from(_raw_sampler: &gltf::texture::Sampler) -> GsglTFSamplerData {

        unimplemented!()
    }
}

impl Default for GsglTFSamplerData {

    fn default() -> GsglTFSamplerData {
        unimplemented!()
    }
}
