
use crate::assets::gltf::error::GltfError;

pub(crate) trait GltfPrimitiveProperty where Self: Sized {
    const PROPERTY_NAME: &'static str;

    fn read<'a, 's, F>(primitive: &gltf::Primitive, reader: &gltf::mesh::Reader<'a, 's, F>) -> Result<Self, GltfError>
        where F: Clone + Fn(gltf::Buffer<'a>) -> Option<&'s [u8]>;
}
