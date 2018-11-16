
use ash::vk;
use ash::version::InstanceV1_0;

use core::instance::HaInstance;

use utils::types::vkint;

pub struct PhysicalMemory {

    _handle: vk::PhysicalDeviceMemoryProperties,
    types  : Vec<vk::MemoryType>,
}

impl PhysicalMemory {

    pub fn inspect(instance: &HaInstance, physical_device: vk::PhysicalDevice) -> PhysicalMemory {

        let handle = instance.handle.get_physical_device_memory_properties(physical_device);
        let types = handle.memory_types.to_vec();

        PhysicalMemory {
            _handle: handle,
            types,
        }
    }

    pub fn find_memory_type(&self, type_filter: vkint, require_flags: vk::MemoryPropertyFlags, candidate_indices: Option<&Vec<usize>>)
        -> Vec<usize> {

        let mut result = vec![];

        if let Some(candidates) = candidate_indices {
            for &i in candidates.iter() {
                if (type_filter & (1 << i)) > 0 && self.types[i].property_flags.subset(require_flags) {
                    result.push(i);
                }
            }
        } else {
            let candidates = (0..self.types.len()).collect::<Vec<_>>();

            for &i in candidates.iter() {
                if (type_filter & (1 << i)) > 0 && self.types[i].property_flags.subset(require_flags) {
                    result.push(i);
                }
            }
        };

        result
    }

    pub fn memory_type(&self, index: usize) -> vk::MemoryType {
        self.types[index].clone()
    }

    pub fn check_requirements(&self) -> bool {

        // TODO: Add requirement check
        true
    }
}
