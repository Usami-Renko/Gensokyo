
use ash::vk;
use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;

use core::debug::ValidationConfig;
use core::error::InstanceError;
use core::platforms;
use core::debug;

use utils::cast;
use types::vkuint;

use std::ptr;
use std::ffi::CString;

/// Wrapper class for `vk::Instance` object.
pub struct GsInstance {

    /// handle of `vk::Instance`.
    pub(crate) handle: ash::Instance,
    /// the object used in instance creation define in ash crate.
    pub(crate) entry: ash::Entry,
    /// an array to store the names of vulkan layers enabled in instance creation.
    pub(crate) enable_layer_names: Vec<CString>,
}

impl GsInstance {

    /// Initialize `vk::Instance` object
    pub fn new(config: &InstanceConfig, validation: &ValidationConfig) -> Result<GsInstance, InstanceError> {

        let entry = ash::Entry::new()
            .or(Err(InstanceError::EntryCreationError))?;

        let app_name    = CString::new(config.name_application.clone()).unwrap();
        let engine_name = CString::new(config.name_engine.clone()).unwrap();

        let application_info = vk::ApplicationInfo {
            s_type              : vk::StructureType::APPLICATION_INFO,
            p_next              : ptr::null(),
            p_application_name  : app_name.as_ptr(),
            application_version : config.version_application,
            p_engine_name       : engine_name.as_ptr(),
            engine_version      : config.version_engine,
            api_version         : config.version_api,
        };

        // get the names of required vulkan layers.
        let enable_layer_names = required_layers(&entry, validation)?;
        let enable_layer_names_ptr = cast::to_array_ptr(&enable_layer_names);
        // get the names of required vulkan extensions.
        let enable_extension_names = platforms::required_extension_names();

        let instance_create_info = vk::InstanceCreateInfo {
            s_type                     : vk::StructureType::INSTANCE_CREATE_INFO,
            p_next                     : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags                      : vk::InstanceCreateFlags::empty(),
            p_application_info         : &application_info,
            enabled_layer_count        : enable_layer_names_ptr.len() as vkuint,
            pp_enabled_layer_names     : enable_layer_names_ptr.as_ptr(),
            enabled_extension_count    : enable_extension_names.len() as vkuint,
            pp_enabled_extension_names : enable_extension_names.as_ptr(),
        };

        // create vk::Instance object.
        let handle = unsafe {
            entry.create_instance(&instance_create_info, None)
                .or(Err(InstanceError::InstanceCreationError))?
        };

        let instance = GsInstance {
            entry, handle, enable_layer_names,
        };

        Ok(instance)
    }

    /// Some cleaning operations before this object was uninitialized.
    ///
    /// For `GsInstance`, it destroy the `vk::Instance` object.
    pub fn clenaup(&self) {
        unsafe {
            self.handle.destroy_instance(None);
        }
    }
}

/// Convenient function to get the names of required vulkan layers.
///
/// Return an vector of CString if succeeds, or an error explan the detail.
fn required_layers(entry: &ash::Entry, validation: &ValidationConfig) -> Result<Vec<CString>, InstanceError> {

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

pub struct InstanceConfig {

    pub version_api         : vkuint,
    pub version_application : vkuint,
    pub version_engine      : vkuint,

    pub name_application : String,
    pub name_engine      : String,
}
