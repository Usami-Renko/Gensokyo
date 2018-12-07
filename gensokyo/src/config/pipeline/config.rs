
use toml;

use gsvk::pipeline::config::PipelineConfig;

use crate::config::engine::ConfigMirror;
use crate::config::pipeline::DepthStencilConfigMirror;
use crate::config::error::ConfigError;

#[derive(Deserialize, Default)]
pub(crate) struct PipelineConfigMirror {

    depth_stencil: DepthStencilConfigMirror,
}

impl ConfigMirror for PipelineConfigMirror {
    type ConfigType = PipelineConfig;

    fn into_config(self) -> Result<Self::ConfigType, ConfigError> {

        let config = PipelineConfig {
            depth_stencil: self.depth_stencil.into_config()?,
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> Result<(), ConfigError> {

        if let Some(v) = toml.get("depth_stencil") {
            self.depth_stencil.parse(v)?;
        }

        Ok(())
    }
}
