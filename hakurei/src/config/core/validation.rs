
use toml;

use config::engine::ConfigMirror;
use config::error::{ ConfigError, MappingError };

use core::debug::DebugReportFlag;

pub(crate) struct ValidationConfig {
    /// tell if validation layer should be enabled.
    pub is_enable: bool,
    /// the layer names required for validation layer support.
    pub required_validation_layers: Vec<String>,
    /// the message type that Validation Layer would report for.
    pub flags: Vec<DebugReportFlag>,
}

#[derive(Deserialize, Default)]
pub(crate) struct ValidationConfigMirror {
    enable: bool,
    layers: Vec<String>,
    flags : Vec<String>,
}

impl ConfigMirror for ValidationConfigMirror {
    type ConfigType = ValidationConfig;

    fn into_config(self) -> Result<Self::ConfigType, ConfigError> {

        let mut flags = vec![];
        for raw_flag in self.flags.iter() {
            flags.push(vk_raw2debug_reqport_flag(raw_flag)?);
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

fn vk_raw2debug_reqport_flag(raw: &String) -> Result<DebugReportFlag, ConfigError> {

    let flag = match raw.as_str() {
        | "Error"              => DebugReportFlag::ErrorBit,
        | "Warning"            => DebugReportFlag::WarningBit,
        | "PerformanceWarning" => DebugReportFlag::PerformanceWarningBit,
        | "Debug"              => DebugReportFlag::DebugBit,
        | "Information"        => DebugReportFlag::InformationBit,
        | _ => return Err(ConfigError::Mapping(MappingError::DebugReportError)),
    };

    Ok(flag)
}
