
// pub use self::obj::{ ObjDataEntity, ModelObjLoader };
pub use self::gltf::{ GltfEntity, ModelGltfLoader };
pub use self::error::ModelLoadingError;

pub(crate) use self::error::{
    // ModelObjLoadingError,
    ModelGltfLoadingError, GltfAttributeMissing,
};
pub(crate) use self::gltf::{
    GltfHierarchyAbstract,
    GltfResources, GltfScene, GltfNode, GltfMesh, GltfPrimitive, // hierarchy
    GltfRawData,
};

mod gltf;
// mod obj;
mod error;
