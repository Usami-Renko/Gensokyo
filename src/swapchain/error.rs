
use std::fmt;
use std::error::Error;

use core::error::SurfaceError;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SwapchainError {

    Init(SwapchainInitError),
    Runtime(SwapchainRuntimeError),
}

impl Error for SwapchainError {
    fn cause(&self) -> Option<&Error> {
        match *self {
            | SwapchainError::Init(ref e)    => Some(e),
            | SwapchainError::Runtime(ref e) => Some(e),
        }
    }
}

impl From<SwapchainInitError> for SwapchainError {
    fn from(error: SwapchainInitError) -> Self {
        SwapchainError::Init(error)
    }
}
impl From<SwapchainRuntimeError> for SwapchainError {
    fn from(error: SwapchainRuntimeError) -> Self {
        SwapchainError::Runtime(error)
    }
}

impl fmt::Display for SwapchainError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = if let Some(err) = self.cause() {
            err.to_string()
        } else {
            "Unknown Error".to_owned()
        };

        write!(f, "{}", description)
    }
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SwapchainInitError {

    SwapchianCreationError,
    ExtensionLoadError,
    SurfacePropertiesQuery(SurfaceError),
    GraphicsQueueNotAvailable,
    PresentQueueNotAvailable,
    SwapchainImageGetError,
    ImageViewCreationError,
}

impl Error for SwapchainInitError {}
impl fmt::Display for SwapchainInitError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            | SwapchainInitError::SwapchianCreationError        => write!(f, "Failed to create Swapchain Object."),
            | SwapchainInitError::ExtensionLoadError            => write!(f, "Failed to load Swapchain Extension."),
            | SwapchainInitError::SurfacePropertiesQuery(ref e) => write!(f, "Failed to query surface property. {}", e),
            | SwapchainInitError::GraphicsQueueNotAvailable     => write!(f, "Graphics Queue is not available"),
            | SwapchainInitError::PresentQueueNotAvailable      => write!(f, "Present Queue is not available"),
            | SwapchainInitError::SwapchainImageGetError        => write!(f, "Failed to get swapchain image from swapchain."),
            | SwapchainInitError::ImageViewCreationError        => write!(f, "Failed to create Swapchain ImageView."),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SwapchainRuntimeError {

    SurfaceUnAvailableError,
    ImageNotReadyError,
    AcquireTimeOut,
    SurfaceSubOptimalError,
    SurfaceOutOfDateError,
    DeviceUnAvailableError,
    OutOfHostMemory,
    OutOfDeviceMemory,
    Unkndow,
}

impl Error for SwapchainRuntimeError {}
impl fmt::Display for SwapchainRuntimeError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match self {
            | SwapchainRuntimeError::SurfaceUnAvailableError => "Surface is no longer available.",
            | SwapchainRuntimeError::ImageNotReadyError      => "Timeout is zero and no image was available.",
            | SwapchainRuntimeError::AcquireTimeOut          => "No image became available within the time allowed.",
            | SwapchainRuntimeError::SurfaceSubOptimalError  => "Swapchain does not match the surface properties exactly.",
            | SwapchainRuntimeError::SurfaceOutOfDateError   => "Surface has changed and is not compatible with the swapchain. Please recreate the swapchain.",
            | SwapchainRuntimeError::DeviceUnAvailableError  => "Device is no longer available.",
            | SwapchainRuntimeError::OutOfHostMemory         => "Host memory is running out.",
            | SwapchainRuntimeError::OutOfDeviceMemory       => "Device memory is running out.",
            | SwapchainRuntimeError::Unkndow                 => "Get unknown error when acquiring image.",
        };

        write!(f, "{}", description)
    }
}
