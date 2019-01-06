
use crate::assets::gltf::primitive::traits::GltfPrimitiveProperty;
use crate::assets::gltf::primitive::attrpatterns::{ GPAttribute, GPAFlag, GPAP, GPAPN, GPAPNTe0, GPAUltimate };
use crate::assets::gltf::error::GltfError;
use crate::utils::types::Matrix4F;

use gsvk::buffer::allocator::{ GsBufferAllocator, BufferBlockIndex };
use gsvk::buffer::instance::{ VertexBlockInfo, GsVertexBlock };
use gsvk::buffer::allocator::types::BufferMemoryTypeAbs;
use gsvk::memory::transfer::BufferDataUploader;
use gsvk::memory::AllocatorError;

pub(super) struct GltfPrimitiveAttributes {

    data: Box<dyn GPAttribute>,
}

impl GltfPrimitiveProperty for GltfPrimitiveAttributes {
    const PROPERTY_NAME: &'static str = "attributes";

    fn read<'a, 's, F>(primitive: &gltf::Primitive, reader: &gltf::mesh::Reader<'a, 's, F>) -> Result<Self, GltfError>
        where F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]> {

        let mut require_flags = GPAFlag::NONE;
        for (attribute, _accessor) in primitive.attributes() {
            match attribute {
                | gltf::Semantic::Positions    => require_flags |= GPAFlag::POSITION,
                | gltf::Semantic::Normals      => require_flags |= GPAFlag::NORMAL,
                | gltf::Semantic::Tangents     => require_flags |= GPAFlag::TANGENT,
                | gltf::Semantic::Colors(0)    => require_flags |= GPAFlag::COLOR_0,
                | gltf::Semantic::TexCoords(0) => require_flags |= GPAFlag::TEXCOORD_0,
                | gltf::Semantic::TexCoords(1) => require_flags |= GPAFlag::TEXCOORD_1,
                | gltf::Semantic::Joints(0)    => require_flags |= GPAFlag::JOINTS_0,
                | gltf::Semantic::Weights(0)   => require_flags |= GPAFlag::WEIGHTS_0,
                | _ => return Err(GltfError::UnknownAttribute)
            }
        }

        let data = match require_flags {
            | GPAFlag::GPAP        => Box::new(GPAP::load(reader)?)        as Box<dyn GPAttribute>,
            | GPAFlag::GPAPN       => Box::new(GPAPN::load(reader)?)       as Box<dyn GPAttribute>,
            | GPAFlag::GPAPNTE0    => Box::new(GPAPNTe0::load(reader)?)    as Box<dyn GPAttribute>,
            | GPAFlag::GPAULTIMATE => Box::new(GPAUltimate::load(reader)?) as Box<dyn GPAttribute>,
            | _ => return Err(GltfError::UnsupportAttributes)
        };

        let target = GltfPrimitiveAttributes { data };
        Ok(target)
    }
}

impl GltfPrimitiveAttributes {

    pub fn append_allocation<M>(&self, allocator: &mut GsBufferAllocator<M>) -> Result<BufferBlockIndex, AllocatorError>
        where M: BufferMemoryTypeAbs {

        let vertex_info = VertexBlockInfo::new(self.data.attribute_size());
        allocator.append_buffer(vertex_info)
    }

    #[inline]
    pub fn upload(&self, to: &GsVertexBlock, by: &mut BufferDataUploader) -> Result<(), AllocatorError> {
        self.data.upload(to, by)
    }

    #[inline]
    pub fn apply_transform(&mut self, transform: &Matrix4F) {
        self.data.update_transform(transform);
    }
}
