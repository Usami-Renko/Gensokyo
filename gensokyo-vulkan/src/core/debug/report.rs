
use ash::vk;
use ash::extensions::DebugReport;

use core::instance::GsInstance;
use core::debug::debugger::DebugInstance;
use core::error::ValidationError;

use types::{ vkptr, vkchar, vklint, vksint, VK_FALSE };

use std::ffi::CStr;
use std::ptr;

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

/// Wrapper class for `vk::DebugReport` object.
pub struct GsDebugReport {

    /// the handle of `vk::DebugReport` object.
    loader: DebugReport,
    /// the handle of callback function used in Validation Layer.
    callback: vk::DebugReportCallbackEXT,
}

pub struct DebugReportConfig {
    /// the message type that Validation Layer would report for.
    pub flags: vk::DebugReportFlagsEXT,
}

impl GsDebugReport {

    /// Initialize debug extension loader and `vk::DebugReport` object.
    pub fn setup(instance: &GsInstance, config: &DebugReportConfig) -> Result<GsDebugReport, ValidationError> {

        // load the debug extension.
        let loader = DebugReport::new(&instance.entry, &instance.handle);

        // configurate debug callback.
        let debug_callback_create_info = vk::DebugReportCallbackCreateInfoEXT {
            s_type      : vk::StructureType::DEBUG_REPORT_CALLBACK_CREATE_INFO_EXT,
            p_next      : ptr::null(),
            // Enum DebugReportFlags enumerate all available flags.
            flags       : config.flags,
            pfn_callback: Some(vulkan_debug_report_callback),
            p_user_data : ptr::null_mut(),
        };

        let callback = unsafe {
            loader.create_debug_report_callback_ext(&debug_callback_create_info, None)
                .or(Err(ValidationError::DebugReportCallbackCreationError))?
        };

        let report = GsDebugReport {
            loader, callback,
        };

        Ok(report)
    }
}

impl DebugInstance for GsDebugReport {

    /// Some cleaning operations before this object was uninitialized.
    ///
    /// For GsDebugReport, it destroy the `vk::DebugReport` object.
    fn cleanup(&self) {
        unsafe {
            self.loader.destroy_debug_report_callback_ext(self.callback, None);
        }
    }
}
