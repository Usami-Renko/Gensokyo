
use crate::assets::glTF::data::{ IntermediateglTFData, GsglTFLoadingData };
use crate::assets::glTF::primitive::templates::GsglTFAttrFlag;
use crate::assets::glTF::error::GltfError;

use gsvk::command::GsCommandRecorder;

pub(crate) struct GsglTFArchitecture<T> {

    pub arch: T,
    pub flag: GsglTFAttrFlag,
}

pub(crate) trait GsglTFLevelEntity<'a>: Sized {
    type LevelglTFType;

    /// Load the architecture of glTF, and decide the vertex type to store attributes of glTF primitive.
    fn read_architecture(level: Self::LevelglTFType) -> Result<GsglTFArchitecture<Self>, GltfError>;
    /// Read the data of primitive. Including attributes, materials, textures, samplers, etc...
    fn read_data(&mut self, level: Self::LevelglTFType, source: &IntermediateglTFData, data: &mut GsglTFLoadingData) -> Result<(), GltfError>;
    /// Record the draw command to vk::CommandBuffer.
    fn record_command(&self, recorder: &GsCommandRecorder);
}
