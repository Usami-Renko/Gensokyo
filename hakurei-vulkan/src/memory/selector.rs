
use ash::vk;

use core::physical::HaPhyDevice;

use memory::{ HaMemoryType, MemoryDstEntity };
use memory::error::MemoryError;

pub struct MemorySelector {

    physical: HaPhyDevice,
    /// The index of memory type that available to use.
    candidate_memories: Vec<usize>,

    dst_memory: HaMemoryType,
    memory_flag: vk::MemoryPropertyFlags,
}

impl MemorySelector {

    pub fn init(physical: &HaPhyDevice, dst_memory: HaMemoryType) -> MemorySelector {

        let memory_flag = dst_memory.property_flags();

        MemorySelector {
            physical: physical.clone(),
            candidate_memories: Vec::new(),
            dst_memory,
            memory_flag,
        }
    }

    pub fn try(&mut self, dst_enitty: &impl MemoryDstEntity) -> Result<(), MemoryError> {

        let new_candidates = self.physical.memory.find_memory_type(
            dst_enitty.type_bytes(),
            self.memory_flag,
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

    pub fn optimal_mem_type(&self) -> Result<vk::MemoryType, MemoryError> {

        let optimal_index = self.candidate_memories.first()
            .ok_or(MemoryError::NoSuitableMemoryError)?.clone();
        let result = self.physical.memory.memory_type(optimal_index);

        Ok(result)
    }

    pub fn reset(&mut self) {

        self.candidate_memories.clear();
        self.memory_flag = self.dst_memory.property_flags();
    }
}