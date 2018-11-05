
use gltf;

use utility::model::GltfHierarchyAbstract;
use utility::model::GltfResources;
use utility::model::GltfNode;
use utility::model::GltfRawData;
use utility::model::ModelGltfLoadingError;

pub(crate) struct GltfScene {

    name : Option<String>,
    nodes: Vec<GltfNode>,
}

impl<'a> GltfHierarchyAbstract<'a> for GltfScene {
    type HierarchyType = gltf::Scene<'a>;

    fn from_hierarchy(hierarchy: Self::HierarchyType, res: &mut GltfResources, data: &GltfRawData) -> Result<Self, ModelGltfLoadingError> {

        let name = hierarchy.name()
            .map(|s| s.to_owned());

        let mut nodes = vec![];
        for raw_node in hierarchy.nodes().into_iter() {
            let node = GltfNode::from_hierarchy(raw_node, res, data)?;
            nodes.push(node);
        }

        let scene = GltfScene {
            name, nodes,
        };

        Ok(scene)
    }
}
