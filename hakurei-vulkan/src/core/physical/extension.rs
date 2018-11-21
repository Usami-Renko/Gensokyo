
use ash::vk;
use ash::version::InstanceV1_0;

use core::instance::HaInstance;
use core::physical::config::PhysicalInspectProperty;
use core::physical::DeviceExtensionType;
use core::error::PhysicalDeviceError;

use utils::cast;

use std::ffi::CString;

pub(crate) struct PhysicalExtension {

    handles: Vec<vk::ExtensionProperties>,
    enable_extensions: Vec<DeviceExtensionType>,
}

#[derive(Debug, Clone)]
pub struct PhysicalExtensionConfig {

    require_extensions: Vec<DeviceExtensionType>,
}

impl PhysicalExtension {

    pub fn query(instance: &HaInstance, physical_device: vk::PhysicalDevice) -> Result<PhysicalExtension, PhysicalDeviceError> {

        let handles = unsafe {
            instance.handle.enumerate_device_extension_properties(physical_device)
                .or(Err(PhysicalDeviceError::EnumerateExtensionsError))?
        };

        let result = PhysicalExtension {
            handles,
            enable_extensions: Vec::new(),
        };

        Ok(result)
    }

    pub fn enable_extensions(&self) -> Vec<CString> {

        self.enable_extensions.iter()
            .map(|e| e.name()).collect()
    }
}

impl PhysicalInspectProperty for PhysicalExtension {
    type ConfigType = PhysicalExtensionConfig;

    fn inspect(&self, config: &Self::ConfigType) -> bool {

        if config.require_extensions.is_empty() { return true }

        let requrie_extension_names: Vec<CString> = config.require_extensions.iter()
            .map(|e| e.name()).collect();
        let available_extensions: Vec<CString> = self.handles.iter()
            .map(|e| cast::vk_to_cstring(&e.extension_name)).collect();

        let is_all_extension_available = requrie_extension_names.iter()
            .all(|test_extension| {
                available_extensions.iter().find(|&backup_extension| {
                    backup_extension == test_extension
                }).is_some()
            });

        is_all_extension_available
    }

    fn set(&mut self, config: &Self::ConfigType) {

        self.enable_extensions = config.require_extensions.clone();
    }
}
