
use toml;
use ash::vk;

use config::engine::ConfigMirror;
use config::error::{ ConfigError, MappingError };

use gsvk::core::debug::ValidationConfig;

#[derive(Deserialize, Default)]
pub(crate) struct ValidationConfigMirror {
    enable: bool,
    layers: Vec<String>,
    flags : Vec<String>,
}

impl ConfigMirror for ValidationConfigMirror {
    type ConfigType = ValidationConfig;

    fn into_config(self) -> Result<Self::ConfigType, ConfigError> {

        let mut flags = vk::DebugReportFlagsEXT::empty();
        for raw_flag in self.flags.iter() {
            flags |= vk_raw2debug_reqport_flag(raw_flag)?;
        }

        let config = ValidationConfig {
            is_enable: self.enable,
            required_validation_layers: self.layers,
            flags,
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> Result<(), ConfigError> {

        if let Some(v) = toml.get("enable") {
            self.enable = v.as_bool().ok_or(ConfigError::ParseError)?;
        }

        if let Some(v) = toml.get("layers") {
            if let Some(layers) = v.as_array() {
                if layers.len() > 0 {
                    self.layers.clear();

                    for layer in layers {
                        let value = layer.as_str().ok_or(ConfigError::ParseError)?;
                        self.layers.push(value.to_owned());
                    }
                }
            } else {
                return Err(ConfigError::ParseError);
            }
        }

        if let Some(v) = toml.get("flags") {
            if let Some(flags) = v.as_array() {
                if flags.len() > 0 {
                    self.flags.clear();

                    for flag in flags {
                        let value = flag.as_str().ok_or(ConfigError::ParseError)?;
                        self.flags.push(value.to_owned());
                    }
                }
            } else {
                return Err(ConfigError::ParseError);
            }
        }

        Ok(())
    }
}

fn vk_raw2debug_reqport_flag(raw: &String) -> Result<vk::DebugReportFlagsEXT, ConfigError> {

    let flag = match raw.as_str() {
        | "Error"              => vk::DebugReportFlagsEXT::ERROR,
        | "Warning"            => vk::DebugReportFlagsEXT::WARNING,
        | "PerformanceWarning" => vk::DebugReportFlagsEXT::PERFORMANCE_WARNING,
        | "Debug"              => vk::DebugReportFlagsEXT::DEBUG,
        | "Information"        => vk::DebugReportFlagsEXT::INFORMATION,
        | _ => return Err(ConfigError::Mapping(MappingError::DebugReportError)),
    };

    Ok(flag)
}
