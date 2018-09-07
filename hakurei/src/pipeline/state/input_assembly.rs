
use ash::vk;

use std::ptr;

pub struct HaInputAssembly {

    topology: vk::PrimitiveTopology,
    primitive_restart_enable: bool,
}

impl HaInputAssembly {

    pub fn setup(topology: vk::PrimitiveTopology, primitive_restart_enable: bool) -> HaInputAssembly {
        HaInputAssembly {
            topology,
            primitive_restart_enable,
        }
    }

    pub(crate) fn info(&self) -> vk::PipelineInputAssemblyStateCreateInfo {
        vk::PipelineInputAssemblyStateCreateInfo {
            s_type   : vk::StructureType::PipelineInputAssemblyStateCreateInfo,
            p_next   : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags    : vk::PipelineInputAssemblyStateCreateFlags::empty(),
            topology : self.topology,
            primitive_restart_enable: if self.primitive_restart_enable { vk::VK_TRUE } else { vk::VK_FALSE },
        }
    }
}

impl Default for HaInputAssembly {

    fn default() -> HaInputAssembly {
        HaInputAssembly {
            topology: vk::PrimitiveTopology::TriangleList,
            primitive_restart_enable: false,
        }
    }
}
