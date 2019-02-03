
use toml;
use serde_derive::Deserialize;

use crate::assets::io::ImageLoadConfig;

use crate::config::engine::ConfigMirror;
use crate::config::resources::ImageLoadConfigMirror;
use crate::error::GsResult;

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

    fn into_config(self) -> GsResult<Self::ConfigType> {

        let config = ResourceConfig {
            image_load: self.image_load.into_config()?,
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> GsResult<()> {

        if let Some(v) = toml.get("image_load") {
            self.image_load.parse(v)?;
        }

        Ok(())
    }
}
