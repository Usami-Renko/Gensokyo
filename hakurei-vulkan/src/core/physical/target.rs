
use ash::vk;
use ash::version::InstanceV1_0;

use VERBOSE;
use core::config::CoreConfig;
use core::instance::HaInstance;
use core::surface::HaSurface;
use core::error::PhysicalDeviceError;
use core::physical::property::PhysicalProperties;
use core::physical::features::PhyscialFeatures;
use core::physical::memory::PhysicalMemory;
use core::physical::family::PhysicalQueueFamilies;
use core::physical::extension::PhysicalExtension;
use core::physical::formats::PhysicalFormatProperties;

use pipeline::config::PipelineConfig;

use utils::marker::VulkanEnum;

use std::fmt;

pub struct HaPhysicalDevice {

    pub handle : vk::PhysicalDevice,

    pub properties : PhysicalProperties,
    pub features   : PhyscialFeatures,
    pub memory     : PhysicalMemory,
    pub families   : PhysicalQueueFamilies,
    pub extensions : PhysicalExtension,
    pub formats    : PhysicalFormatProperties,
}

impl HaPhysicalDevice {

    // TODO: Remove pipeline_conig.
    pub fn new(instance: &HaInstance, surface: &HaSurface, config: &CoreConfig, pipeline_conig: &PipelineConfig) -> Result<HaPhysicalDevice, PhysicalDeviceError> {

        let alternative_devices = instance.handle.enumerate_physical_devices()
            .or(Err(PhysicalDeviceError::EnumerateDeviceError))?;

        let requirement = config.to_physical_requirement();

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

            let formats = PhysicalFormatProperties::inspect(instance, physical_device, &pipeline_conig.depth_stencil)?;

            let selected_physical_device = HaPhysicalDevice {
                handle: physical_device,
                properties, features, memory, families, extensions, formats,
            };

            optimal_device = Some(selected_physical_device);

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
    Unknown,
    IntegratedGPU,
    DiscreteGPU,
    VirtualGPU,
    CPU,
}

impl VulkanEnum for PhysicalDeviceType {
    type EnumType = vk::PhysicalDeviceType;

    fn value(&self) -> Self::EnumType {
        match self {
            | PhysicalDeviceType::Unknown       => vk::PhysicalDeviceType::Other,
            | PhysicalDeviceType::IntegratedGPU => vk::PhysicalDeviceType::IntegratedGpu,
            | PhysicalDeviceType::DiscreteGPU   => vk::PhysicalDeviceType::DiscreteGpu,
            | PhysicalDeviceType::VirtualGPU    => vk::PhysicalDeviceType::VirtualGpu,
            | PhysicalDeviceType::CPU           => vk::PhysicalDeviceType::Cpu,
        }
    }
}

impl From<vk::PhysicalDeviceType> for PhysicalDeviceType {

    fn from(typ: vk::PhysicalDeviceType) -> PhysicalDeviceType {
        match typ {
            | vk::PhysicalDeviceType::Cpu           => PhysicalDeviceType::CPU,
            | vk::PhysicalDeviceType::IntegratedGpu => PhysicalDeviceType::VirtualGPU,
            | vk::PhysicalDeviceType::DiscreteGpu   => PhysicalDeviceType::DiscreteGPU,
            | vk::PhysicalDeviceType::VirtualGpu    => PhysicalDeviceType::IntegratedGPU,
            | vk::PhysicalDeviceType::Other         => PhysicalDeviceType::Unknown,
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
            | PhysicalDeviceType::Unknown => "Unknown",
        };

        write!(f, "{}", description)
    }
}
