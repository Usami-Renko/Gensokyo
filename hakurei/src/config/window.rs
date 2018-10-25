
use toml;
use ash::vk::uint32_t;

use config::engine::ConfigMirror;
use config::error::ConfigError;
use utility::dimension::Dimension2D;

#[derive(Debug, Clone)]
pub(crate) struct WindowConfig {

    pub dimension: Dimension2D,
    pub title    : String,
}

#[derive(Deserialize, Default)]
pub(crate) struct WindowConfigMirror {
    title: String,
    dimension: Dimension,
}

#[derive(Deserialize, Default)]
struct Dimension {
    width : uint32_t,
    height: uint32_t,
}

impl ConfigMirror for WindowConfigMirror {
    type ConfigType = WindowConfig;

    fn into_config(self) -> Result<Self::ConfigType, ConfigError> {

        let config = WindowConfig {
            title: self.title,
            dimension: Dimension2D {
                width : self.dimension.width,
                height: self.dimension.height,
            },
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> Result<(), ConfigError> {

        if let Some(v) = toml.get("title") {
            self.title = v.as_str().ok_or(ConfigError::ParseError)?.to_owned();
        }

        if let Some(v) = toml.get("dimension") {
            if let Some(v) = v.get("width") {
                self.dimension.width = v.as_integer().ok_or(ConfigError::ParseError)? as uint32_t;
            }
            if let Some(v) = v.get("height") {
                self.dimension.height = v.as_integer().ok_or(ConfigError::ParseError)? as uint32_t;
            }
        }

        Ok(())
    }
}
