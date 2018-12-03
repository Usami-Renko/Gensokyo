
use std::fmt;
use std::error::Error;

use tobj::LoadError;
use gltf::Error as GltfError;

#[derive(Debug)]
pub enum ModelLoadingError {

    Obj(ModelObjLoadingError),
    Gltf(ModelGltfLoadingError),
}

impl Error for ModelLoadingError {}
impl fmt::Display for ModelLoadingError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | ModelLoadingError::Obj(e)  => e.to_string(),
            | ModelLoadingError::Gltf(e) => e.to_string(),
        };

        write!(f, "{}", description)
    }
}

#[derive(Debug)]
pub enum ModelObjLoadingError {

    Loading(LoadError),
}

impl Error for ModelObjLoadingError {}
impl fmt::Display for ModelObjLoadingError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | ModelObjLoadingError::Loading(e) => e.to_string(),
        };

        write!(f, "{}", description)
    }
}

#[derive(Debug)]
pub enum ModelGltfLoadingError {

    Gltf(GltfError),
    AttriMissing(GltfAttributeMissing),
    AttributeElementCountNotMatch,
}

impl_from_err!(AttriMissing(GltfAttributeMissing) -> ModelGltfLoadingError);

impl Error for ModelGltfLoadingError {}
impl fmt::Display for ModelGltfLoadingError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | ModelGltfLoadingError::Gltf(e)         => e.to_string(),
            | ModelGltfLoadingError::AttriMissing(e) => e.to_string(),
            | ModelGltfLoadingError::AttributeElementCountNotMatch =>
                String::from("The element count between attributes is not match."),
        };

        write!(f, "{}", description)
    }
}

#[derive(Debug)]
pub enum GltfAttributeMissing {
    Position,
    Color,
    TexCoord,
    Index,
}

impl Error for GltfAttributeMissing {}
impl fmt::Display for GltfAttributeMissing {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | GltfAttributeMissing::Position => "Position attribute is missing in Gltf file.",
            | GltfAttributeMissing::Color    => "Color attribute is missing in Gltf file.",
            | GltfAttributeMissing::TexCoord => "Texture Coordinate attribute is missing in Gltf file.",
            | GltfAttributeMissing::Index    => "Index attribute is missing in Gltf file.",
        };

        write!(f, "{}", description)
    }
}
