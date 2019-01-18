
use ash::vk_make_version;

use crate::config::engine::ConfigMirror;
use crate::error::{ GsResult, GsError };

use gsvk::core::instance::InstanceConfig;
use gsvk::types::vkuint;

#[derive(Deserialize, Default)]
pub struct InstanceConfigMirror {

    version : Version,
    name    : Name,
}

impl ConfigMirror for InstanceConfigMirror {
    type ConfigType = InstanceConfig;

    fn into_config(self) -> GsResult<Self::ConfigType> {

        let config = InstanceConfig {

            api_version         : vk_to_version(&self.version.api)?,
            application_version : vk_to_version(&self.version.application)?,
            engine_version      : vk_to_version(&self.version.engine)?,

            application_name : self.name.application,
            engine_name      : self.name.engine,
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> GsResult<()> {

        if let Some(v) = toml.get("version") {

            if let Some(v) = v.get("api") {
                self.version.api = v.as_str()
                    .ok_or(GsError::config("Version Api"))?.to_owned();
            }
            if let Some(v) = v.get("application") {
                self.version.application = v.as_str()
                    .ok_or(GsError::config("Version Application"))?.to_owned();
            }
            if let Some(v) = v.get("engine") {
                self.version.engine = v.as_str()
                    .ok_or(GsError::config("Version Engine"))?.to_owned();
            }
        }

        if let Some(v) = toml.get("name") {

            if let Some(v) = v.get("application") {
                let application_name = v.as_str()
                    .ok_or(GsError::config("Name Application"))?.to_owned();
                self.name.application = Some(application_name);
            }
            if let Some(v) = v.get("engine") {
                let engine_name = v.as_str()
                    .ok_or(GsError::config("Name Engine"))?.to_owned();
                self.name.engine = Some(engine_name);
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
    pub application: Option<String>,
    pub engine     : Option<String>,
}

fn vk_to_version(raw_version: &String) -> GsResult<vkuint> {

    let versions: Vec<_> = raw_version.split('.').collect();

    if versions.len() == 3 {

        let major = versions[0].parse::<vkuint>()
            .or(Err(GsError::config("Parse Vulkan Major Version")))?;
        let minor = versions[1].parse::<vkuint>()
            .or(Err(GsError::config("Parse Vulkan Minor Version")))?;
        let patch = versions[2].parse::<vkuint>()
            .or(Err(GsError::config("Parse Vulkan Patch Version")))?;

        let version = vk_make_version!(major, minor, patch);

        Ok(version)
    } else {
        Err(GsError::config("Parse Vulkan Instance Version"))
    }
}
