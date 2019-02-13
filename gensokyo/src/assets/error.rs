
use failure_derive::Fail;

use crate::error::GsError;

#[derive(Debug, Fail)]
pub enum AssetsError {
    #[fail(display = "{}", _0)]
    Image(#[cause] image::ImageError),
    #[fail(display = "{}", _0)]
    Gltf(#[cause] GltfError),
}

#[derive(Debug, Fail)]
pub enum GltfError {
    #[fail(display = "glTF: {}", _0)]
    Reading(#[cause] gltf::Error),
    #[fail(display = "{}", description)]
    Loading { description: &'static str },
    #[fail(display = "Failed to convert glTF content to bytes: {}", _0)]
    Convert(#[cause] bincode::Error),
}

impl GltfError {

    pub fn loading(description: &'static str) -> GltfError {
        GltfError::Loading { description }
    }
}

impl From<AssetsError> for GsError {

    fn from(error: AssetsError) -> GsError {
        GsError::assets(error)
    }
}
