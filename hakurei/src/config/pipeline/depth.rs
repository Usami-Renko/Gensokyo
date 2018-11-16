
use toml;

use vk::resources::image::ImageTiling;
use vk::pipeline::config::DepthStencilConfig;
use vk::utils::types::vkformat;

use config::engine::ConfigMirror;
use config::error::{ ConfigError, MappingError };

#[derive(Deserialize, Default)]
pub(crate) struct DepthStencilConfigMirror {
    prefer_depth_stencil_formats: Vec<String>,
    prefer_image_tiling: String,
}

impl ConfigMirror for DepthStencilConfigMirror {
    type ConfigType = DepthStencilConfig;

    fn into_config(self) -> Result<Self::ConfigType, ConfigError> {

        let mut prefer_depth_stencil_formats = vec![];
        for raw_format in self.prefer_depth_stencil_formats.iter() {

            use vk::utils::format::vk_string_to_format;
            prefer_depth_stencil_formats.push(vk_string_to_format(raw_format));
        }

        let config = DepthStencilConfig {
            prefer_depth_stencil_formats,
            prefer_image_tiling: vk_raw2image_tiling(&self.prefer_image_tiling)?,
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> Result<(), ConfigError> {

        if let Some(v) = toml.get("prefer_depth_stencil_formats") {
            if let Some(formats) = v.as_array() {
                if formats.len() > 0 {
                    self.prefer_depth_stencil_formats.clear();

                    for format in formats {
                        let value = format.as_str().ok_or(ConfigError::ParseError)?;
                        self.prefer_depth_stencil_formats.push(value.to_owned());
                    }
                }
            } else {
                return Err(ConfigError::ParseError);
            }
        }

        if let Some(v) = toml.get("prefer_image_tiling") {
            self.prefer_image_tiling = v.as_str().ok_or(ConfigError::ParseError)?.to_owned();
        }

        Ok(())
    }
}

fn vk_raw2image_tiling(raw: &String) -> Result<ImageTiling, ConfigError> {

    let tiling = match raw.as_str() {
        | "Optimal" => ImageTiling::Optimal,
        | "Linear"  => ImageTiling::Linear,
        | _ => return Err(ConfigError::Mapping(MappingError::ImgTilingMappingError)),
    };

    Ok(tiling)
}
