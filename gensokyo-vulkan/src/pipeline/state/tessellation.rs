
use ash::vk;

use crate::types::vkuint;

use std::ptr;

pub struct GsTessellationState {

    /// points_count is number of control points per patch.
    ///
    /// patchControlPoints must be greater than zero and less than or equal to vkPhysicalDeviceLimits::maxTessellationPatchSize.
    points_count: vkuint,
}

impl GsTessellationState {

    pub fn setup(points_count: vkuint) -> GsTessellationState {
        GsTessellationState {
            points_count,
        }
    }

    pub(crate) fn info(&self) -> vk::PipelineTessellationStateCreateInfo {
        vk::PipelineTessellationStateCreateInfo {
            s_type : vk::StructureType::PIPELINE_TESSELLATION_STATE_CREATE_INFO,
            p_next : ptr::null(),
            // flags is reserved for future use in API version 1.1.82.
            flags  : vk::PipelineTessellationStateCreateFlags::empty(),
            patch_control_points: self.points_count,
        }
    }
}
