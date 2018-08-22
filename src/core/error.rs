
use std::fmt;
use std::error::Error;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum InstanceError {

    EntryCreationError,
    ValidationLayerNotSupportError,
    InstanceCreationError,
    LayerPropertiesEnumerateError,
}

impl Error for InstanceError {}
impl fmt::Display for InstanceError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | InstanceError::EntryCreationError             => "Failed to create Entry Object.",
            | InstanceError::ValidationLayerNotSupportError => "Validation Layer is not support.",
            | InstanceError::InstanceCreationError          => "Failed to create Instance Object.",
            | InstanceError::LayerPropertiesEnumerateError  => "Failed to enumerate Instance Layer Properties.",
        };

        write!(f, "Error: {}", description)
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ValidationError {

    DebugReportCreationError,
    DebugCallbackCreationError,
}

impl Error for ValidationError {}
impl fmt::Display for ValidationError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | ValidationError::DebugReportCreationError   => "Failed to create DebugReport Object.",
            | ValidationError::DebugCallbackCreationError => "Failed to create DebugReport Callback Object.",
        };

        write!(f, "Error: {}", description)
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PhysicalDeviceError {

    NoSuitableDeviceError,
    EnumerateDeviceError,
    GraphicsQueueNotSupportError,
    PresentQueueNotSupportError,
    EnumerateExtensionsError,
}

impl Error for PhysicalDeviceError {}
impl fmt::Display for PhysicalDeviceError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | PhysicalDeviceError::NoSuitableDeviceError        => "No Physical Device suitable for requirements.",
            | PhysicalDeviceError::EnumerateDeviceError         => "Failed to enumerate Physical Devices.",
            | PhysicalDeviceError::GraphicsQueueNotSupportError => "Physical device does not support graphics requirement.",
            | PhysicalDeviceError::PresentQueueNotSupportError  => "Physical device does not support present requirement.",
            | PhysicalDeviceError::EnumerateExtensionsError     => "Failed to enumerate Device Extensions."
        };

        write!(f, "Error: {}", description)
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SurfaceError {

    SurfaceCreationError,
    ExtensionLoadError,
    QueryCapabilitiesError,
    QueryFormatsError,
    QueryPresentModeError,
}

impl Error for SurfaceError {}
impl fmt::Display for SurfaceError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | SurfaceError::SurfaceCreationError   => "Failed to create Surface.",
            | SurfaceError::ExtensionLoadError     => "Failed to load Surface extension.",
            | SurfaceError::QueryCapabilitiesError => "Failed to query surface capabilities.",
            | SurfaceError::QueryFormatsError      => "Failed to query surface formats.",
            | SurfaceError::QueryPresentModeError  => "Failed to query surface present mode.",
        };

        write!(f, "Error: {}", description)
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LogicalDeviceError {

    DeviceCreationError,
}

impl Error for LogicalDeviceError {}
impl fmt::Display for LogicalDeviceError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | LogicalDeviceError::DeviceCreationError => "Failed to create Logical Device.",
        };

        write!(f, "Error: {}", description)
    }
}