
use toml;

use crate::assets::io::ImageLoadConfig;
use crate::config::engine::ConfigMirror;
use crate::error::{ GsResult, GsError };

use gsvk::types::format::GsFormat;

#[derive(Deserialize, Default)]
pub(crate) struct ImageLoadConfigMirror {

    flip_vertical  : bool,
    flip_horizontal: bool,
    byte_per_pixel : u32,
    force_rgba     : bool,
}

impl ConfigMirror for ImageLoadConfigMirror {
    type ConfigType = ImageLoadConfig;

    fn into_config(self) -> GsResult<Self::ConfigType> {

        let config = ImageLoadConfig {
            flip_vertical  : self.flip_vertical,
            flip_horizontal: self.flip_horizontal,
            byte_per_pixel : self.byte_per_pixel,
            force_rgba     : self.force_rgba,
            // TODO: Reset this member from toml setting.
            img_format     : GsFormat::RGBA8_UNORM,
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> GsResult<()> {

        if let Some(v) = toml.get("flip_vertical") {
            self.flip_vertical = v.as_bool()
                .ok_or(GsError::config("flip_vertical"))?;
        }

        if let Some(v) = toml.get("flip_horizontal") {
            self.flip_horizontal = v.as_bool()
                .ok_or(GsError::config("flip_horizontal"))?;
        }

        if let Some(v) = toml.get("byte_per_pixel") {
            self.byte_per_pixel = v.as_integer()
                .ok_or(GsError::config("byte_per_pixel"))? as _;
        }

        if let Some(v) = toml.get("force_rgba") {
            self.force_rgba = v.as_bool()
                .ok_or(GsError::config("force_rgba"))?;
        }

        Ok(())
    }
}
