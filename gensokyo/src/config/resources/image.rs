
use toml;
use ash::vk;

use crate::assets::io::ImageLoadConfig;

use crate::config::engine::ConfigMirror;
use crate::config::error::ConfigError;

#[derive(Deserialize, Default)]
pub(crate) struct ImageLoadConfigMirror {

    flip_vertical  : bool,
    flip_horizontal: bool,
    byte_per_pixel : u32,
    force_rgba     : bool,
}

impl ConfigMirror for ImageLoadConfigMirror {
    type ConfigType = ImageLoadConfig;

    fn into_config(self) -> Result<Self::ConfigType, ConfigError> {

        let config = ImageLoadConfig {
            flip_vertical  : self.flip_vertical,
            flip_horizontal: self.flip_horizontal,
            byte_per_pixel : self.byte_per_pixel,
            force_rgba     : self.force_rgba,
            // TODO: Reset this member from tom setting.
            img_format     : vk::Format::R8G8B8A8_UNORM,
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> Result<(), ConfigError> {

        if let Some(v) = toml.get("flip_vertical") {
            self.flip_vertical = v.as_bool().ok_or(ConfigError::ParseError)?
        }

        if let Some(v) = toml.get("flip_horizontal") {
            self.flip_horizontal = v.as_bool().ok_or(ConfigError::ParseError)?
        }

        if let Some(v) = toml.get("byte_per_pixel") {
            self.byte_per_pixel = v.as_integer().ok_or(ConfigError::ParseError)? as u32;
        }

        if let Some(v) = toml.get("force_rgba") {
            self.force_rgba = v.as_bool().ok_or(ConfigError::ParseError)?;
        }

        Ok(())
    }
}
