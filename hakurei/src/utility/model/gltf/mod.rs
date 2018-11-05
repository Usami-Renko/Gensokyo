
pub use self::entity::GltfEntity;
pub use self::loader::ModelGltfLoader;

pub(crate) use self::traits::GltfHierarchyAbstract;
pub(crate) use self::hierarchy::{
    GltfResources, GltfRawData, // resource
    GltfScene,     // scene
    GltfNode,      // node
    GltfMesh,      // mesh
    GltfPrimitive, // primitive
};

mod loader;
mod entity;
mod hierarchy;
mod traits;
