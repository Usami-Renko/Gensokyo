
use toml;

use gsvk::pipeline::config::DepthStencilConfig;

use crate::config::engine::ConfigMirror;
use crate::error::{ GsResult, GsError };

#[derive(Deserialize)]
pub(crate) struct DepthStencilConfigMirror {
    prefer_depth_stencil_formats: Vec<String>,
}

impl Default for DepthStencilConfigMirror {

    fn default() -> DepthStencilConfigMirror {
        DepthStencilConfigMirror {
            prefer_depth_stencil_formats: vec![
                String::from("D32_SFLOAT"),
                String::from("D32_SFLOAT_S8_UINT"),
                String::from("D24_UNORM_S8_UINT"),
            ],
        }
    }
}

impl ConfigMirror for DepthStencilConfigMirror {
    type ConfigType = DepthStencilConfig;

    fn into_config(self) -> GsResult<Self::ConfigType> {

        let mut prefer_depth_stencil_formats = vec![];
        for raw_format in self.prefer_depth_stencil_formats.iter() {

            use gsvk::utils::format::vk_string_to_format;
            prefer_depth_stencil_formats.push(vk_string_to_format(raw_format));
        }

        let config = DepthStencilConfig { prefer_depth_stencil_formats };
        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> GsResult<()> {

        if let Some(v) = toml.get("prefer_depth_stencil_formats") {
            if let Some(formats) = v.as_array() {
                if formats.len() > 0 {
                    self.prefer_depth_stencil_formats.clear();

                    for (i, format) in formats.iter().enumerate() {
                        let value = format.as_str()
                            .ok_or(GsError::config(format!("prefer_depth_stencil_formats #{}", i)))?;
                        self.prefer_depth_stencil_formats.push(value.to_owned());
                    }
                }
            } else {
                return Err(GsError::config("[pipeline.depth_stencil.prefer_depth_stencil_formats]"))
            }
        }

        Ok(())
    }
}
