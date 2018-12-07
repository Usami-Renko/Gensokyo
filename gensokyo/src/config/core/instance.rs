
use gsvk::core::instance::InstanceConfig;

use crate::config::engine::ConfigMirror;
use crate::config::error::ConfigError;

#[derive(Deserialize, Default)]
pub struct InstanceConfigMirror {

    version   : Version,
    name      : Name,
}

impl ConfigMirror for InstanceConfigMirror {
    type ConfigType = InstanceConfig;

    fn into_config(self) -> Result<Self::ConfigType, ConfigError> {

        use crate::utils::shortcuts::vk_to_version;

        let config = InstanceConfig {

            version_api         : vk_to_version(&self.version.api)?,
            version_application : vk_to_version(&self.version.application)?,
            version_engine      : vk_to_version(&self.version.engine)?,

            name_application : self.name.application,
            name_engine      : self.name.engine,
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> Result<(), ConfigError> {

        if let Some(v) = toml.get("version") {

            if let Some(v) = v.get("api") {
                self.version.api = v.as_str().ok_or(ConfigError::ParseError)?.to_owned();
            }
            if let Some(v) = v.get("application") {
                self.version.application = v.as_str().ok_or(ConfigError::ParseError)?.to_owned();
            }
            if let Some(v) = v.get("engine") {
                self.version.engine = v.as_str().ok_or(ConfigError::ParseError)?.to_owned();
            }
        }

        if let Some(v) = toml.get("name") {

            if let Some(v) = v.get("application") {
                self.name.application = v.as_str().ok_or(ConfigError::ParseError)?.to_owned();
            }
            if let Some(v) = v.get("engine") {
                self.name.engine = v.as_str().ok_or(ConfigError::ParseError)?.to_owned();
            }
        }

        Ok(())
    }
}


#[derive(Deserialize, Default)]
struct Version {
    pub api        : String,
    pub application: String,
    pub engine     : String,
}

#[derive(Deserialize, Default)]
struct Name {
    pub application: String,
    pub engine     : String,
}
