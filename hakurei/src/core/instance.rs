
use ash::vk;
use ash::version::{ EntryV1_0, InstanceV1_0 };

use core::{ EntryV1, InstanceV1 };

use config::core;
use config::engine::EngineConfig;
use core::error::InstanceError;
use core::debug::ValidationInfo;
use core::platforms;
use core::debug;

use utility::cast;

use std::ptr;
use std::ffi::CString;

/// Wrapper class for `vk::Instance` object.
pub struct HaInstance {

    /// the object used in instance creation define in ash crate.
    pub entry:  EntryV1,
    /// handle of `vk::Instance`.
    pub handle: InstanceV1,
    /// an array to store the names of vulkan layers enabled in instance creation.
    pub enable_layer_names: Vec<CString>,
}

impl HaInstance {

    /// Initialize `vk::Instance` object
    pub fn new(config: &EngineConfig) -> Result<HaInstance, InstanceError> {

        let entry = EntryV1::new()
            .or(Err(InstanceError::EntryCreationError))?;

        let app_name    = CString::new(core::APPLICATION_NAME).unwrap();
        let engine_name = CString::new(core::ENGINE_NAME).unwrap();

        let app_info = vk::ApplicationInfo {
            s_type              : vk::StructureType::ApplicationInfo,
            p_next              : ptr::null(),
            p_application_name  : app_name.as_ptr(),
            application_version : core::APPLICATION_VERSION,
            p_engine_name       : engine_name.as_ptr(),
            engine_version      : core::ENGINE_VERSION,
            api_version         : core::API_VERSION,
        };

        // get the names of required vulkan layers.
        let enable_layer_names = required_layers(&entry, &config.core.validation)?;
        let enable_layer_names_ptr = cast::to_array_ptr(&enable_layer_names);
        // get the names of required vulkan extensions.
        let enable_extension_names = platforms::required_extension_names();

        let instance_create_info = vk::InstanceCreateInfo {
            s_type                     : vk::StructureType::InstanceCreateInfo,
            p_next                     : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags                      : vk::InstanceCreateFlags::empty(),
            p_application_info         : &app_info,
            enabled_layer_count        : enable_layer_names_ptr.len() as u32,
            pp_enabled_layer_names     : enable_layer_names_ptr.as_ptr(),
            enabled_extension_count    : enable_extension_names.len() as u32,
            pp_enabled_extension_names : enable_extension_names.as_ptr(),
        };

        // create vk::Instance object.
        let handle = unsafe {
            entry.create_instance(&instance_create_info, None)
                .or(Err(InstanceError::InstanceCreationError))?
        };

        let instance = HaInstance {
            entry,
            handle,

            enable_layer_names,
        };

        Ok(instance)
    }

    /// Some cleaning operations before this object was uninitialized.
    ///
    /// For HaInstance, it destroy the `vk::Instance` object.
    pub fn clenaup(&self) {
        unsafe {
            self.handle.destroy_instance(None);
        }
    }
}

/// Convenient function to get the names of required vulkan layers.
///
/// Return an vector of CString if succeeds, or an error explan the detail.
fn required_layers(entry: &EntryV1, validation: &ValidationInfo) -> Result<Vec<CString>, InstanceError> {

    // required validation layer name if need  ---------------------------
    let mut enable_layer_names = vec![];

    if validation.is_enable {
        if debug::is_support_validation_layer(entry, &validation.required_validation_layers)? {
            enable_layer_names = validation.required_validation_layers.iter()
                .map(|layer_name| CString::new(layer_name.as_str()).unwrap())
                .collect();
        } else {
            return Err(InstanceError::ValidationLayerNotSupportError)

        }
    }
    // -------------------------------------------------------------------

    // required other layers ---------------------------------------------
    // currently not ohter layers is needed
    // -------------------------------------------------------------------

//    let raw_names = enable_layer_names.iter().map

    Ok(enable_layer_names)
}
