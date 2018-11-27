
use toml;

use gsvk::core::config::CoreConfig;
use gsvk::pipeline::config::PipelineConfig;

use config::manifest;
use config::core::CoreConfigMirror;
use config::window::{ WindowConfig, WindowConfigMirror };
use config::pipeline::PipelineConfigMirror;
use config::resources::{ ResourceConfig, ResourceConfigMirror };
use config::error::ConfigError;

use std::env;
use std::fs;
use std::path::PathBuf;
use std::io::Read;

const MANIFEST_CONFIG_NAME: &str = "hakurei.toml";

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

    pub fn init(manifest: Option<PathBuf>) -> Result<EngineConfig, ConfigError> {

        let mut program_config = EngineConfigMirror::default();
        let toml_configs = manifest::manifest_toml();

        // initialize configuration with default setting.
        program_config.parse(&toml_configs)?;

        // try to search for user's configuration setting.
        let user_config_path = if let Some(_) = manifest {
            manifest
        } else {
            EngineConfig::search_manifest()?
        };

        if let Some(config_path) = user_config_path {

            let config_content = EngineConfig::read_manifest(config_path)?;
            let user_config = config_content.parse::<toml::Value>()
                .map_err(|e| ConfigError::UserConfigSyntaxError(e))?;
            // override original configurations.
            program_config.parse(&user_config)?;
        }

        let final_configs = program_config.into_config()?;

        Ok(final_configs)
    }

    /// Iteratively search for `MANIFEST_CONFIG_NAME` starting at the current working directory and working up through its parents.
    ///
    /// Returns the path to the file or an None if the file couldn't be found.
    fn search_manifest() -> Result<Option<PathBuf>, ConfigError> {

        let cwd = env::current_dir()
            .map_err(|_| ConfigError::DirectoryAccessError)?;
        let mut current = cwd.as_path();

        loop {

            let manifest = current.join(MANIFEST_CONFIG_NAME);
            if fs::metadata(&manifest).is_ok() {
                // succeed to find manifest configuration file.
                return Ok(Some(manifest))
            }

            // continute search its parent directory.
            match current.parent() {
                Some(p) => current = p,
                None => break,
            }
        }

        Ok(None)
    }

    /// Read the manifest file content to string.
    fn read_manifest(at_path: PathBuf) -> Result<String, ConfigError> {

        let mut file_handle = fs::File::open(at_path)
            .map_err(|_| ConfigError::IoError)?;
        let mut contents = String::new();
        file_handle.read_to_string(&mut contents)
            .map_err(|_| ConfigError::IoError)?;

        return Ok(contents)
    }
}
