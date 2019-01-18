
use ash::version::EntryV1_0;

use crate::core::instance::GsInstance;
use crate::core::debug::report::{ GsDebugReport, DebugReportConfig };
use crate::core::debug::utils::{ GsDebugUtils, DebugUtilsConfig };

use crate::VERBOSE;
use crate::utils::cast;

use crate::error::{ VkResult, VkError };

/// Wrapper class for the validation tools used in Vulkan.
///
/// Based on the content of `ValidationConfig`, `GsDebugger` will use `vk::DebugReport` or `vk::DebugUtils` in validation checking.
pub struct GsDebugger {

    target: Option<Box<dyn DebugInstance>>,
}

/// `DebugInstance` is used as a trait object. It specifies the common behaviors of a Vulkan validation tool.
pub(super) trait DebugInstance {

    /// Destroy this validation tool.
    fn destroy(&self);
}

/// An enum type indicates all support validation tools used in `GsDebugger`.
pub enum DebugInstanceType {

    DebugReport,
    DebugUtils,
    None,
}

pub struct ValidationConfig {

    /// `is_enable` tell if validation layer should be enabled.
    pub is_enable: bool,
    /// `required_validation_layers` is the layer names required for validation layer support.
    pub required_validation_layers: Vec<String>,
    /// `instance_type` is the type of debug tools to use(Debug Report or Debug Utils).
    pub debug_type: DebugInstanceType,
    /// `report_config` specifies the configuration paramaters used in Debug Report.
    pub report_config: Option<DebugReportConfig>,
    /// `utils_config` specifies the configuration paramaters used in Debug Utils.
    pub  utils_config: Option<DebugUtilsConfig>,
}

impl GsDebugger {

    /// Initialize the validation tool in Vulkan which is specified in `config`.
    pub fn new(instance: &GsInstance, config: &ValidationConfig) -> VkResult<GsDebugger> {

        let target = if config.is_enable == false {
            None
        } else {

            match config.debug_type {
                | DebugInstanceType::DebugReport => {
                    if let Some(ref report_config) = config.report_config {
                        let report = GsDebugReport::setup(instance, report_config)?;
                        Some(Box::new(report) as Box<DebugInstance>)
                    } else {
                        println!("The program require using DebugReport, but failed to obtain its configuration.");
                        None
                    }
                },
                | DebugInstanceType::DebugUtils => {
                    if let Some(ref utils_config) = config.utils_config {
                        let utils = GsDebugUtils::setup(instance, utils_config)?;
                        Some(Box::new(utils) as Box<DebugInstance>)
                    } else {
                        println!("The program require using DebugUtils, but failed to obtain its configuration.");
                        None
                    }
                },
                | _ => None,
            }
        };

        let debugger = GsDebugger { target };
        Ok(debugger)
    }

    /// Destroy this validation tool.
    pub fn destroy(&self) {

        if let Some(ref debug_instance) = self.target {
            debug_instance.destroy();
        }
    }
}

/// helper function to check if all required layers of validation layer are satisfied.
pub(in crate::core) fn is_support_validation_layer(entry: &ash::Entry, required_validation_layers: &[String]) -> VkResult<bool> {

    let layer_properties = entry.enumerate_instance_layer_properties()
        .or(Err(VkError::query("Layer Properties")))?;

    // Print the layer name to console in verbose mode.
    if VERBOSE {
        if layer_properties.len() == 0 {
            println!("[info] No available layers.");
        } else {

            println!("[info] Instance available layers:");
            for layer in layer_properties.iter() {
                let layer_name = cast::chars2string(&layer.layer_name);
                println!("\t{}", layer_name)
            }
        }
    }

    for required_layer_name in required_validation_layers.iter() {
        let mut is_required_layer_found = false;

        for layer_property in layer_properties.iter() {

            let test_layer_name = cast::chars2string(&layer_property.layer_name);
            if (*required_layer_name) == test_layer_name {
                is_required_layer_found = true;
                break
            }
        }

        if is_required_layer_found == false {
            return Ok(false)
        }
    }

    Ok(true)
}
