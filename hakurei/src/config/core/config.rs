
use toml;
use ash::vk::uint32_t;

use config::engine::ConfigMirror;
use config::core::{ ValidationConfig, ValidationConfigMirror };
use config::core::{ DeviceConfig, DeviceConfigMirror };
use config::core::{ SwapchainConfig, SwapchainConfigMirror };
use config::error::ConfigError;

use core::physical::PhysicalRequirement;

use utility::cast;

pub(crate) struct CoreConfig {

    pub version_api: uint32_t,
    pub version_application: uint32_t,
    pub version_engine: uint32_t,

    pub name_application: String,
    pub name_engine: String,

    pub validation: ValidationConfig,
    pub device    : DeviceConfig,
    pub swapchain : SwapchainConfig,
}

#[derive(Deserialize, Default)]
pub(crate) struct CoreConfigMirror {

    version: Version,
    name: Name,

    validation: ValidationConfigMirror,
    device    : DeviceConfigMirror,
    swapchain : SwapchainConfigMirror,
}


impl ConfigMirror for CoreConfigMirror {
    type ConfigType = CoreConfig;

    fn into_config(self) -> Result<Self::ConfigType, ConfigError> {

        let config = CoreConfig {

            version_api: cast::vk_to_version(&self.version.api)?,
            version_application: cast::vk_to_version(&self.version.application)?,
            version_engine: cast::vk_to_version(&self.version.engine)?,

            name_application: self.name.application,
            name_engine: self.name.engine,

            validation: self.validation.into_config()?,
            device    : self.device.into_config()?,
            swapchain : self.swapchain.into_config()?,
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

        if let Some(v) = toml.get("validation") {
            self.validation.parse(v)?;
        }

        if let Some(v) = toml.get("device") {
            self.device.parse(v)?;
        }

        if let Some(v) = toml.get("swapchain") {
            self.swapchain.parse(v)?;
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

impl CoreConfig {

    pub(crate) fn to_physical_requirement(&self) -> PhysicalRequirement {
        PhysicalRequirement::init()
            .require_device_types(self.device.device_types.clone())
            .require_features(self.device.features.clone())
            .require_queue_extensions(self.device.extensions.clone())
            .require_queue_operations(self.device.queue_operations.clone())
            .require_swapchain_image_count(self.swapchain.image_count)
    }
}