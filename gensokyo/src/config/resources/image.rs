
use toml;

use crate::assets::io::ImageLoadConfig;
use crate::config::engine::ConfigMirror;
use crate::error::{ GsResult, GsError };

#[derive(Deserialize)]
pub(crate) struct ImageLoadConfigMirror {

    flip_vertical  : bool,
    flip_horizontal: bool,
    byte_per_pixel : u32,
    force_rgba     : bool,
    image_format   : String,
}

impl Default for ImageLoadConfigMirror {

    fn default() -> ImageLoadConfigMirror {
        ImageLoadConfigMirror {
            flip_vertical  : false,
            flip_horizontal: false,
            force_rgba     : true,
            byte_per_pixel : 4,
            image_format   : String::from("R8G8B8A8_UNORM"),
        }
    }
}

impl ConfigMirror for ImageLoadConfigMirror {
    type ConfigType = ImageLoadConfig;

    fn into_config(self) -> GsResult<Self::ConfigType> {

        use gsvk::utils::format::vk_string_to_format;

        let config = ImageLoadConfig {
            flip_vertical  : self.flip_vertical,
            flip_horizontal: self.flip_horizontal,
            byte_per_pixel : self.byte_per_pixel,
            force_rgba     : self.force_rgba,
            img_format     : vk_string_to_format(&self.image_format),
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> GsResult<()> {

        if let Some(v) = toml.get("flip_vertical") {
            self.flip_vertical = v.as_bool()
                .ok_or(GsError::config("resources.image_load.flip_vertical"))?;
        }

        if let Some(v) = toml.get("flip_horizontal") {
            self.flip_horizontal = v.as_bool()
                .ok_or(GsError::config("resources.image_load.flip_horizontal"))?;
        }

        if let Some(v) = toml.get("byte_per_pixel") {
            self.byte_per_pixel = v.as_integer()
                .ok_or(GsError::config("resources.image_load.byte_per_pixel"))? as _;
        }

        if let Some(v) = toml.get("force_rgba") {
            self.force_rgba = v.as_bool()
                .ok_or(GsError::config("resources.image_load.force_rgba"))?;
        }

        if let Some(v) = toml.get("image_format") {
            self.image_format = v.as_str()
                .ok_or(GsError::config("resources.image_load.image_format"))?.to_owned();
        }

        Ok(())
    }
}
