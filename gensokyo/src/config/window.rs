
use toml;

use crate::config::engine::ConfigMirror;
use crate::error::{ GsResult, GsError };

use gsvk::types::{ vkuint, vkDim2D };

#[derive(Debug, Clone)]
pub(crate) struct WindowConfig {

    pub title: String,
    pub mode : String,
    pub always_on_top: bool,
    pub is_resizable : bool,

    pub dimension: vkDim2D,
    pub min_dimension: Option<vkDim2D>,
    pub max_dimension: Option<vkDim2D>,

    pub is_cursor_grap: bool,
    pub is_cursor_hide: bool,
}

#[derive(Deserialize)]
pub(crate) struct WindowConfigMirror {

    title: String,
    mode : String,
    always_on_top: bool,
    is_resizable : bool,

    dimension: Dimension,
    cursor: Cursor,
}

#[derive(Deserialize)]
struct Dimension {
    width : vkuint,
    height: vkuint,
    min_width : vkuint,
    min_height: vkuint,
    max_width : vkuint,
    max_height: vkuint,
}

#[derive(Deserialize)]
struct Cursor {
    is_grab: bool,
    is_hide: bool,
}

impl Default for WindowConfigMirror {

    fn default() -> WindowConfigMirror {
        WindowConfigMirror {
            title: String::from("Gensokyo Rendering Engine"),
            mode : String::from("normal"),
            always_on_top: false,
            is_resizable : true,

            dimension: Dimension {
                width : 800,
                height: 600,
                min_width : 400,
                min_height: 300,
                max_width : 1280,
                max_height: 720,
            },
            cursor: Cursor {
                is_grab: false,
                is_hide: false,
            },
        }
    }
}

impl ConfigMirror for WindowConfigMirror {
    type ConfigType = WindowConfig;

    fn into_config(self) -> GsResult<Self::ConfigType> {

        let config = WindowConfig {
            title: self.title,
            mode : self.mode,
            always_on_top: self.always_on_top,
            is_resizable : self.is_resizable,
            dimension: vkDim2D {
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

    fn parse(&mut self, toml: &toml::Value) -> GsResult<()> {

        if let Some(v) = toml.get("title") {
            self.title = v.as_str()
                .ok_or(GsError::config("[window.title]"))?.to_owned();
        }
        if let Some(v) = toml.get("mode") {
            self.mode = v.as_str()
                .ok_or(GsError::config("[window.mode]"))?.to_owned();
        }
        if let Some(v) = toml.get("always_on_top") {
            self.always_on_top = v.as_bool()
                .ok_or(GsError::config("[window.always_on_top]"))?.to_owned();
        }
        if let Some(v) = toml.get("is_resizable") {
            self.is_resizable = v.as_bool()
                .ok_or(GsError::config("[window.is_resizable]"))?.to_owned();
        }

        if let Some(v) = toml.get("dimension") {

            if let Some(v) = v.get("width") {
                self.dimension.width = v.as_integer()
                    .ok_or(GsError::config("[window.dimension.width]"))?.to_owned() as _;
            }
            if let Some(v) = v.get("height") {
                self.dimension.height = v.as_integer()
                    .ok_or(GsError::config("[window.dimension.height]"))?.to_owned() as _;
            }
            if let Some(v) = v.get("min_width") {
                self.dimension.min_width = v.as_integer()
                    .ok_or(GsError::config("[window.dimension.min_width]"))?.to_owned() as _;
            }
            if let Some(v) = v.get("min_height") {
                self.dimension.min_height = v.as_integer()
                    .ok_or(GsError::config("[window.dimension.min_height]"))?.to_owned() as _;
            }
            if let Some(v) = v.get("max_width") {
                self.dimension.max_width = v.as_integer()
                    .ok_or(GsError::config("[window.dimension.max_width]"))?.to_owned() as _;
            }
            if let Some(v) = v.get("max_height") {
                self.dimension.max_height = v.as_integer()
                    .ok_or(GsError::config("[window.dimension.max_height]"))?.to_owned() as _;
            }
        }

        if let Some(v) = toml.get("cursor") {

            if let Some(v) = v.get("is_grab") {
                self.cursor.is_grab = v.as_bool()
                    .ok_or(GsError::config("[window.cursor.is_grab]"))?.to_owned();
            }
            if let Some(v) = v.get("is_hide") {
                self.cursor.is_hide = v.as_bool()
                    .ok_or(GsError::config("[window.cursor.is_hide]"))?.to_owned();
            }
        }

        Ok(())
    }
}

fn parse_dimension(width: vkuint, height: vkuint) -> Option<vkDim2D> {

    if width == vkuint::default() || height == vkuint::default() {
        None
    } else {
        let min_dimension = vkDim2D {
            width, height,
        };
        Some(min_dimension)
    }
}
