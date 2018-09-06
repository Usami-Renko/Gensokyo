
use config::window::WindowConfig;
use config::swapchain::SwapchainConfig;
use config::image::ImageLoadConfig;

pub struct EngineConfig {

    pub window    : WindowConfig,
    pub swapchain : SwapchainConfig,
    pub image_load: ImageLoadConfig,
}

impl Default for EngineConfig {

    fn default() -> EngineConfig {
        EngineConfig {
            window    : WindowConfig::default(),
            swapchain : SwapchainConfig::default(),
            image_load: ImageLoadConfig::default(),
        }
    }
}
