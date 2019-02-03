
use toml;
use serde_derive::Deserialize;

use ash::vk;

use crate::config::engine::ConfigMirror;
use crate::error::{ GsResult, GsError };

use gsvk::core::debug::{ ValidationConfig, DebugReportConfig, DebugUtilsConfig, DebugInstanceType };

#[derive(Deserialize)]
pub(crate) struct ValidationConfigMirror {

    enable: bool,
    layers: Vec<String>,

    instance_type: Option<String>,
    report_config: Option<DebugReportConfigMirror>,
    utils_config : Option<DebugUtilsConfigMirror>,

    print_instance_layers: bool, // default is false.
}

impl Default for ValidationConfigMirror {

    fn default() -> ValidationConfigMirror {
        ValidationConfigMirror {
            enable: true,
            layers: vec![
                String::from("VK_LAYER_LUNARG_standard_validation"),
            ],

            instance_type: Some(String::from("DebugUtils")),
            report_config: Some(DebugReportConfigMirror::default()),
            utils_config : Some(DebugUtilsConfigMirror::default()),

            print_instance_layers: false,
        }
    }
}

impl ConfigMirror for ValidationConfigMirror {
    type ConfigType = ValidationConfig;

    fn into_config(self) -> GsResult<Self::ConfigType> {

        let config = ValidationConfig {
            is_enable: self.enable,
            required_validation_layers: self.layers,

            debug_type: vk_raw2debug_instance_type(&self.instance_type)?,
            report_config: if let Some(config) = self.report_config { Some(config.into_config()?) } else { None },
            utils_config : if let Some(config) = self.utils_config  { Some(config.into_config()?) } else { None },

            print_instance_layers: self.print_instance_layers,
        };

        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> GsResult<()> {

        if let Some(v) = toml.get("enable") {
            self.enable = v.as_bool()
                .ok_or(GsError::config("core.validation.enable"))?;
        }

        if let Some(v) = toml.get("layers") {
            if let Some(layers) = v.as_array() {
                if layers.len() > 0 {
                    self.layers.clear();

                    for (i, layer) in layers.iter().enumerate() {
                        let value = layer.as_str()
                            .ok_or(GsError::config(format!("layers #{}", i)))?;
                        self.layers.push(value.to_owned());
                    }
                }
            } else {
                return Err(GsError::config("core.validation.layers"))
            }
        }

        if let Some(v) = toml.get("types") {
            let instance_type = v.as_str()
                .ok_or(GsError::config("core.validation.types"))?.to_owned();
            self.instance_type = Some(instance_type);
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

        if let Some(v) = toml.get("print_instance_layers") {
            self.print_instance_layers = v.as_bool()
                .ok_or(GsError::config("core.validation.print_instance_layers"))?.to_owned();
        }

        Ok(())
    }
}

fn vk_raw2debug_instance_type(raw: &Option<String>) -> GsResult<DebugInstanceType> {

    let r#type = if let Some(instance_type) = raw {
        match instance_type.as_str() {
            | "DebugReport" => DebugInstanceType::DebugReport,
            | "DebugUtils"  => DebugInstanceType::DebugUtils,
            | _ => return Err(GsError::config(instance_type)),
        }
    } else {
        DebugInstanceType::None
    };

    Ok(r#type)
}


#[derive(Deserialize)]
pub(crate) struct DebugReportConfigMirror {

    flags: Vec<String>,
}

impl Default for DebugReportConfigMirror {

    fn default() -> DebugReportConfigMirror {
        DebugReportConfigMirror {
            flags: vec![
                String::from("Error"),
                String::from("Warning"),
                String::from("PerformanceWarning"),
            ],
        }
    }
}

impl ConfigMirror for DebugReportConfigMirror {
    type ConfigType = DebugReportConfig;

    fn into_config(self) -> GsResult<Self::ConfigType> {

        let mut flags = vk::DebugReportFlagsEXT::empty();
        for raw_flag in self.flags.iter() {
            flags |= vk_raw2debug_report_flag(raw_flag)?;
        }

        let config = DebugReportConfig { flags };
        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> GsResult<()> {

        if let Some(v) = toml.get("flags") {
            if let Some(flags) = v.as_array() {
                if flags.len() > 0 {
                    self.flags.clear();

                    for (i, flag) in flags.iter().enumerate() {
                        let value = flag.as_str()
                            .ok_or(GsError::config(format!("flags #{}", i)))?;
                        self.flags.push(value.to_owned());
                    }
                }
            } else {
                return Err(GsError::config("core.validation.report.flags"))
            }
        }

        Ok(())
    }
}

fn vk_raw2debug_report_flag(raw: &String) -> GsResult<vk::DebugReportFlagsEXT> {

    let flag = match raw.as_str() {
        | "Error"              => vk::DebugReportFlagsEXT::ERROR,
        | "Warning"            => vk::DebugReportFlagsEXT::WARNING,
        | "PerformanceWarning" => vk::DebugReportFlagsEXT::PERFORMANCE_WARNING,
        | "Debug"              => vk::DebugReportFlagsEXT::DEBUG,
        | "Information"        => vk::DebugReportFlagsEXT::INFORMATION,
        | _ => return Err(GsError::config(raw)),
    };

    Ok(flag)
}

#[derive(Deserialize)]
pub(crate) struct DebugUtilsConfigMirror {

    flags    : Vec<String>,
    severity : Vec<String>,
    types    : Vec<String>,
}

impl Default for DebugUtilsConfigMirror {

    fn default() -> DebugUtilsConfigMirror {
        DebugUtilsConfigMirror {
            flags: Vec::new(),
            severity: vec![
                String::from("Warning"),
                String::from("Error"),
            ],
            types: vec![
                String::from("General"),
                String::from("Performance"),
                String::from("Validation"),
            ],
        }
    }
}

impl ConfigMirror for DebugUtilsConfigMirror {
    type ConfigType = DebugUtilsConfig;

    fn into_config(self) -> GsResult<Self::ConfigType> {

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

        let config = DebugUtilsConfig { flags, severity, types };
        Ok(config)
    }

    fn parse(&mut self, toml: &toml::Value) -> GsResult<()> {

        if let Some(v) = toml.get("flags") {
            if let Some(flags) = v.as_array() {
                if flags.len() > 0 {
                    self.flags.clear();

                    for (i, flag) in flags.iter().enumerate() {
                        let value = flag.as_str()
                            .ok_or(GsError::config(format!("flags #{}", i)))?;
                        self.flags.push(value.to_owned());
                    }
                }
            } else {
                return Err(GsError::config("core.validation.utils.flags"))
            }
        }

        if let Some(v) = toml.get("severity") {
            if let Some(severities) = v.as_array() {
                if severities.len() > 0 {
                    self.severity.clear();

                    for (i, severity) in severities.iter().enumerate() {
                        let value = severity.as_str()
                            .ok_or(GsError::config(format!("severity #{}", i)))?;
                        self.severity.push(value.to_owned());
                    }
                }
            } else {
                return Err(GsError::config("core.validation.utils.severity"))
            }
        }

        if let Some(v) = toml.get("types") {
            if let Some(types) = v.as_array() {
                if types.len() > 0 {
                    self.types.clear();

                    for (i, r#type) in types.iter().enumerate() {
                        let value =r#type.as_str()
                            .ok_or(GsError::config(format!("types #{}", i)))?;
                        self.types.push(value.to_owned());
                    }
                }
            } else {
                return Err(GsError::config("core.validation.utils.types"))
            }
        }

        Ok(())
    }
}

fn vk_raw2debug_utils_severity(raw: &String) -> GsResult<vk::DebugUtilsMessageSeverityFlagsEXT> {

    let flag = match raw.as_str() {
        | "Verbose" => vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE,
        | "Warning" => vk::DebugUtilsMessageSeverityFlagsEXT::WARNING,
        | "Error"   => vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        | "Info"    => vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
        | _ => return Err(GsError::config(raw)),
    };

    Ok(flag)
}

fn vk_raw2debug_utils_types(raw: &String) -> GsResult<vk::DebugUtilsMessageTypeFlagsEXT> {

    let flag = match raw.as_str() {
        | "General"     => vk::DebugUtilsMessageTypeFlagsEXT::GENERAL,
        | "Performance" => vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
        | "Validation"  => vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
        | _ => return Err(GsError::config(raw)),
    };

    Ok(flag)
}
