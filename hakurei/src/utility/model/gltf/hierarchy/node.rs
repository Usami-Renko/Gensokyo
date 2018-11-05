
use gltf;

use utility::model::GltfHierarchyAbstract;
use utility::model::GltfResources;
use utility::model::GltfMesh;
use utility::model::GltfRawData;
use utility::model::ModelGltfLoadingError;

pub(crate) struct GltfNode {

    name: Option<String>,
    mesh: Option<usize>,
}

impl<'a> GltfHierarchyAbstract<'a> for GltfNode {
    type HierarchyType = gltf::Node<'a>;

    fn from_hierarchy(hierarchy: Self::HierarchyType, res: &mut GltfResources, data: &GltfRawData) -> Result<Self, ModelGltfLoadingError> {

        let name = hierarchy.name()
            .map(|s| s.to_owned());

        let mesh = if let Some(raw_mesh) = hierarchy.mesh() {

            let mesh = GltfMesh::from_hierarchy(raw_mesh, data)?;
            let mesh_index = res.append_mesh(mesh);

            Some(mesh_index)
        } else {
            None
        };

        let node = GltfNode {
            name, mesh,
        };

        Ok(node)
    }
}

impl<'a> GltfNode {

    fn get_mesh(&self, res: &'a GltfResources) -> Option<&'a GltfMesh> {

        let mesh_index = self.mesh?;
        Some(&res.meshes[mesh_index])
    }
}
