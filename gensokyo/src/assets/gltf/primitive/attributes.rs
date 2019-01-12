
use crate::assets::glTF::primitive::templates::{ GPAttribute, GsglTFAttrFlag };
use crate::assets::glTF::primitive::templates::{ GPAP, GPAPN, GPAPNTe0, GPAUltimate };
use crate::assets::glTF::error::GltfError;

use gsvk::buffer::instance::VertexBlockInfo;
use gsvk::types::vkbytes;

pub(crate) struct GsglTFAttributesData {

    vertex_size: vkbytes,
    pub content: Box<dyn GPAttribute>,
}

impl GsglTFAttributesData {

    pub fn new(flag: GsglTFAttrFlag) -> Result<GsglTFAttributesData, GltfError> {

        let content = match flag {
            | GsglTFAttrFlag::GPAP        => Box::new(GPAP::default())        as Box<dyn GPAttribute>,
            | GsglTFAttrFlag::GPAPN       => Box::new(GPAPN::default())       as Box<dyn GPAttribute>,
            | GsglTFAttrFlag::GPAPNTE0    => Box::new(GPAPNTe0::default())    as Box<dyn GPAttribute>,
            | GsglTFAttrFlag::GPAULTIMATE => Box::new(GPAUltimate::default()) as Box<dyn GPAttribute>,
            | _ => return Err(GltfError::UnsupportAttributes)
        };

        let attributes = GsglTFAttributesData {
            vertex_size: flag.vertex_size()
                .ok_or(GltfError::UnsupportAttributes)?,
            content,
        };
        Ok(attributes)
    }

    pub fn data_size(&self) -> vkbytes {
        (self.content.data_length() as vkbytes) * self.vertex_size
    }

    pub fn vertex_info(&self) -> VertexBlockInfo {

        VertexBlockInfo::new(self.data_size())
    }
}
