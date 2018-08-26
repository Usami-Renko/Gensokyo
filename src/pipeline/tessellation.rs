
use ash::vk;
use ash::vk::uint32_t;

use std::ptr;

pub struct HaTessellationState {

    /// points_count is number of control points per patch.
    ///
    /// patchControlPoints must be greater than zero and less than or equal to vkPhysicalDeviceLimits::maxTessellationPatchSize.
    points_count: uint32_t,
    max_count: uint32_t,
}

impl HaTessellationState {

    pub fn setup(points_count: uint32_t) -> HaTessellationState {
        HaTessellationState {
            points_count,
            // TODO: Add limitation check for points_count filed in future.
            max_count: 0,
        }
    }

    pub fn info(&self) -> vk::PipelineTessellationStateCreateInfo {
        vk::PipelineTessellationStateCreateInfo {
            s_type : vk::StructureType::PipelineTessellationStateCreateInfo,
            p_next : ptr::null(),
            // flags is reserved for future use in API version 1.0.82.
            flags  : vk::PipelineTessellationStateCreateFlags::empty(),
            patch_control_points: self.points_count,
        }
    }
}
