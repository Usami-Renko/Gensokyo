
use ash::vk;
use ash::version::InstanceV1_0;

use core::instance::GsInstance;
use types::vkuint;

pub(crate) struct PhysicalMemory {

    _handle: vk::PhysicalDeviceMemoryProperties,
    types  : Vec<vk::MemoryType>,
}

impl PhysicalMemory {

    pub fn query(instance: &GsInstance, physical_device: vk::PhysicalDevice) -> PhysicalMemory {

        let handle = unsafe {
            instance.handle.get_physical_device_memory_properties(physical_device)
        };
        let types = handle.memory_types.to_vec();

        PhysicalMemory {
            _handle: handle,
            types,
        }
    }

    pub fn find_memory_type(&self, type_filter: vkuint, require_flags: vk::MemoryPropertyFlags, candidate_indices: Option<&Vec<usize>>)
        -> Vec<usize> {

        let mut result = vec![];

        if let Some(candidates) = candidate_indices {
            for &i in candidates.iter() {
                if (type_filter & (1 << i)) > 0 && self.types[i].property_flags.contains(require_flags) {
                    result.push(i);
                }
            }
        } else {
            let candidates: Vec<usize> = (0..self.types.len()).collect();

            for &i in candidates.iter() {
                if (type_filter & (1 << i)) > 0 && self.types[i].property_flags.contains(require_flags) {
                    result.push(i);
                }
            }
        };

        result
    }

    pub fn memory_type(&self, index: usize) -> vk::MemoryType {
        self.types[index].clone()
    }
}
