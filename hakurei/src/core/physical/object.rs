
use ash::vk;
use ash::version::InstanceV1_0;

use config::VERBOSE;
use config::engine::EngineConfig;
use core::instance::HaInstance;
use core::surface::HaSurface;
use core::error::PhysicalDeviceError;
use core::physical::property::PhysicalProperties;
use core::physical::features::PhyscialFeatures;
use core::physical::memory::PhysicalMemory;
use core::physical::family::PhysicalQueueFamilies;
use core::physical::extension::PhysicalExtension;
use core::physical::formats::PhysicalFormatProperties;

use utility::marker::VulkanEnum;

use std::fmt;

pub struct HaPhysicalDevice {

    pub(crate) handle     : vk::PhysicalDevice,
    pub(super) properties : PhysicalProperties,
    pub(crate) features   : PhyscialFeatures,
    pub(crate) memory     : PhysicalMemory,
    pub(crate) families   : PhysicalQueueFamilies,
    pub(crate) extensions : PhysicalExtension,
    pub(crate) formats    : PhysicalFormatProperties,
}

impl HaPhysicalDevice {

    pub fn new(instance: &HaInstance, surface: &HaSurface, config: &EngineConfig) -> Result<HaPhysicalDevice, PhysicalDeviceError> {

        let alternative_devices = instance.handle.enumerate_physical_devices()
            .or(Err(PhysicalDeviceError::EnumerateDeviceError))?;

        let requirement = config.core.to_physical_requirement();

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

            let formats = PhysicalFormatProperties::inspect(instance, physical_device, config)?;

            optimal_device = Some(
                HaPhysicalDevice {
                    handle: physical_device,
                    properties, features, memory, families, extensions, formats,
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


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PhysicalDeviceType {
    Known,
    IntegratedGPU,
    DiscreteGPU,
    VirtualGPU,
    CPU,
}

impl VulkanEnum for PhysicalDeviceType {
    type EnumType = vk::PhysicalDeviceType;

    fn value(&self) -> Self::EnumType {
        match *self {
            | PhysicalDeviceType::Known         => vk::PhysicalDeviceType::Other,
            | PhysicalDeviceType::IntegratedGPU => vk::PhysicalDeviceType::IntegratedGpu,
            | PhysicalDeviceType::DiscreteGPU   => vk::PhysicalDeviceType::DiscreteGpu,
            | PhysicalDeviceType::VirtualGPU    => vk::PhysicalDeviceType::VirtualGpu,
            | PhysicalDeviceType::CPU           => vk::PhysicalDeviceType::Cpu,
        }
    }
}

impl From<vk::PhysicalDeviceType> for PhysicalDeviceType {

    fn from(type_: vk::PhysicalDeviceType) -> PhysicalDeviceType {
        match type_ {
            | vk::PhysicalDeviceType::Cpu           => PhysicalDeviceType::CPU,
            | vk::PhysicalDeviceType::IntegratedGpu => PhysicalDeviceType::VirtualGPU,
            | vk::PhysicalDeviceType::DiscreteGpu   => PhysicalDeviceType::DiscreteGPU,
            | vk::PhysicalDeviceType::VirtualGpu    => PhysicalDeviceType::IntegratedGPU,
            | vk::PhysicalDeviceType::Other         => PhysicalDeviceType::Known,
        }
    }
}

impl fmt::Display for PhysicalDeviceType {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let description = match self {
            | PhysicalDeviceType::CPU           => "CPU",
            | PhysicalDeviceType::IntegratedGPU => "Integrated GPU",
            | PhysicalDeviceType::DiscreteGPU   => "Discrate GPU",
            | PhysicalDeviceType::VirtualGPU    => "Virtual GPU",
            | PhysicalDeviceType::Known         => "Unknown",
        };

        write!(f, "{}", description)
    }
}
