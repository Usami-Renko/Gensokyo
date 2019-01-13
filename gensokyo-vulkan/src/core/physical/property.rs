
use ash::vk;
use ash::{ vk_version_major, vk_version_minor, vk_version_patch };
use ash::version::InstanceV1_0;

use crate::core::instance::GsInstance;
use crate::core::physical::config::PhysicalInspectProperty;

use crate::types::vkuint;

use crate::utils::cast;

pub(crate) struct PhysicalProperties {

    handle: vk::PhysicalDeviceProperties,
    device_name: String,
    api_version: vkuint,
    device_type: vk::PhysicalDeviceType,
}

#[derive(Debug, Clone)]
pub struct PhysicalPropertiesConfig {

    pub require_device_types: Vec<vk::PhysicalDeviceType>,
}

impl PhysicalProperties {

    pub fn query(instance: &GsInstance, physical_device: vk::PhysicalDevice) -> PhysicalProperties {

        let handle = unsafe {
            instance.handle.get_physical_device_properties(physical_device)
        };
        let device_name = cast::chars2string(&handle.device_name);
        let api_version = handle.api_version;
        let device_type = handle.device_type;

        PhysicalProperties {
            handle, device_name, api_version, device_type,
        }
    }

    pub fn print_device_detail(&self) {

        let (major, minor, patch) = (
            vk_version_major!(self.api_version),
            vk_version_minor!(self.api_version),
            vk_version_patch!(self.api_version),
        );

        let device_type = self.handle.device_type;
        let device_type = match device_type {
            | vk::PhysicalDeviceType::CPU            => "CPU",
            | vk::PhysicalDeviceType::INTEGRATED_GPU => "Integrated GPU",
            | vk::PhysicalDeviceType::DISCRETE_GPU   => "Discrete GPU",
            | vk::PhysicalDeviceType::VIRTUAL_GPU    => "Virtual GPU",
            | _ => "Unknown",
        };

        println!("[info] Physical Device Details:");
        println!("\tDevice Name: {}", self.device_name);
        println!("\tDevice API version: ({}, {}, {})", major, minor, patch);
        println!("\tDevice Type: {}", device_type);
    }

    pub fn limits(&self) -> &vk::PhysicalDeviceLimits {
        &self.handle.limits
    }
}

impl PhysicalInspectProperty for PhysicalProperties {
    type ConfigType = PhysicalPropertiesConfig;

    fn inspect(&self, config: &Self::ConfigType) -> bool {

        config.require_device_types.is_empty() ||
            config.require_device_types.iter().any(|&option| option == self.device_type)
    }

    fn set(&mut self, _config: &Self::ConfigType) {
        // nothing to set, leave it empty...
    }
}
