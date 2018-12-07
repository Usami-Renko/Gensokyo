
use toml;
use ash::vk;

use crate::config::engine::ConfigMirror;
use crate::config::error::{ ConfigError, MappingError };

use gsvk::core::debug::{ ValidationConfig, DebugReportConfig, DebugUtilsConfig, DebugInstanceType };

#[derive(Deserialize, Default)]
pub(crate) struct ValidationConfigMirror {

    enable: bool,
    layers: Vec<String>,

    instance_type: Option<String>,
    report_config: Option<DebugReportConfigMirror>,
    utils_config : Option<DebugUtilsConfigMirror>,
}

impl ConfigMirror for ValidationConfigMirror {
    type ConfigType = ValidationConfig;

    fn into_config(self) -> Result<Self::ConfigType, ConfigError> {

        let config = ValidationConfig {
            is_enable: self.enable,
            required_validation_layers: self.layers,

            debug_type: vk_raw2debug_instance_type(&self.instance_type)?,
            report_config: if let Some(config) = self.report_config { Some(config.into_config()?) } else { None },
            utils_config : if let Some(config) = self.utils_config  { Some(config.into_config()?) } else { None },
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

        if let Some(v) = toml.get("types") {
            self.instance_type = Some(v.as_str().ok_or(ConfigError::ParseError)?.to_owned());
        }

        if let Some(v) = toml.get("report") {
            let mut report_config = DebugReportConfigMirror::default();
            report_config.parse(v)?;
            self.report_config = Some(report_config);
        }

        if let Some(v) = toml.get("utils") {
            let mut utils_config = DebugUtilsConfigMirror::default();
            utils_config.parse(v)?;
            self.utils_config = Some(utils_config);
        }

        Ok(())
    }
}

fn vk_raw2debug_instance_type(raw: &Option<String>) -> Result<DebugInstanceType, ConfigError> {

    let r#type = if let Some(instance_type) = raw {
        match instance_type.as_str() {
            | "DebugReport" => DebugInstanceType::DebugReport,
            | "DebugUtils"  => DebugInstanceType::DebugUtils,
            | _ => return Err(ConfigError::Mapping(MappingError::DebugInstanceTypeError))
        }
    } else {
        DebugInstanceType::None
    };

    Ok(r#type)
}


#[derive(Deserialize, Default)]
pub(crate) struct DebugReportConfigMirror {

    flags: Vec<String>,
}

impl ConfigMirror for DebugReportConfigMirror {
    type ConfigType = DebugReportConfig;

    fn into_config(self) -> Result<Self::ConfigType, ConfigError> {

        let mut flags = vk::DebugReportFlagsEXT::empty();
        for raw_flag in self.flags.iter() {
            flags |= vk_raw2debug_report_flag(raw_flag)?;
        }

        let config = DebugReportConfig {
            flags,
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> Result<(), ConfigError> {

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

fn vk_raw2debug_report_flag(raw: &String) -> Result<vk::DebugReportFlagsEXT, ConfigError> {

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

#[derive(Deserialize, Default)]
pub(crate) struct DebugUtilsConfigMirror {

    flags    : Vec<String>,
    severity : Vec<String>,
    types    : Vec<String>,
}

impl ConfigMirror for DebugUtilsConfigMirror {
    type ConfigType = DebugUtilsConfig;

    fn into_config(self) -> Result<Self::ConfigType, ConfigError> {

        // vk::DebugUtilsMessengerCreateFlagsEXT is reserved for future use in API version 1.1.92.
        let flags = vk::DebugUtilsMessengerCreateFlagsEXT::empty();

        let mut severity = vk::DebugUtilsMessageSeverityFlagsEXT::empty();
        for raw_flag in self.severity.iter() {
            severity |= vk_raw2debug_utils_severity(raw_flag)?;
        }

        let mut types = vk::DebugUtilsMessageTypeFlagsEXT::empty();
        for raw_flag in self.types.iter() {
            types |= vk_raw2debug_utils_types(raw_flag)?;
        }

        let config = DebugUtilsConfig {
            flags, severity, types,
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> Result<(), ConfigError> {

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

        if let Some(v) = toml.get("severity") {
            if let Some(severities) = v.as_array() {
                if severities.len() > 0 {
                    self.severity.clear();

                    for severity in severities {
                        let value = severity.as_str().ok_or(ConfigError::ParseError)?;
                        self.severity.push(value.to_owned());
                    }
                }
            } else {
                return Err(ConfigError::ParseError);
            }
        }

        if let Some(v) = toml.get("types") {
            if let Some(types) = v.as_array() {
                if types.len() > 0 {
                    self.types.clear();

                    for r#type in types {
                        let value =r#type.as_str().ok_or(ConfigError::ParseError)?;
                        self.types.push(value.to_owned());
                    }
                }
            } else {
                return Err(ConfigError::ParseError);
            }
        }

        Ok(())
    }
}

fn vk_raw2debug_utils_severity(raw: &String) -> Result<vk::DebugUtilsMessageSeverityFlagsEXT, ConfigError> {

    let flag = match raw.as_str() {
        | "Verbose" => vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE,
        | "Warning" => vk::DebugUtilsMessageSeverityFlagsEXT::WARNING,
        | "Error"   => vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        | "Info"    => vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
        | _ => return Err(ConfigError::Mapping(MappingError::DebugUtilsError)),
    };

    Ok(flag)
}

fn vk_raw2debug_utils_types(raw: &String) -> Result<vk::DebugUtilsMessageTypeFlagsEXT, ConfigError> {

    let flag = match raw.as_str() {
        | "General"     => vk::DebugUtilsMessageTypeFlagsEXT::GENERAL,
        | "Performance" => vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        | "Validation"  => vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
        | _ => return Err(ConfigError::Mapping(MappingError::DebugUtilsError)),
    };

    Ok(flag)
}
