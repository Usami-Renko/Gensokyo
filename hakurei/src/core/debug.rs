
use ash::vk;
use ash::version::EntryV1_0;
use ash::extensions::DebugReport;

use core::EntryV1;
use core::error::InstanceError;
use core::instance::HaInstance;
use core::error::ValidationError;

use config::VERBOSE;
use config::core::VALIDATION_FLAGS;
use utility::cast;
use utility::marker::VulkanFlags;

use std::ptr;
use std::ffi::CStr;

/// a struct stores all need information during the initialization of Validation Layer.
pub struct ValidationInfo {
    // tell if validation layer should be enabled.
    pub is_enable: bool,
    // the layer names required for validation layer support.
    pub required_validation_layers: [&'static str; 1],
}

/// Wrapper class for vk::DebugReport object.
pub struct HaDebugger {

    /// the handle of vk::DebugReport object.
    loader: DebugReport,
    /// the handle of callback function used in Validation Layer.
    callback: vk::DebugReportCallbackEXT,
}

impl HaDebugger {

    /// initialize debug extension loader and vk::DebugReport object.
    pub fn setup(instance: &HaInstance) -> Result<HaDebugger, ValidationError> {

        // load the debug extension
        let loader = DebugReport::new(&instance.entry, &instance.handle)
            .or(Err(ValidationError::DebugReportCreationError))?;

        // configurate debug callback.
        let debug_callback_create_info = vk::DebugReportCallbackCreateInfoEXT {
            s_type       : vk::StructureType::DebugReportCallbackCreateInfoExt,
            p_next       : ptr::null(),
            // Enum DebugReportFlags enumerate all available flags.
            flags        : VALIDATION_FLAGS.flags(),
            pfn_callback : vulkan_debug_report_callback,
            p_user_data  : ptr::null_mut(),
        };

        let callback = unsafe {
            loader.create_debug_report_callback_ext(&debug_callback_create_info, None)
                .or(Err(ValidationError::DebugCallbackCreationError))?
        };

        let debugger = HaDebugger {
            loader,
            callback,
        };

        Ok(debugger)
    }

    /// Some cleaning operations before this object was uninitialized.
    ///
    /// For HaDebugger, it destroy the vk::DebugReport object.
    pub fn cleanup(&self) {
        unsafe {
            self.loader.destroy_debug_report_callback_ext(self.callback, None);
        }
    }
}

/// the message type that Validation Layer would report for.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DebugReportFlag {
    ErrorBit,
    InformationBit,
    DebugBit,
    WarningBit,
    PerformanceWarningBit,
}

impl VulkanFlags for [DebugReportFlag] {
    type FlagType = vk::DebugReportFlagsEXT;

    /// Convenient method to combine flags.
    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::DebugReportFlagsEXT::empty(), |acc, flag| {
            match *flag {
                | DebugReportFlag::ErrorBit              => acc | vk::DEBUG_REPORT_ERROR_BIT_EXT,
                | DebugReportFlag::InformationBit        => acc | vk::DEBUG_REPORT_INFORMATION_BIT_EXT,
                | DebugReportFlag::DebugBit              => acc | vk::DEBUG_REPORT_DEBUG_BIT_EXT,
                | DebugReportFlag::WarningBit            => acc | vk::DEBUG_REPORT_WARNING_BIT_EXT,
                | DebugReportFlag::PerformanceWarningBit => acc | vk::DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT,
            }
        })
    }
}

/// the callback function in Debug Report.
unsafe extern "system" fn vulkan_debug_report_callback(
    _flags        : vk::DebugReportFlagsEXT,
    _obj_type     : vk::DebugReportObjectTypeEXT,
    _obj          : vk::uint64_t,
    _location     : vk::size_t,
    _code         : vk::int32_t,
    _layer_prefix : *const vk::c_char,
    p_message     : *const vk::c_char,
    _user_data    : *mut vk::c_void
) -> u32 {

    println!("[Debug] {:?}", CStr::from_ptr(p_message));
    vk::VK_FALSE
}

/// helper function to check if all required layers of validation layer are satisfied.
pub fn is_support_validation_layer(entry: &EntryV1, required_validation_layers: &[&str]) -> Result<bool, InstanceError> {

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

    for &required_layer_name in required_validation_layers.iter() {
        let mut is_required_layer_found = false;

        for layer_property in layer_properties.iter() {

            let test_layer_name = cast::vk_to_string(&layer_property.layer_name);
            if required_layer_name == test_layer_name {
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


