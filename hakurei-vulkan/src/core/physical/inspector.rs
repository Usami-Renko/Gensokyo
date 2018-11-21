
use VERBOSE;

use ash::version::InstanceV1_0;

use core::instance::HaInstance;
use core::surface::HaSurface;

use core::physical::config::{ PhysicalConfig, PhysicalInspectProperty };
use core::physical::target::HaPhysicalDevice;
use core::physical::extension::PhysicalExtension;
use core::physical::family::PhysicalQueueFamilies;
use core::physical::features::PhyscialFeatures;
use core::physical::memory::PhysicalMemory;
use core::physical::property::PhysicalProperties;

use core::error::PhysicalDeviceError;

pub struct PhysicalInspector {

    config: PhysicalConfig,
}

impl PhysicalInspector {

    pub fn new(config: &PhysicalConfig) -> PhysicalInspector {

        PhysicalInspector {
            config: config.clone(),
        }
    }

    pub fn inspect(&self, instance: &HaInstance, surface: &HaSurface) -> Result<HaPhysicalDevice, PhysicalDeviceError> {

        let alternative_devices = unsafe {
            instance.handle.enumerate_physical_devices()
                .or(Err(PhysicalDeviceError::EnumerateDeviceError))?
        };

        let mut optimal_device = None;

        for physical_device in alternative_devices.into_iter() {

            let extensions = PhysicalExtension::query(instance, physical_device)?;
            let is_extension_support = extensions.inspect(&self.config.extension);
            if is_extension_support == false { continue }

            let families = PhysicalQueueFamilies::query(instance, physical_device, surface)?;
            let is_family_support = families.inspect(&self.config.queue_family);
            if is_family_support == false { continue }

            let features = PhyscialFeatures::query(instance, physical_device);
            let is_features_support = features.inspect(&self.config.features);
            if is_features_support == false { continue }

            let properties = PhysicalProperties::query(instance, physical_device);
            let is_properties_support = properties.inspect(&self.config.properties);
            if is_properties_support == false { continue }

            let memory = PhysicalMemory::query(instance, physical_device);

            let physical = HaPhysicalDevice {
                handle: physical_device,
                families, features, extensions, memory,
            };
            optimal_device = Some(physical);

            if VERBOSE {
                properties.print_device_detail();
            }

            break
        }

        optimal_device.ok_or(PhysicalDeviceError::NoSuitableDeviceError)
    }
}
