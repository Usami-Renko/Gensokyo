
use toml;

use config::engine::ConfigMirror;
use config::error::ConfigError;

#[derive(Debug, Clone)]
pub(crate) struct ImageLoadConfig {

    /// flip_vertical define whether to flip vertical when loading image.
    pub flip_vertical  : bool,
    /// flip_horizontal define whether to flip horizontal when loading image.
    pub flip_horizontal: bool,
    /// byte_per_pixel define the byte count in per pixel.
    pub byte_per_pixel: u32,
    /// force_rgba define whether to load the image from file with rgba channel.
    pub force_rgba: bool,
}

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
