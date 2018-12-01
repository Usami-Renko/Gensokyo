
use VERBOSE;

use ash::version::InstanceV1_0;

use core::instance::GsInstance;
use core::surface::GsSurface;

use core::physical::config::{ PhysicalConfig, PhysicalInspectProperty };
use core::physical::target::GsPhysicalDevice;
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

    pub fn inspect(&self, instance: &GsInstance, surface: &GsSurface) -> Result<GsPhysicalDevice, PhysicalDeviceError> {

        let alternative_devices = unsafe {
            instance.handle.enumerate_physical_devices()
                .or(Err(PhysicalDeviceError::EnumerateDeviceError))?
        };

        let mut optimal_device = None;

        for physical_device in alternative_devices.into_iter() {

            let mut extensions = PhysicalExtension::query(instance, physical_device)?;
            if extensions.inspect(&self.config.extension) {
                extensions.set(&self.config.extension);
            } else {
                continue
            }

            let mut families = PhysicalQueueFamilies::query(instance, physical_device, surface)?;
            if families.inspect(&self.config.queue_family) {
                families.set(&self.config.queue_family)
            } else {
                continue
            }

            let mut features = PhyscialFeatures::query(instance, physical_device);
            if features.inspect(&self.config.features) {
                features.set(&self.config.features)
            } else {
                continue
            }

            let mut properties = PhysicalProperties::query(instance, physical_device);
            if properties.inspect(&self.config.properties) {
                properties.set(&self.config.properties)
            } else {
                continue
            }

            let memory = PhysicalMemory::query(instance, physical_device);

            let physical = GsPhysicalDevice {
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
