
use ash::vk;
use ash::version::EntryV1_0;
use ash::extensions::DebugReport;

use core::error::InstanceError;
use core::instance::HaInstance;
use core::error::ValidationError;

use VERBOSE;
use utils::cast;

use types::{ vklint, vksint, vkchar, vkptr, VK_FALSE };

use std::ptr;
use std::ffi::CStr;

/// Wrapper class for `vk::DebugReport` object.
pub struct HaDebugger {

    /// the handle of `vk::DebugReport` object.
    loader: DebugReport,
    /// the handle of callback function used in Validation Layer.
    callback: vk::DebugReportCallbackEXT,
}

impl HaDebugger {

    /// Initialize debug extension loader and `vk::DebugReport` object.
    pub fn setup(instance: &HaInstance, config: &ValidationConfig) -> Result<HaDebugger, ValidationError> {

        // load the debug extension
        let loader = DebugReport::new(&instance.entry, &instance.handle);

        // configurate debug callback.
        let debug_callback_create_info = vk::DebugReportCallbackCreateInfoEXT {
            s_type      : vk::StructureType::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
            p_next      : ptr::null(),
            // Enum DebugReportFlags enumerate all available flags.
            flags       : config.flags,
            pfn_callback: if config.is_enable { Some(vulkan_debug_report_callback) } else { None },
            p_user_data : ptr::null_mut(),
        };

        let callback = unsafe {
            loader.create_debug_report_callback_ext(&debug_callback_create_info, None)
                .or(Err(ValidationError::DebugCallbackCreationError))?
        };

        let debugger = HaDebugger {
            loader, callback,
        };

        Ok(debugger)
    }

    /// Some cleaning operations before this object was uninitialized.
    ///
    /// For HaDebugger, it destroy the `vk::DebugReport` object.
    pub fn cleanup(&self) {
        unsafe {
            self.loader.destroy_debug_report_callback_ext(self.callback, None);
        }
    }
}

/// the callback function in Debug Report.
unsafe extern "system" fn vulkan_debug_report_callback(
    _flags       : vk::DebugReportFlagsEXT,
    _obj_type    : vk::DebugReportObjectTypeEXT,
    _obj         : vklint,
    _location    : usize,
    _code        : vksint,
    _layer_prefix: *const vkchar,
    p_message    : *const vkchar,
    _user_data   : vkptr
) -> u32 {

    println!("[Debug] {:?}", CStr::from_ptr(p_message));
    VK_FALSE
}

pub struct ValidationConfig {
    /// tell if validation layer should be enabled.
    pub is_enable: bool,
    /// the layer names required for validation layer support.
    pub required_validation_layers: Vec<String>,
    /// the message type that Validation Layer would report for.
    pub flags: vk::DebugReportFlagsEXT,
}

/// helper function to check if all required layers of validation layer are satisfied.
pub(super) fn is_support_validation_layer(entry: &ash::Entry, required_validation_layers: &[String]) -> Result<bool, InstanceError> {

    let layer_properties = entry.enumerate_instance_layer_properties()
        .or(Err(InstanceError::LayerPropertiesEnumerateError))?;

    // Print the layer name to console in verbose mode.
    if VERBOSE {
        if layer_properties.len() == 0 {
            println!("[info] No available layers.");
        } else {

            println!("[info] Instance available layers:");
            for layer in layer_properties.iter() {
                let layer_name = cast::vk_to_string(&layer.layer_name);
                println!("\t{}", layer_name)
            }
        }
    }

    for required_layer_name in required_validation_layers.iter() {
        let mut is_required_layer_found = false;

        for layer_property in layer_properties.iter() {

            let test_layer_name = cast::vk_to_string(&layer_property.layer_name);
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

