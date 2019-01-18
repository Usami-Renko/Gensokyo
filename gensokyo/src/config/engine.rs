
use toml;

use gsvk::core::config::CoreConfig;
use gsvk::pipeline::config::PipelineConfig;

use crate::config::manifest;
use crate::config::core::CoreConfigMirror;
use crate::config::window::{ WindowConfig, WindowConfigMirror };
use crate::config::pipeline::PipelineConfigMirror;
use crate::config::resources::{ ResourceConfig, ResourceConfigMirror };
use crate::error::{ GsResult, GsError, GsErrorKind };

use std::env;
use std::fs;
use std::path::PathBuf;
use std::io::Read;

use failure::ResultExt;

const MANIFEST_CONFIG_NAME: &str = "gensokyo.toml";

pub(crate) trait ConfigMirror {
    type ConfigType;

    /// Parse raw configuration to actual configuration type.
    fn into_config(self) -> GsResult<Self::ConfigType>;
    /// Parse the configuration from the toml table. Also overrides previous values if needed.
    fn parse(&mut self, toml: &toml::Value) -> GsResult<()>;
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

    fn into_config(self) -> GsResult<Self::ConfigType> {

        let config = EngineConfig {
            core     : self.core.into_config()?,
            window   : self.window.into_config()?,
            pipeline : self.pipeline.into_config()?,
            resources: self.resources.into_config()?,
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> GsResult<()> {

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

    pub fn init(manifest: Option<PathBuf>) -> GsResult<EngineConfig> {

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
                .or(Err(GsError::config("Failed to read user manifest, due to its syntax error.")))?;
            // override original configurations.
            program_config.parse(&user_config)?;
        }

        let final_configs = program_config.into_config()?;
        Ok(final_configs)
    }

    /// Iteratively search for `MANIFEST_CONFIG_NAME` starting at the current working directory and working up through its parents.
    ///
    /// Returns the path to the file or an None if the file couldn't be found.
    fn search_manifest() -> GsResult<Option<PathBuf>> {

        let cwd = env::current_dir()
            .with_context(|_| GsErrorKind::Io)?;
        let mut current = cwd.as_path();

        loop {

            let manifest = current.join(MANIFEST_CONFIG_NAME);
            if fs::metadata(&manifest).is_ok() {
                // succeed to find manifest configuration file.
                return Ok(Some(manifest))
            }

            // continue search its parent directory.
            match current.parent() {
                Some(p) => current = p,
                None => break,
            }
        }

        Ok(None)
    }

    /// Read the manifest file content to string.
    fn read_manifest(at_path: PathBuf) -> GsResult<String> {

        let mut file_handle = fs::File::open(at_path.clone())
            .with_context(|_| GsErrorKind::path(at_path))?;
        let mut contents = String::new();
        file_handle.read_to_string(&mut contents)
            .or(Err(GsError::other("Unable to read Manifest content.")))?;

        return Ok(contents)
    }
}
