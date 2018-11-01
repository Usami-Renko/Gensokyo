
use ash::vk;
use ash::vk::uint32_t;
use ash::version::InstanceV1_0;

use core::instance::HaInstance;
use core::physical::HaPhyDevice;

use resources::error::MemoryError;

pub(crate) struct PhysicalMemory {

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

    pub fn find_memory_type(&self, type_filter: uint32_t, require_flags: vk::MemoryPropertyFlags, candidate_indices: Option<&Vec<usize>>)
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


pub(crate) struct MemorySelector {

    physical: HaPhyDevice,
    /// The index of memory type that available to use.
    candidate_memories: Vec<usize>,
}

impl MemorySelector {

    pub fn init(physical: &HaPhyDevice) -> MemorySelector {
        MemorySelector {
            physical: physical.clone(),
            candidate_memories: vec![],
        }
    }

    pub fn try(&mut self, type_filter: uint32_t, require_flags: vk::MemoryPropertyFlags) -> Result<(), MemoryError> {

        let new_candidates = self.physical.memory.find_memory_type(
            type_filter,
            require_flags,
            if self.candidate_memories.is_empty() { None } else { Some(&self.candidate_memories) }
        );

        if new_candidates.is_empty() {
            Err(MemoryError::NoSuitableMemoryError)
        } else {
            self.candidate_memories = new_candidates;
            Ok(())
        }
    }

    pub fn optimal_memory(&self) -> Result<usize, MemoryError> {

        // TODO: Use better method to find optimal memory
        let optimal_index = self.candidate_memories.first()
            .ok_or(MemoryError::NoSuitableMemoryError)?.clone();
        Ok(optimal_index)
    }

    pub fn reset(&mut self) {

        self.candidate_memories.clear();
    }
}
