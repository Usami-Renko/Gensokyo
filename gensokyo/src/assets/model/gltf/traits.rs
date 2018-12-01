
use assets::model::GltfResources;
use assets::model::GltfRawData;
use assets::model::ModelGltfLoadingError;

pub(crate) trait GltfHierarchyAbstract<'a> {
    type HierarchyType;

    fn from_hierarchy(hierarchy: Self::HierarchyType, res: &mut GltfResources, data: &GltfRawData) -> Result<Self, ModelGltfLoadingError>
        where Self: Sized;
}

trait GltfResourceAbstract {
    type ResourceType;
}
