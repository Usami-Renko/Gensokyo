
use config::core::CoreConfig;
use config::window::WindowConfig;
use config::swapchain::SwapchainConfig;
use config::image::ImageLoadConfig;

pub struct EngineConfig {

    pub core      : CoreConfig,
    pub window    : WindowConfig,
    pub swapchain : SwapchainConfig,
    pub image_load: ImageLoadConfig,
}

impl Default for EngineConfig {

    fn default() -> EngineConfig {
        EngineConfig {
            core      : CoreConfig::default(),
            window    : WindowConfig::default(),
            swapchain : SwapchainConfig::default(),
            image_load: ImageLoadConfig::default(),
        }
    }
}
