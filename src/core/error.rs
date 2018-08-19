
use std::fmt;

#[derive(Clone, Debug)]
pub enum InstanceError {

    EntryCreationError,
    ValidationLayerNotSupportError,
    InstanceCreationError,
    InstanceLayerPropertiesEnumerateError,
}

impl fmt::Display for InstanceError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | InstanceError::EntryCreationError                    => "Failed to create Entry Object.",
            | InstanceError::ValidationLayerNotSupportError        => "Validation Layer is not support.",
            | InstanceError::InstanceCreationError                 => "Failed to create Instance Object.",
            | InstanceError::InstanceLayerPropertiesEnumerateError => "Failed to enumerate Instance Layer Properties.",
        };

        write!(f, "Error: {}", description)
    }
}


#[derive(Clone, Debug)]
pub enum ValidationError {

    DebugReportCreationError,
    DebugCallbackCreationError,
}

impl fmt::Display for ValidationError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | ValidationError::DebugReportCreationError   => "Failed to create DebugReport Object.",
            | ValidationError::DebugCallbackCreationError => "Failed to create DebugReport Callback Object.",
        };

        write!(f, "Error: {}", description)
    }
}
