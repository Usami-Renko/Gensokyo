
use std::fmt;
use std::error::Error;

use resources::error::FramebufferError;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum SwapchainInitError {

    SwapchianCreationError,
    ExtensionLoadError,
    SurfacePropertiesQueryError,
    GraphicsQueueNotAvailable,
    PresentQueueNotAvailable,
    SwapchainImageGetError,
    ImageViewCreationError,
    Framebuffer(FramebufferError),
}

impl Error for SwapchainInitError {}

impl fmt::Display for SwapchainInitError {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            | SwapchainInitError::SwapchianCreationError      => write!(f, "Failed to create Swapchain Object."),
            | SwapchainInitError::ExtensionLoadError          => write!(f, "Failed to load Swapchain Extension."),
            | SwapchainInitError::SurfacePropertiesQueryError => write!(f, "Failed to query surface property."),
            | SwapchainInitError::GraphicsQueueNotAvailable   => write!(f, "Graphics Queue is not available"),
            | SwapchainInitError::PresentQueueNotAvailable    => write!(f, "Present Queue is not available"),
            | SwapchainInitError::SwapchainImageGetError      => write!(f, "Failed to get swapchain image from swapchain."),
            | SwapchainInitError::ImageViewCreationError      => write!(f, "Failed to create Swapchain ImageView."),
            | SwapchainInitError::Framebuffer(ref e)          => write!(f, "{}", e),
        }
    }
}
