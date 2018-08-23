
use ash::vk;

use std::ptr;

// TODO: This module need futher development yet.
pub struct HaInputAssembly {

    state    : vk::PipelineVertexInputStateCreateInfo,
    assembly : vk::PipelineInputAssemblyStateCreateInfo,
}

impl HaInputAssembly {

    pub fn init() -> HaInputAssembly {
        HaInputAssembly {
            ..Default::default()
        }
    }
}

impl Default for HaInputAssembly {

    fn default() -> HaInputAssembly {
        HaInputAssembly {
            state: vk::PipelineVertexInputStateCreateInfo {
                s_type : vk::StructureType::PipelineVertexInputStateCreateInfo,
                p_next : ptr::null(),
                // flags is reserved for future use in API version 1.0.82.
                flags  : vk::PipelineVertexInputStateCreateFlags::empty(),
                vertex_binding_description_count   : 0,
                p_vertex_binding_descriptions      : ptr::null(),
                vertex_attribute_description_count : 0,
                p_vertex_attribute_descriptions    : ptr::null(),

            },
            assembly: vk::PipelineInputAssemblyStateCreateInfo {
                s_type   : vk::StructureType::PipelineInputAssemblyStateCreateInfo,
                p_next   : ptr::null(),
                // flags is reserved for future use in API version 1.0.82.
                flags    : vk::PipelineInputAssemblyStateCreateFlags::empty(),
                topology : vk::PrimitiveTopology::TriangleList,
                primitive_restart_enable: vk::VK_FALSE,
            }
        }
    }
}
