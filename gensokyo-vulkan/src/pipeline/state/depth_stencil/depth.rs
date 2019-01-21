
use ash::vk;

use crate::pipeline::state::dynamic::DynamicableValue;

use crate::types::{ vkfloat, vkbool, VK_TRUE, VK_FALSE };

pub struct DepthTest {

    /// test_enable controls whether depth testing is enabled.
    pub test_enable: vkbool,
    /// write_enable controls whether depth writes are enabled when write_enable is true.
    /// Depth writes are always disabled when test_enable is false.
    pub write_enable: vkbool,
    /// compare_op is the comparison operator used in the depth test.
    pub compare_op: vk::CompareOp,

    pub depth_bound: DynamicableValue<DepthBoundInfo>,
}


impl DepthTest {

    pub fn enable(wirte_enable: bool, compare_op: vk::CompareOp, bounds_enable: bool) -> DepthTest {
        DepthTest {
            test_enable: VK_TRUE,
            write_enable: if wirte_enable { VK_TRUE } else { VK_FALSE },
            compare_op,
            depth_bound: if bounds_enable {
                DynamicableValue::Fixed { value: DepthBoundInfo::enable() }
            } else {
                DynamicableValue::Fixed { value: DepthBoundInfo::disable() }
            }
        }
    }

    pub fn disable() -> DepthTest {
        DepthTest { ..Default::default() }
    }

    pub fn set_depth_bound(&mut self, bound: DynamicableValue<DepthBoundInfo>) {
        self.depth_bound = bound;
    }

    pub(crate) fn is_dynamic_depthbound(&self) -> bool {
        self.depth_bound.is_dynamic()
    }
}

impl Default for DepthTest {

    fn default() -> DepthTest {
        DepthTest {
            test_enable:  VK_FALSE,
            write_enable: VK_FALSE,
            compare_op: vk::CompareOp::LESS,

            depth_bound: DynamicableValue::Fixed { value : DepthBoundInfo::disable() },
        }
    }
}

#[derive(Debug, Clone)]
pub struct DepthBoundInfo {

    /// `enable` controls whether depth bounds testing is enabled.
    pub enable: vkbool,
    /// `min_bounds` define minimum value used in the depth bounds test.
    ///
    /// Default is 0.0. This value must be smaller than min_bounds and between 0.0 and 1.0.
    pub min_bound: vkfloat,
    /// `max_bounds` define the maximum value used in depth bounds test.
    ///
    /// Default is 1.0. This value must be bigger than depth_bounds and between 0.0 and 1.0.
    pub max_bound: vkfloat,
}

impl DepthBoundInfo {

    /// Initialize Depth Bound with disabling depth bound test.
    pub fn disable() -> DepthBoundInfo {

        DepthBoundInfo {
            ..Default::default()
        }
    }

    /// Initialize Depth Bound with enabling depth bound test.
    pub fn enable() -> DepthBoundInfo {

        DepthBoundInfo {
            enable    : VK_TRUE,
            ..Default::default()
        }
    }

    /// Initialize Depth Bound value.
    ///
    /// `min_bounds` is the minimum value used in depth bounds test.
    ///
    /// `max_bounds` is the maximum value used in depth bounds test.
    pub fn setup(min: vkfloat, max: vkfloat) -> DepthBoundInfo {

        DepthBoundInfo {
            enable: VK_TRUE,
            min_bound: min,
            max_bound: max,
        }
    }
}

impl Default for DepthBoundInfo {

    fn default() -> DepthBoundInfo {

        DepthBoundInfo {
            enable    : VK_FALSE,
            min_bound: 0.0,
            max_bound: 1.0,
        }
    }
}

impl DynamicableValue<DepthBoundInfo> {

    pub(super) fn to_depth_bound(&self) -> DepthBoundInfo {

        match self {
            | DynamicableValue::Fixed { value } => value.clone(),
            | DynamicableValue::Dynamic => DepthBoundInfo::enable(),
        }
    }
}
