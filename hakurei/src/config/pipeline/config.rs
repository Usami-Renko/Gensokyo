
use toml;

use config::engine::ConfigMirror;
use config::pipeline::{ DepthStencilConfig, DepthStencilConfigMirror };
use config::error::ConfigError;

pub(crate) struct PipelineConfig {

    pub depth_stencil: DepthStencilConfig,
}

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
