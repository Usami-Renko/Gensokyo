
use ash::vk;

use crate::types::vkuint;

use std::ptr;

#[derive(Debug)]
pub enum DynamicableValue<T> {
    Fixed { value: T },
    Dynamic,
}

/// Most states are baked into the pipeline, but there are still a few dynamic states that can be changed within a command buffer.
#[derive(Default)]
pub struct GsDynamicState {

    /// DynamicState specifies which pieces of pipeline state will use the values from dynamic state commands rather than from pipeline state creation info.
    states: Vec<vk::DynamicState>,
}

impl GsDynamicState {

    pub(crate) fn info(&self) -> vk::PipelineDynamicStateCreateInfo {

        vk::PipelineDynamicStateCreateInfo {
            s_type: vk::StructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO,
            p_next: ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags : vk::PipelineDynamicStateCreateFlags::empty(),
            dynamic_state_count: self.states.len() as vkuint,
            p_dynamic_states   : self.states.as_ptr(),
        }
    }

    pub fn add_state(&mut self, state: vk::DynamicState) {
        self.states.push(state);
    }

    pub fn is_contain_state(&self) -> bool {
        !self.states.is_empty()
    }
}

impl<T> DynamicableValue<T>{

    pub fn is_dynamic(&self) -> bool {
        match self {
            | DynamicableValue::Fixed { .. } => false,
            | DynamicableValue::Dynamic => true,
        }
    }
}

impl Clone for DynamicableValue<vkuint> {
    fn clone(&self) -> Self {
        self.to_owned()
    }
}

