
use crate::assets::model::GltfResources;
use crate::assets::model::GltfRawData;
use crate::assets::model::ModelGltfLoadingError;

pub(crate) trait GltfHierarchyAbstract<'a> {
    type HierarchyType;

    fn from_hierarchy(hierarchy: Self::HierarchyType, res: &mut GltfResources, data: &GltfRawData) -> Result<Self, ModelGltfLoadingError>
        where Self: Sized;
}

trait GltfResourceAbstract {
    type ResourceType;
}
