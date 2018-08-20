
use ash;
use ash::vk;
use ash::version::{ V1_0, EntryV1_0, InstanceV1_0 };

pub type EntryV1    = ash::Entry<V1_0>;
pub type InstanceV1 = ash::Instance<V1_0>;

use core::error::InstanceError;
use core::platforms;
use core::debug;

use constant::core::*;
use constant::VERBOSE;

use utility::cast;

use std::ptr;
use std::ffi::CString;

pub struct Instance {

    pub entry:  EntryV1,
    pub handle: InstanceV1,

    pub enable_layer_names: Vec<CString>,
}

impl Instance {

    pub fn new() -> Result<Instance, InstanceError> {

        let entry = EntryV1::new()
            .or(Err(InstanceError::EntryCreationError))?;

        let app_name    = CString::new(APPLICATION_NAME).unwrap();
        let engine_name = CString::new(ENGINE_NAME).unwrap();

        let app_info = vk::ApplicationInfo {
            s_type              : vk::StructureType::ApplicationInfo,
            p_next              : ptr::null(),
            p_application_name  : app_name.as_ptr(),
            application_version : APPLICATION_VERSION,
            p_engine_name       : engine_name.as_ptr(),
            engine_version      : ENGINE_VERSION,
            api_version         : API_VERSION,
        };

        let enable_layer_names = required_layers(&entry)?;
        let enable_layer_names_ptr = cast::to_array_ptr(&enable_layer_names);
        let enable_extension_names = platforms::required_extension_names();

        let instance_create_info = vk::InstanceCreateInfo {
            s_type                     : vk::StructureType::InstanceCreateInfo,
            p_next                     : ptr::null(),
            // flags is reserved for future use in API version 1.0.82
            flags                      : vk::InstanceCreateFlags::empty(),
            p_application_info         : &app_info,
            enabled_layer_count        : enable_layer_names_ptr.len() as u32,
            pp_enabled_layer_names     : enable_layer_names_ptr.as_ptr(),
            enabled_extension_count    : enable_extension_names.len() as u32,
            pp_enabled_extension_names : enable_extension_names.as_ptr(),
        };

        let instance_handle = unsafe {
            entry.create_instance(&instance_create_info, None)
                .or(Err(InstanceError::InstanceCreationError))?
        };

        let instance = Instance {
            entry,
            handle: instance_handle,

            enable_layer_names,
        };

        Ok(instance)
    }

    pub fn clenaup(&self) {
        unsafe {
            self.handle.destroy_instance(None);
            if VERBOSE {
                println!("[Info] Vulkan Instance had been destroy.");
            }
        }
    }
}

fn required_layers(entry: &EntryV1) -> Result<Vec<CString>, InstanceError> {

    // required validation layer name if need  ---------------------------
    let mut enable_layer_names = vec![];

    if VALIDATION.is_enable {
        if debug::is_support_validation_layer(entry, &VALIDATION.required_validation_layers)? {
            enable_layer_names = VALIDATION.required_validation_layers.iter()
                .map(|layer_name| CString::new(*layer_name).unwrap())
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
