
use config::resources::ImageLoadConfig;

pub struct ResourceConfig {

    pub image_load: ImageLoadConfig,
}

impl Default for ResourceConfig {

    fn default() -> ResourceConfig {
        ResourceConfig {
            image_load: ImageLoadConfig::default(),
        }
    }
}

