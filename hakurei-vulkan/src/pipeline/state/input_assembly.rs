
use ash::vk;

use types::{ VK_TRUE, VK_FALSE };
use std::ptr;

pub struct HaInputAssemblyState {

    topology: vk::PrimitiveTopology,
    primitive_restart_enable: bool,
}

impl HaInputAssemblyState {

    pub fn setup(topology: vk::PrimitiveTopology, primitive_restart_enable: bool) -> HaInputAssemblyState {

        HaInputAssemblyState {
            topology, primitive_restart_enable,
        }
    }

    pub(crate) fn info(&self) -> vk::PipelineInputAssemblyStateCreateInfo {

        vk::PipelineInputAssemblyStateCreateInfo {
            s_type   : vk::StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
            p_next   : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags    : vk::PipelineInputAssemblyStateCreateFlags::empty(),
            topology : self.topology,
            primitive_restart_enable: if self.primitive_restart_enable { VK_TRUE } else { VK_FALSE },
        }
    }
}

impl Default for HaInputAssemblyState {

    fn default() -> HaInputAssemblyState {

        HaInputAssemblyState {
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            primitive_restart_enable: false,
        }
    }
}
