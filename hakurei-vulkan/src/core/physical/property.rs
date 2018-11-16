
use ash::vk;
use ash::vk::uint32_t;
use ash::version::InstanceV1_0;

use core::instance::HaInstance;
use core::physical::PhysicalDeviceType;
use utils::cast;

pub struct PhysicalProperties {

    handle: vk::PhysicalDeviceProperties,
    device_name: String,
    api_version: uint32_t,
    device_type: PhysicalDeviceType,
}

impl PhysicalProperties {

    pub fn inspect(instance: &HaInstance, physical_device: vk::PhysicalDevice) -> PhysicalProperties {

        let handle = instance.handle.get_physical_device_properties(physical_device);
        let device_name = cast::vk_to_string(&handle.device_name);
        let api_version = handle.api_version;
        let device_type = handle.device_type.into();

        PhysicalProperties {
            handle,
            device_name,
            api_version,
            device_type,
        }
    }

    pub fn check_requirements(&self, requrie_device_types: &Vec<PhysicalDeviceType>) -> bool {

        requrie_device_types.is_empty() || requrie_device_types.iter().any(|&option| option == self.device_type)
    }

    pub fn print_device_detail(&self) {

        let (major, minor, patch) = (
            vk_version_major!(self.api_version),
            vk_version_minor!(self.api_version),
            vk_version_patch!(self.api_version),
        );

        let device_type: PhysicalDeviceType =  self.handle.device_type.into();

        println!("[info] Physical Device Details:");
        println!("\tDevice Name: {}", self.device_name);
        println!("\tDevice API version: ({}, {}, {})", major, minor, patch);
        println!("\tDevice Type: {}", device_type);
    }
}


