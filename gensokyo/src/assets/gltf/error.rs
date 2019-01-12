
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum GltfError {

    Loading(gltf::Error),
    ModelContentMissing,
    UnsupportAttributes,
    UnknownAttribute,
    UnsupportRenderMode,
    VerificationError,
}

impl Error for GltfError {}
impl fmt::Display for GltfError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | GltfError::Loading(e) => e.to_string(),
            | GltfError::ModelContentMissing => String::from("There is no model scene in this gltf file."),
            | GltfError::UnsupportAttributes => String::from("Unsupport glTF primitive attributes combination."),
            | GltfError::UnknownAttribute    => String::from("Unknown property was found when reading glTF."),
            | GltfError::UnsupportRenderMode => String::from("Unsupport glTF primitive render mode."),
            | GltfError::VerificationError   => String::from("Failed to verify the content of glTF."),
        };

        write!(f, "{}", description)
    }
}
