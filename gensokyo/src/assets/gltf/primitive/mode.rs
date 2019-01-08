
pub(super) struct GltfPrimitiveMode(pub gltf::mesh::Mode);

impl GltfPrimitiveMode {

    pub fn read(primitive: &gltf::Primitive) -> GltfPrimitiveMode {

        GltfPrimitiveMode(primitive.mode())
    }
}
