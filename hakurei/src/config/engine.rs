
use toml;

use config::core::{ CoreConfig, CoreConfigMirror };
use config::default::defalut_config_toml;
use config::window::{ WindowConfig, WindowConfigMirror };
use config::pipeline::{ PipelineConfig, PipelineConfigMirror };
use config::resources::{ ResourceConfig, ResourceConfigMirror };
use config::error::ConfigError;

pub(crate) trait ConfigMirror {
    type ConfigType;

    /// Parse raw configuration to actual configuration type.
    fn into_config(self) -> Result<Self::ConfigType, ConfigError>;
    /// Parse the configuration from the toml table. Also overrides previous values if needed.
    fn parse(&mut self, toml: &toml::Value) -> Result<(), ConfigError>;
}

pub(crate) struct EngineConfig {

    pub core     : CoreConfig,
    pub window   : WindowConfig,
    pub pipeline : PipelineConfig,
    pub resources: ResourceConfig,
}

#[derive(Deserialize, Default)]
struct EngineConfigMirror {

    core     : CoreConfigMirror,
    window   : WindowConfigMirror,
    pipeline : PipelineConfigMirror,
    resources: ResourceConfigMirror,
}

impl ConfigMirror for EngineConfigMirror {
    type ConfigType = EngineConfig;

    fn into_config(self) -> Result<Self::ConfigType, ConfigError> {

        let config = EngineConfig {
            core     : self.core.into_config()?,
            window   : self.window.into_config()?,
            pipeline : self.pipeline.into_config()?,
            resources: self.resources.into_config()?,
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> Result<(), ConfigError> {

        if let Some(v) = toml.get("core") {
            self.core.parse(v)?;
        }

        if let Some(v) = toml.get("window") {
            self.window.parse(v)?;
        }

        if let Some(v) = toml.get("pipeline") {
            self.pipeline.parse(v)?;
        }

        if let Some(v) = toml.get("resources") {
            self.resources.parse(v)?;
        }

        Ok(())
    }
}

impl EngineConfig {

    pub fn init() -> Result<EngineConfig, ConfigError> {

        let mut unset_configs = EngineConfigMirror::default();
        let toml_configs = defalut_config_toml();

        unset_configs.parse(&toml_configs)?;
        let final_configs = unset_configs.into_config()?;

        Ok(final_configs)
    }
}
