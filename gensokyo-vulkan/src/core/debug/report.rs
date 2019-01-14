
use ash::vk;

use crate::core::instance::GsInstance;
use crate::core::debug::debugger::DebugInstance;
use crate::core::error::ValidationError;

use crate::types::{ vkptr, vkchar, vklint, vksint, VK_FALSE };

use std::ffi::CStr;
use std::ptr;

/// the callback function used in Debug Report.
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

/// Wrapper class for `vk::DebugReport` object.
pub struct GsDebugReport {

    /// the handle of `vk::DebugReport` object.
    loader: ash::extensions::ext::DebugReport,
    /// the handle of callback function used in Validation Layer.
    callback: vk::DebugReportCallbackEXT,
}

/// The configuration parameters used in the initialization of `vk::DebugReport`.
pub struct DebugReportConfig {
    /// the message type that Validation Layer would report for.
    pub flags: vk::DebugReportFlagsEXT,
}

impl GsDebugReport {

    /// Initialize debug extension loader and `vk::DebugReport` object.
    pub fn setup(instance: &GsInstance, config: &DebugReportConfig) -> Result<GsDebugReport, ValidationError> {

        // load the debug extension.
        let loader = ash::extensions::ext::DebugReport::new(&instance.entry, &instance.handle);

        // configure debug callback.
        let debug_callback_create_info = vk::DebugReportCallbackCreateInfoEXT {
            s_type      : vk::StructureType::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
            p_next      : ptr::null(),
            // Enum DebugReportFlags enumerate all available flags.
            flags       : config.flags,
            pfn_callback: Some(vulkan_debug_report_callback),
            p_user_data : ptr::null_mut(),
        };

        let callback = unsafe {
            loader.create_debug_report_callback(&debug_callback_create_info, None)
                .or(Err(ValidationError::DebugReportCallbackCreationError))?
        };

        let report = GsDebugReport {
            loader, callback,
        };

        Ok(report)
    }
}

impl DebugInstance for GsDebugReport {

    /// Destroy the `vk::DebugReport` object.
    fn destroy(&self) {
        unsafe {
            self.loader.destroy_debug_report_callback(self.callback, None);
        }
    }
}
