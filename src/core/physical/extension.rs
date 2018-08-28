
use ash;
use ash::vk;
use ash::version::InstanceV1_0;

use core::instance::HaInstance;
use core::error::PhysicalDeviceError;

use utility::cast;

use std::ffi::CString;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DeviceExtensionType {
    Swapchain,
}

impl DeviceExtensionType {

    fn name(&self) -> CString {
        match *self {
            | DeviceExtensionType::Swapchain => ash::extensions::Swapchain::name().to_owned()
        }
    }
}

pub struct PhysicalExtension {

    handles: Vec<vk::ExtensionProperties>,
    pub enables: Vec<CString>,
}

impl PhysicalExtension {

    pub fn inspect(instance: &HaInstance, physical_device: vk::PhysicalDevice) -> Result<PhysicalExtension, PhysicalDeviceError> {

        let handles = instance.handle.enumerate_device_extension_properties(physical_device)
            .or(Err(PhysicalDeviceError::EnumerateExtensionsError))?;

        let result = PhysicalExtension {
            handles,
            enables: vec![]
        };

        Ok(result)
    }

    pub fn check_requirements(&self, requrie_extensions: &Vec<DeviceExtensionType>) -> bool {

        if requrie_extensions.is_empty() { return true }

        let requrie_extension_names: Vec<CString> = requrie_extensions.iter()
            .map(|e| e.name()).collect();
        let available_extensions: Vec<CString> = self.handles.iter()
            .map(|e| cast::vk_to_cstring(&e.extension_name)).collect();

        requrie_extension_names.iter().all(|test_extension| {
            available_extensions.iter().find(|&backup_extension| {
                backup_extension == test_extension
            }).is_some()
        })
    }

    pub fn enable_extensions(&mut self, requrie_extensions: &Vec<DeviceExtensionType>) {

        self.enables = requrie_extensions.iter().map(|e| e.name()).collect()
    }
}
