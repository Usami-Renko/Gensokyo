
use utility::model::GltfResources;
use utility::model::GltfRawData;
use utility::model::ModelGltfLoadingError;

pub(crate) trait GltfHierarchyAbstract<'a> {
    type HierarchyType;

    fn from_hierarchy(hierarchy: Self::HierarchyType, res: &mut GltfResources, data: &GltfRawData) -> Result<Self, ModelGltfLoadingError>
        where Self: Sized;
}

trait GltfResourceAbstract {
    type ResourceType;
}
