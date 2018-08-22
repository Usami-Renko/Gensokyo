
use ash::vk;
use ash::version::InstanceV1_0;

use core::instance::HaInstance;

pub struct PhysicalMemory {

    handle: vk::PhysicalDeviceMemoryProperties,
}

impl PhysicalMemory {

    pub fn inspect(instance: &HaInstance, physical_device: vk::PhysicalDevice) -> PhysicalMemory {

        let handle = instance.handle.get_physical_device_memory_properties(physical_device);

        PhysicalMemory {
            handle,
        }
    }

    pub fn check_requirements(&self) -> bool {

        // TODO: Add requirement check
        true
    }
}


