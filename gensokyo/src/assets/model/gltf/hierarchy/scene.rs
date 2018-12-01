
use gltf;

use assets::model::GltfHierarchyAbstract;
use assets::model::GltfResources;
use assets::model::GltfNode;
use assets::model::GltfRawData;
use assets::model::ModelGltfLoadingError;

pub(crate) struct GltfScene {

    _name: Option<String>,
    _nodes: Vec<GltfNode>,
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
            _name: name,
            _nodes: nodes,
        };

        Ok(scene)
    }
}
