
use ash::vk;
use ash::version::EntryV1_0;
use ash::version::InstanceV1_0;

use crate::core::debug;
use crate::core::debug::{ ValidationConfig, DebugInstanceType };
use crate::core::platforms;

use crate::utils::cast;
use crate::types::vkuint;

use crate::error::{ VkResult, VkErrorKind, VkError };

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

/// An enum type indicates all support extensions for `vk::Instance`.
enum InstanceExtensionType {
    Surface,
    PlatformSurface,
    DebugReport,
    DebugUtils,
}

impl GsInstance {

    /// Initialize `vk::Instance` object.
    pub fn new(config: &InstanceConfig, validation: &ValidationConfig) -> VkResult<GsInstance> {

        let entry = ash::Entry::new()
            .or(Err(VkErrorKind::Unlink(String::from("Entry"))))?;

        let app_name = cast::string2cstring((&config.application_name).into())
            .ok_or(VkError::str_convert("Vulkan Application Name"))?;
        let engine_name = cast::string2cstring((&config.engine_name).into())
            .ok_or(VkError::str_convert("Vulkan Engine Name"))?;

        let application_info = vk::ApplicationInfo {
            s_type              : vk::StructureType::APPLICATION_INFO,
            p_next              : ptr::null(),
            p_application_name  : app_name.as_ptr(),
            application_version : config.application_version,
            p_engine_name       : engine_name.as_ptr(),
            engine_version      : config.engine_version,
            api_version         : config.api_version,
        };

        // get the names of required vulkan layers.
        let enable_layer_names = required_layers(&entry, validation)?;
        let enable_layer_names_ptr = cast::cstrings2ptrs(&enable_layer_names);
        // get the names of required vulkan extensions.
        let require_extensions = GsInstance::require_extensions(validation);
        let enable_extension_names = instance_extensions_to_names(&require_extensions);

        let instance_create_info = vk::InstanceCreateInfo {
            s_type                     : vk::StructureType::INSTANCE_CREATE_INFO,
            p_next                     : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags                      : vk::InstanceCreateFlags::empty(),
            p_application_info         : &application_info,
            enabled_layer_count        : enable_layer_names_ptr.len() as _,
            pp_enabled_layer_names     : enable_layer_names_ptr.as_ptr(),
            enabled_extension_count    : enable_extension_names.len() as _,
            pp_enabled_extension_names : enable_extension_names.as_ptr(),
        };

        // create vk::Instance object.
        let handle = unsafe {
            entry.create_instance(&instance_create_info, None)
                .map_err(|e| VkError::unlink(format!("Instance({})", e)))?
        };

        let instance = GsInstance {
            entry, handle, enable_layer_names,
        };

        Ok(instance)
    }

    /// Specify the necessary extensions.
    fn require_extensions(validation: &ValidationConfig) -> Vec<InstanceExtensionType> {

        let mut instance_extensions = vec![
            InstanceExtensionType::Surface,
            InstanceExtensionType::PlatformSurface,
        ];

        match validation.debug_type {
            | DebugInstanceType::DebugReport =>
                instance_extensions.push(InstanceExtensionType::DebugReport),
            | DebugInstanceType::DebugUtils =>
                instance_extensions.push(InstanceExtensionType::DebugUtils),
            | DebugInstanceType::None => {},
        }

        instance_extensions
    }

    /// Destroy the `vk::Instance` object. This function must be called before this wrapper class is dropped.
    ///
    /// In Vulkan, all child objects created using instance must have been destroyed prior to destroying instance.
    pub fn destroy(&self) {
        unsafe {
            self.handle.destroy_instance(None);
        }
    }
}

/// Convenient function to get the names of required vulkan layers.
///
/// Return an vector of CString if succeeds, or an error explan the detail.
fn required_layers(entry: &ash::Entry, validation: &ValidationConfig) -> VkResult<Vec<CString>> {

    // required validation layer name if need  ---------------------------
    let mut enable_layer_names = vec![];

    if validation.is_enable {
        if debug::is_support_validation_layer(entry, &validation.required_validation_layers)? {

            for layer in validation.required_validation_layers.iter() {

                let layer_name = cast::string2cstring(Some(layer))
                    .ok_or(VkError::str_convert("Vulkan Layers Name"))?;
                enable_layer_names.push(layer_name);
            }
        } else {
            return Err(VkError::unsupported("Validation Layer"))
        }
    }
    // -------------------------------------------------------------------

    // required other layers ---------------------------------------------
    // currently not ohter layers is needed
    // -------------------------------------------------------------------

    Ok(enable_layer_names)
}

/// Translate the type of instance extension to c-style string.
fn instance_extensions_to_names(extensions: &[InstanceExtensionType]) -> Vec<*const i8> {

    extensions.iter().map(|extension| {
        match extension {
            | InstanceExtensionType::Surface         => ash::extensions::khr::Surface::name().as_ptr(),
            | InstanceExtensionType::PlatformSurface => platforms::platform_surface_names().as_ptr(),
            | InstanceExtensionType::DebugReport     => ash::extensions::ext::DebugReport::name().as_ptr(),
            | InstanceExtensionType::DebugUtils      => ash::extensions::ext::DebugUtils::name().as_ptr(),
        }
    }).collect()
}

/// The configuration parameters used in the initialization of `vk::Instance`.
pub struct InstanceConfig {

    /// `api_version` must be the highest version of Vulkan that the application is designed to use.
    ///
    /// The patch version number is ignored and only the major and minor versions must match those requested in `api_version`.
    pub api_version: vkuint,
    /// `application_version` is an unsigned integer variable containing the developer-supplied version number of the application.
    pub application_version: vkuint,
    /// `engine_version`is an unsigned integer variable containing the developer-supplied version number of the engine used to create the application.
    pub engine_version: vkuint,

    /// `application_name` is a string containing the name of the application or None if it is not provided.
    pub application_name: Option<String>,
    /// `engine_name` is the name of the engine used to create the application or None if it is not provided.
    pub engine_name: Option<String>,
}
