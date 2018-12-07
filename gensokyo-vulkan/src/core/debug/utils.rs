
use ash::vk;

use crate::core::instance::GsInstance;
use crate::core::debug::debugger::DebugInstance;
use crate::core::error::ValidationError;

use crate::types::{ vkbool, vkptr, VK_FALSE };

use std::ffi::CStr;
use std::ptr;

/// the callback function in Debug Utils.
unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity : vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type     : vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data  : *const vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data     : vkptr
) -> vkbool {

    let severity = debug_utils_severity_to_string(message_severity);
    let types = debug_utils_types_to_string(message_type);
    let message = CStr::from_ptr((*p_callback_data).p_message);
    println!("[Debug]{}{}{:?}", severity, types, message);

    VK_FALSE
}

/// Wrapper class for `vk::DebugUtils` object.
pub struct GsDebugUtils {

    /// the handle of `vk::DebugUtils` object.
    loader: ash::extensions::DebugUtils,
    /// the handle of callback function used in Validation Layer.
    utils_messager: vk::DebugUtilsMessengerEXT,
}

pub struct DebugUtilsConfig {

    pub flags    : vk::DebugUtilsMessengerCreateFlagsEXT,
    pub severity : vk::DebugUtilsMessageSeverityFlagsEXT,
    pub types    : vk::DebugUtilsMessageTypeFlagsEXT,
}

impl GsDebugUtils {

    /// Initialize debug report extension loader and `vk::DebugUtilsMessagerExt` object.
    pub fn setup(instance: &GsInstance, config: &DebugUtilsConfig) -> Result<GsDebugUtils, ValidationError> {

        let loader = ash::extensions::DebugUtils::new(&instance.entry, &instance.handle);

        let messager_create_info = vk::DebugUtilsMessengerCreateInfoEXT {
            s_type: vk::StructureType::DEBUG_UTILS_MESSENGER_CREATE_INFO_EXT,
            p_next: ptr::null(),
            flags            : config.flags,
            message_severity : config.severity,
            message_type     : config.types,
            pfn_user_callback: Some(vulkan_debug_utils_callback),
            p_user_data      : ptr::null_mut(),
        };

        let utils_messager = unsafe {
            loader.create_debug_utils_messenger_ext(&messager_create_info, None)
                .or(Err(ValidationError::DebugUtilsCallbackCreationEror))?
        };

        let utils = GsDebugUtils {
            loader, utils_messager,
        };

        Ok(utils)
    }
}

impl DebugInstance for GsDebugUtils {

    fn cleanup(&self) {
        unsafe {
            self.loader.destroy_debug_utils_messenger_ext(self.utils_messager, None);
        }
    }
}

fn debug_utils_severity_to_string(severity: vk::DebugUtilsMessageSeverityFlagsEXT) -> &'static str {
    match severity {
        | vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => "[Verbose]",
        | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => "[Warning]",
        | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR   => "[Error]",
        | vk::DebugUtilsMessageSeverityFlagsEXT::INFO    => "[Info]",
        | _ => "[Unknown]",
    }
}

fn debug_utils_types_to_string(types: vk::DebugUtilsMessageTypeFlagsEXT) -> &'static str {
    match types {
        | vk::DebugUtilsMessageTypeFlagsEXT::GENERAL     => "[General]",
        | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "[Performance]",
        | vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION  => "[Validation]",
        | _ => "[Unknown]",
    }
}
