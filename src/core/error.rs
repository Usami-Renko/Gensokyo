
use std::fmt;

#[derive(Clone, Debug)]
pub enum InstanceError {

    EntryCreationError,
    ValidationLayerNotSupportError,
    InstanceCreationError,
}

impl fmt::Display for InstanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | InstanceError::EntryCreationError             => "Failed to create Entry Object.",
            | InstanceError::ValidationLayerNotSupportError => "Validation Layer is not support.",
            | InstanceError::InstanceCreationError          => "Failed to create Instance Object.",
        };

        write!(f, "Error: {}", description)
    }
}
