
mod features;
mod property;
mod memory;
mod family;
mod requirement;
mod extension;
mod limits;

use ash::vk;
use ash::version::InstanceV1_0;

use core::instance::HaInstance;
use core::surface::HaSurface;
use core::error::PhysicalDeviceError;

use config::VERBOSE;

use self::property::PhysicalProperties;
use self::features::PhyscialFeatures;
use self::memory::PhysicalMemory;
use self::family::PhysicalQueueFamilies;
use self::extension::PhysicalExtension;

pub use ash::vk::PhysicalDeviceType as PhysicalDeviceType;
pub use self::requirement::PhysicalRequirement;
pub use self::extension::DeviceExtensionType;
pub use self::memory::MemorySelector;

pub struct HaPhysicalDevice {

    pub(crate) handle     : vk::PhysicalDevice,
    pub(super) properties : PhysicalProperties,
    pub(super) features   : PhyscialFeatures,
    pub(crate) memory     : PhysicalMemory,
    pub(super) families   : PhysicalQueueFamilies,
    pub(super) extensions : PhysicalExtension,
}

impl HaPhysicalDevice {

    pub fn new(instance: &HaInstance, surface: &HaSurface, requirement: PhysicalRequirement)
               -> Result<HaPhysicalDevice, PhysicalDeviceError> {

        let alternative_devices = instance.handle.enumerate_physical_devices()
            .or(Err(PhysicalDeviceError::EnumerateDeviceError))?;

        let mut optimal_device = None;

        for &physical_device in alternative_devices.iter() {

            let properties = PhysicalProperties::inspect(instance, physical_device);
            let is_properties_support = properties.check_requirements(&requirement.device_types);
            if is_properties_support == false { continue }

            let mut features = PhyscialFeatures::inspect(instance, physical_device);
            let is_features_support = features.check_requirements(&requirement.features);
            if is_features_support {
                features.enable_features(&requirement.features);
            } else {
                continue
            }

            let memory = PhysicalMemory::inspect(instance, physical_device);
            let is_memory_support = memory.check_requirements();
            if is_memory_support == false { continue }

            let families = PhysicalQueueFamilies::inspect(instance, physical_device, surface)?;
            let is_families_support = families.check_requirements(&requirement.queue_operations);
            if is_families_support == false { continue }

            let mut extensions = PhysicalExtension::inspect(instance, physical_device)?;
            let is_extensions_support = extensions.check_requirements(&requirement.extensions);
            if is_extensions_support {
                extensions.enable_extensions(&requirement.extensions);
            } else {
                continue
            }

            optimal_device = Some(
                HaPhysicalDevice {
                    handle: physical_device,
                    properties,
                    features,
                    memory,
                    families,
                    extensions,
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
        // leave it empty
    }
}
