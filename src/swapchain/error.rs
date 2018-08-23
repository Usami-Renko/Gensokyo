
use std::fmt;
use std::error::Error;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SwapchainInitError {

    SwapchianCreationError,
    ExtensionLoadError,
    SurfacePropertiesQueryError,
    GraphicsQueueNotAvailable,
    PresentQueueNotAvailable,
    SwapchainImageGetError,
    ImageViewCreationError,
}

impl Error for SwapchainInitError {}

impl fmt::Display for SwapchainInitError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | SwapchainInitError::SwapchianCreationError      => "Failed to create Swapchain Object.",
            | SwapchainInitError::ExtensionLoadError          => "Failed to load Swapchain Extension.",
            | SwapchainInitError::SurfacePropertiesQueryError => "Failed to query surface property.",
            | SwapchainInitError::GraphicsQueueNotAvailable   => "Graphics Queue is not available",
            | SwapchainInitError::PresentQueueNotAvailable    => "Present Queue is not available",
            | SwapchainInitError::SwapchainImageGetError      => "Failed to get swapchain image from swapchain.",
            | SwapchainInitError::ImageViewCreationError      => "Failed to create Swapchain ImageView.",
        };

        write!(f, "{}", description)
    }
}
