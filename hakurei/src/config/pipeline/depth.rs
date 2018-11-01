
use toml;
use ash::vk;

use config::engine::ConfigMirror;
use config::macros::vk_string_to_format;
use config::error::{ ConfigError, MappingError };

pub(crate) struct DepthStencilConfig {

    /// The prefer format for depth or stencil buffer.
    ///
    /// Although this format can be specified in pipeline creation, it's recommended to specify the format in this config setting, because in this way the hakurei engine can help to check if this format is supported in the system.
    ///
    /// The pipeline will use the first format which support VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT in vkGetPhysicalDeviceFormatProperties call.
    pub prefer_depth_stencil_formats: Vec<vk::Format>,
    /// The prefer image tiling mode for depth or stencil buffer.
    pub prefer_image_tiling: vk::ImageTiling,
}

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
            prefer_depth_stencil_formats.push(vk_string_to_format(raw_format)?);
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

fn vk_raw2image_tiling(raw: &String) -> Result<vk::ImageTiling, ConfigError> {

    let tiling = match raw.as_str() {
        | "Optimal" => vk::ImageTiling::Optimal,
        | "Linear"  => vk::ImageTiling::Linear,
        | _ => return Err(ConfigError::Mapping(MappingError::ImgTilingMappingError)),
    };

    Ok(tiling)
}
