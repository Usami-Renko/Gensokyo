
use gltf;

use utility::model::GltfPrimitive;
use utility::model::GltfRawData;
use utility::model::ModelGltfLoadingError;

pub(crate) struct GltfMesh {

    _name: Option<String>,
    // TODO: Remove the pub(crate) decleration.
    pub(crate) primitives: Vec<GltfPrimitive>,
}

impl GltfMesh {

    pub fn from_hierarchy(hierarchy: gltf::Mesh, data: &GltfRawData) -> Result<Self, ModelGltfLoadingError> {

        let name = hierarchy.name()
            .map(|s| s.to_owned());

        let mut primitives = vec![];
        for raw_primitive in hierarchy.primitives().into_iter() {
            let primitive = GltfPrimitive::from_hierarchy(raw_primitive, data)?;
            primitives.push(primitive);
        }

        let mesh = GltfMesh {
            _name: name, primitives,
        };

        Ok(mesh)
    }
}
