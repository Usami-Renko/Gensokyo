
mod features;
mod property;
mod memory;
mod family;
mod requirement;

use ash::vk;
use ash::version::InstanceV1_0;

use core::instance::Instance;
use core::surface::Surface;
use core::error::PhysicalDeviceError;

use constant::VERBOSE;

use self::property::PhysicalProperties;
use self::features::PhyscialFeatures;
use self::memory::PhysicalMemory;
use self::family::PhysicalQueueFamilies;

pub use ash::vk::PhysicalDeviceType as PhysicalDeviceType;
pub use self::requirement::PhysicalRequirement;

pub struct PhysicalDevice {

    handle:     vk::PhysicalDevice,
    properties: PhysicalProperties,
    features:   PhyscialFeatures,
    memory:     PhysicalMemory,
    families:   PhysicalQueueFamilies,
}

impl PhysicalDevice {

    pub fn new(instance: &Instance, surface: &Surface, requirement: PhysicalRequirement)
        -> Result<PhysicalDevice, PhysicalDeviceError> {

        let alternative_devices = instance.handle.enumerate_physical_devices()
            .or(Err(PhysicalDeviceError::EnumerateDeviceError))?;

        let mut optimal_device = None;

        for &physical_device in alternative_devices.iter() {
            let properties = PhysicalProperties::inspect(instance, physical_device);
            let is_properties_support = properties.check_requirements(&requirement.device_types);
            if is_properties_support == false { continue }

            let features = PhyscialFeatures::inspect(instance, physical_device);
            let is_features_support = features.check_requirements(&requirement.features);
            if is_features_support == false { continue }

            let memory = PhysicalMemory::inspect(instance, physical_device);
            let is_memory_support = memory.check_requirements();
            if is_memory_support == false { continue }

            let families = PhysicalQueueFamilies::inspect(instance, physical_device, surface)?;
            let is_families_support = families.check_requirements(&requirement.queue_operations);
            if is_families_support == false { continue }

            optimal_device = Some(
                PhysicalDevice {
                    handle: physical_device,
                    properties,
                    features,
                    memory,
                    families,
                }
            );

            break
        }

        if VERBOSE {
            if let Some(ref optimal_device) = optimal_device {
                optimal_device.properties.print_device_detail();
            }
        }

        optimal_device.ok_or(PhysicalDeviceError::NoSuitableDeviceError)
    }

    pub fn cleanup(&self) {
        // No method for delete physical device
        if VERBOSE {
            println!("[Info] Physical Device had been destroy.");
        }
    }
}
