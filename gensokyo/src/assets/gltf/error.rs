
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum GltfError {

    Loading(gltf::Error),
    ModelContentMissing,
}

impl Error for GltfError {}
impl fmt::Display for GltfError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | GltfError::Loading(e) => e.to_string(),
            | GltfError::ModelContentMissing => String::from("There is no model scene in this gltf file."),
        };

        write!(f, "{}", description)
    }
}
