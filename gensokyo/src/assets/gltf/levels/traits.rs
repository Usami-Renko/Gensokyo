
use crate::assets::glTF::data::{ IntermediateglTFData, GsglTFLoadingData };
use crate::assets::glTF::primitive::transforms::GsglTFNodeUniformFlags;
use crate::assets::glTF::primitive::attributes::GsglTFAttrFlags;
use crate::assets::glTF::error::GltfError;

pub(crate) struct GsglTFArchitecture<T> {

    pub arch: T,
    pub attr_flags: GsglTFAttrFlags,
    pub node_flags: GsglTFNodeUniformFlags,
}

pub(crate) trait GsglTFLevelEntity<'a>: Sized {
    type GltfArchLevel;
    type GltfDataLevel;

    /// Load the architecture of glTF, and decide the vertex type to store attributes of glTF primitive.
    fn read_architecture(level: Self::GltfArchLevel) -> Result<GsglTFArchitecture<Self>, GltfError>;
    /// Read the data of primitive. Including attributes, materials, textures, samplers, etc...
    fn read_data(&mut self, level: Self::GltfDataLevel, source: &IntermediateglTFData, data: &mut GsglTFLoadingData) -> Result<(), GltfError>;
}
