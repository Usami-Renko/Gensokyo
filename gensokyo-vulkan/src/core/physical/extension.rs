
use ash::vk;
use ash::version::InstanceV1_0;

use crate::core::instance::GsInstance;
use crate::core::physical::config::PhysicalInspectProperty;
use crate::utils::cast;
use crate::error::{ VkResult, VkError };

use std::ffi::CString;

pub(crate) struct PhysicalExtension {

    handles: Vec<vk::ExtensionProperties>,
    enable_extensions: Vec<CString>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DeviceExtensionType {
    Swapchain,
}

impl DeviceExtensionType {

    pub(super) fn name(&self) -> CString {
        match self {
            | DeviceExtensionType::Swapchain => {
                // FIXME: Use the comment code instead
                // ash::extensions::Swapchain::name()
                CString::new("VK_KHR_swapchain").unwrap()
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct PhysicalExtensionConfig {

    pub require_extensions: Vec<DeviceExtensionType>,
}

impl PhysicalExtension {

    pub fn query(instance: &GsInstance, physical_device: vk::PhysicalDevice) -> VkResult<PhysicalExtension> {

        let handles = unsafe {
            instance.handle.enumerate_device_extension_properties(physical_device)
                .or(Err(VkError::query("Device Extensions")))?
        };

        let result = PhysicalExtension {
            handles,
            enable_extensions: vec![],
        };

        Ok(result)
    }

    pub fn borrow_enable_extensions(&self) -> &Vec<CString> {

        &self.enable_extensions
    }
}

impl PhysicalInspectProperty for PhysicalExtension {
    type ConfigType = PhysicalExtensionConfig;

    fn inspect(&self, config: &Self::ConfigType) -> bool {

        if config.require_extensions.is_empty() { return true }

        let require_extension_names: Vec<CString> = config.require_extensions.iter()
            .map(|e| e.name()).collect();
        let available_extensions: Vec<CString> = self.handles.iter()
            .map(|e| cast::chars2cstring(&e.extension_name)).collect();

        let is_all_extension_available = require_extension_names.iter()
            .all(|test_extension| {
                available_extensions.contains(test_extension)
            });

        is_all_extension_available
    }

    fn set(&mut self, config: &Self::ConfigType) {

        self.enable_extensions = config.require_extensions.iter()
            .map(|e| e.name()).collect();
    }
}
