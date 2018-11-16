
use toml;

use resources::image::io::ImageLoadConfig;

use config::engine::ConfigMirror;
use config::resources::ImageLoadConfigMirror;
use config::error::ConfigError;

#[derive(Debug, Clone)]
pub(crate) struct ResourceConfig {

    pub image_load: ImageLoadConfig,
}

#[derive(Deserialize, Default)]
pub(crate) struct ResourceConfigMirror {
    image_load: ImageLoadConfigMirror,
}

impl ConfigMirror for ResourceConfigMirror {
    type ConfigType = ResourceConfig;

    fn into_config(self) -> Result<Self::ConfigType, ConfigError> {

        let config = ResourceConfig {
            image_load: self.image_load.into_config()?,
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> Result<(), ConfigError> {

        if let Some(v) = toml.get("image_load") {
            self.image_load.parse(v)?;
        }

        Ok(())
    }
}
