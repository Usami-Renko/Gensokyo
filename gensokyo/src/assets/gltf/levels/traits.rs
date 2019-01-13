
use crate::assets::glTF::data::{ IntermediateglTFData, GsglTFLoadingData };
use crate::assets::glTF::primitive::transforms::GsglTFNodeUniformFlags;
use crate::assets::glTF::primitive::attributes::GsglTFAttrFlags;
use crate::assets::glTF::error::GltfError;

use gsvk::command::GsCommandRecorder;

pub(crate) struct GsglTFArchitecture<T> {

    pub arch: T,
    pub attr_flags: GsglTFAttrFlags,
    pub node_flags: GsglTFNodeUniformFlags,
}

pub(crate) trait GsglTFLevelEntity<'a>: Sized {
    type LevelglTFMessage;
    type LevelglTFData;

    /// Load the architecture of glTF, and decide the vertex type to store attributes of glTF primitive.
    fn read_architecture(level: Self::LevelglTFMessage) -> Result<GsglTFArchitecture<Self>, GltfError>;
    /// Read the data of primitive. Including attributes, materials, textures, samplers, etc...
    fn read_data(&mut self, level: Self::LevelglTFData, source: &IntermediateglTFData, data: &mut GsglTFLoadingData) -> Result<(), GltfError>;
    /// Record the draw command to vk::CommandBuffer.
    fn record_command(&self, recorder: &GsCommandRecorder);
}
