
use ash::vk;

use utility::marker::VulkanEnum;

use std::ptr;

pub struct HaInputAssembly {

    topology: vk::PrimitiveTopology,
    primitive_restart_enable: bool,
}

impl HaInputAssembly {

    pub fn setup(topology: PrimitiveTopology, primitive_restart_enable: bool) -> HaInputAssembly {
        HaInputAssembly {
            topology: topology.value(),
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


// TODO: Add description for PrimitiveTopology.
/// Primitive topology determines how consecutive vertices are organized into primitives, and determines the type of primitive that is used at the beginning of the graphics pipeline.
///
/// The effective topology for later stages of the pipeline is altered by tessellation or geometry shading (if either is in use) and depends on the execution modes of those shaders.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum PrimitiveTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip,
    TriangleFan,
    LineListWithAdjacency,
    LineStripWithAdjacency,
    TriangleListWithAdjacency,
    TriangleStripWithAdjacency,
    PatchList,
}

impl VulkanEnum for PrimitiveTopology {
    type EnumType = vk::PrimitiveTopology;

    fn value(&self) -> Self::EnumType {
        match *self {
            | PrimitiveTopology::PointList                  => vk::PrimitiveTopology::PointList,
            | PrimitiveTopology::LineList                   => vk::PrimitiveTopology::LineList,
            | PrimitiveTopology::LineStrip                  => vk::PrimitiveTopology::LineStrip,
            | PrimitiveTopology::TriangleList               => vk::PrimitiveTopology::TriangleList,
            | PrimitiveTopology::TriangleStrip              => vk::PrimitiveTopology::TriangleStrip,
            | PrimitiveTopology::TriangleFan                => vk::PrimitiveTopology::TriangleFan,
            | PrimitiveTopology::LineListWithAdjacency      => vk::PrimitiveTopology::LineListWithAdjacency,
            | PrimitiveTopology::LineStripWithAdjacency     => vk::PrimitiveTopology::LineStripWithAdjacency,
            | PrimitiveTopology::TriangleListWithAdjacency  => vk::PrimitiveTopology::TriangleListWithAdjacency,
            | PrimitiveTopology::TriangleStripWithAdjacency => vk::PrimitiveTopology::TriangleStripWithAdjacency,
            | PrimitiveTopology::PatchList                  => vk::PrimitiveTopology::PatchList,
        }
    }
}
