
use toml;
use serde_derive::Deserialize;

use gsvk::pipeline::config::PipelineConfig;

use crate::config::engine::ConfigMirror;
use crate::config::pipeline::DepthStencilConfigMirror;
use crate::error::GsResult;

#[derive(Deserialize, Default)]
pub(crate) struct PipelineConfigMirror {

    depth_stencil: DepthStencilConfigMirror,
}

impl ConfigMirror for PipelineConfigMirror {
    type ConfigType = PipelineConfig;

    fn into_config(self) -> GsResult<Self::ConfigType> {

        let config = PipelineConfig {
            depth_stencil: self.depth_stencil.into_config()?,
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> GsResult<()> {

        if let Some(v) = toml.get("depth_stencil") {
            self.depth_stencil.parse(v)?;
        }

        Ok(())
    }
}
