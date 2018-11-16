
use toml;

use config::engine::ConfigMirror;
use config::error::ConfigError;

use vk::utils::types::{ vkint, vkDimension2D };

#[derive(Debug, Clone)]
pub(crate) struct WindowConfig {

    pub title: String,
    pub mode : String,
    pub always_on_top: bool,
    pub is_resizable : bool,

    pub dimension: vkDimension2D,
    pub min_dimension: Option<vkDimension2D>,
    pub max_dimension: Option<vkDimension2D>,

    pub is_cursor_grap: bool,
    pub is_cursor_hide: bool,
}

#[derive(Deserialize, Default)]
pub(crate) struct WindowConfigMirror {

    title: String,
    mode : String,
    always_on_top: bool,
    is_resizable : bool,

    dimension: Dimension,
    cursor: Cursor,
}

#[derive(Deserialize, Default)]
struct Dimension {
    width : vkint,
    height: vkint,
    min_width : vkint,
    min_height: vkint,
    max_width : vkint,
    max_height: vkint,
}

#[derive(Deserialize, Default)]
struct Cursor {
    is_grab: bool,
    is_hide: bool,
}

impl ConfigMirror for WindowConfigMirror {
    type ConfigType = WindowConfig;

    fn into_config(self) -> Result<Self::ConfigType, ConfigError> {

        let config = WindowConfig {
            title: self.title,
            mode : self.mode,
            always_on_top: self.always_on_top,
            is_resizable : self.is_resizable,
            dimension: vkDimension2D {
                width : self.dimension.width,
                height: self.dimension.height,
            },
            min_dimension: parse_dimension(self.dimension.min_width, self.dimension.min_height),
            max_dimension: parse_dimension(self.dimension.max_width, self.dimension.max_height),
            is_cursor_grap: self.cursor.is_grab,
            is_cursor_hide: self.cursor.is_hide,
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> Result<(), ConfigError> {

        if let Some(v) = toml.get("title") {
            self.title = v.as_str().ok_or(ConfigError::ParseError)?.to_owned();
        }
        if let Some(v) = toml.get("mode") {
            self.mode = v.as_str().ok_or(ConfigError::ParseError)?.to_owned();
        }
        if let Some(v) = toml.get("always_on_top") {
            self.always_on_top = v.as_bool().ok_or(ConfigError::ParseError)?;
        }
        if let Some(v) = toml.get("is_resizable") {
            self.is_resizable = v.as_bool().ok_or(ConfigError::ParseError)?;
        }

        if let Some(v) = toml.get("dimension") {

            if let Some(v) = v.get("width") {
                self.dimension.width = v.as_integer().ok_or(ConfigError::ParseError)? as vkint;
            }
            if let Some(v) = v.get("height") {
                self.dimension.height = v.as_integer().ok_or(ConfigError::ParseError)? as vkint;
            }
            if let Some(v) = v.get("min_width") {
                self.dimension.min_width = v.as_integer().ok_or(ConfigError::ParseError)? as vkint;
            }
            if let Some(v) = v.get("min_height") {
                self.dimension.min_height = v.as_integer().ok_or(ConfigError::ParseError)? as vkint;
            }
            if let Some(v) = v.get("max_width") {
                self.dimension.max_width = v.as_integer().ok_or(ConfigError::ParseError)? as vkint;
            }
            if let Some(v) = v.get("max_height") {
                self.dimension.max_height = v.as_integer().ok_or(ConfigError::ParseError)? as vkint;
            }
        }

        if let Some(v) = toml.get("cursor") {

            if let Some(v) = v.get("is_grab") {
                self.cursor.is_grab = v.as_bool().ok_or(ConfigError::ParseError)?;
            }
            if let Some(v) = v.get("is_hide") {
                self.cursor.is_hide = v.as_bool().ok_or(ConfigError::ParseError)?;
            }
        }

        Ok(())
    }
}

fn parse_dimension(width: vkint, height: vkint) -> Option<vkDimension2D> {

    if width == vkint::default() || height == vkint::default() {
        None
    } else {
        let min_dimension = vkDimension2D {
            width, height,
        };
        Some(min_dimension)
    }
}
