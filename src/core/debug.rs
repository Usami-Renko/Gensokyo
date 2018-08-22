
use ash::vk;
use ash::version::EntryV1_0;
use ash::extensions::DebugReport;

use core::EntryV1;
use core::error::InstanceError;
use core::instance::Instance;
use core::error::ValidationError;

use constant::VERBOSE;
use constant::core::VALIDATION_FLAGS;
use utility::cast;
use utility::marker::VulkanFlags;

use std::ptr;
use std::ffi::CStr;

pub struct ValidationInfo {
    pub is_enable: bool,
    pub required_validation_layers: [&'static str; 1],
}

pub struct Debugger {

    loader: DebugReport,
    callback: vk::DebugReportCallbackEXT,
}

impl Debugger {

    pub fn setup(instance: &Instance) -> Result<Debugger, ValidationError> {

        let loader = DebugReport::new(&instance.entry, &instance.handle)
            .or(Err(ValidationError::DebugReportCreationError))?;

        let debug_callback_create_info = vk::DebugReportCallbackCreateInfoEXT {
            s_type       : vk::StructureType::DebugReportCallbackCreateInfoExt,
            p_next       : ptr::null(),
            // Enum DebugReportFlags enumerate all available flags
            flags        : VALIDATION_FLAGS.flags(),
            pfn_callback : vulkan_debug_report_callback,
            p_user_data  : ptr::null_mut(),
        };

        let callback = unsafe {
            loader.create_debug_report_callback_ext(&debug_callback_create_info, None)
                .or(Err(ValidationError::DebugCallbackCreationError))?
        };

        let debugger = Debugger {
            loader,
            callback,
        };

        Ok(debugger)
    }

    pub fn cleanup(&self) {
        unsafe {
            self.loader.destroy_debug_report_callback_ext(self.callback, None);

            if VERBOSE {
                println!("[info] DebugReport Callback had been destroy.")
            }
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DebugReportFlags {
    ErrorBit,
    InformationBit,
    DebugBit,
    WarningBit,
    PerformanceWarningBit,
}

impl VulkanFlags for [DebugReportFlags] {
    type FlagType = vk::DebugReportFlagsEXT;

    fn flags(&self) -> Self::FlagType {
        self.iter().fold(vk::DebugReportFlagsEXT::empty(), |acc, flag| {
            match *flag {
                | DebugReportFlags::ErrorBit              => acc | vk::DEBUG_REPORT_ERROR_BIT_EXT,
                | DebugReportFlags::InformationBit        => acc | vk::DEBUG_REPORT_INFORMATION_BIT_EXT,
                | DebugReportFlags::DebugBit              => acc | vk::DEBUG_REPORT_DEBUG_BIT_EXT,
                | DebugReportFlags::WarningBit            => acc | vk::DEBUG_REPORT_WARNING_BIT_EXT,
                | DebugReportFlags::PerformanceWarningBit => acc | vk::DEBUG_REPORT_PERFORMANCE_WARNING_BIT_EXT,
            }
        })
    }
}

unsafe extern "system" fn vulkan_debug_report_callback(
    _flags        : vk::DebugReportFlagsEXT,
    _obj_type     : vk::DebugReportObjectTypeEXT,
    _obj          : vk::uint64_t,
    _location     : vk::size_t,
    _code         : vk::int32_t,
    _layer_prefix : *const vk::c_char,
    p_message    : *const vk::c_char,
    _user_data    : *mut vk::c_void
) -> u32 {

    println!("[Debug] {:?}", CStr::from_ptr(p_message));
    vk::VK_FALSE
}

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


