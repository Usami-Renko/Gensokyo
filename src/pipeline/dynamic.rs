
use ash::vk;
use ash::vk::uint32_t;

use std::ptr;

pub struct HaDynamicState {

    /// DynamicState specifies which pieces of pipeline state will use the values from dynamic state commands rather than from pipeline state creation info.
    states: Vec<vk::DynamicState>,
}

impl HaDynamicState {

    pub fn null() -> HaDynamicState {
        HaDynamicState { states: vec![] }
    }

    pub fn setup(states: Vec<vk::DynamicState>) -> HaDynamicState {
        HaDynamicState { states, }
    }

    pub fn info(&self) -> vk::PipelineDynamicStateCreateInfo {
        vk::PipelineDynamicStateCreateInfo {
            s_type : vk::StructureType::PipelineDynamicStateCreateInfo,
            p_next : ptr::null(),
            // flags is reserved for future use in API version 1.0.82.
            flags  : vk::PipelineDynamicStateCreateFlags::empty(),

            dynamic_state_count : self.states.len() as uint32_t,
            p_dynamic_states    : self.states.as_ptr(),
        }
    }

    pub fn add_state(&mut self, state: vk::DynamicState) {
        self.states.push(state);
    }
}
