
use ash::vk;
use ash::vk::Bool32;

use pipeline::state::DynamicableValue;

use std::os::raw::c_float;

pub struct DepthTest {
    /// test_enable controls whether depth testing is enabled.
    pub(super) test_enable: Bool32,
    /// write_enable controls whether depth writes are enabled when write_enable is true.
    /// Depth writes are always disabled when test_enable is false.
    pub(super) write_enable: Bool32,
    /// compare_op is the comparison operator used in the depth test.
    pub(super) compare_op: vk::CompareOp,

    pub(super) depth_bound: DynamicableValue<DepthBoundInfo>,
}


impl DepthTest {

    pub fn enable(wirte_enable: bool, compare_op: vk::CompareOp, bounds_enable: bool) -> DepthTest {
        DepthTest {
            test_enable: vk::VK_TRUE,
            write_enable: if wirte_enable { 1 } else { 0 },
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
            test_enable:  vk::VK_FALSE,
            write_enable: vk::VK_FALSE,
            compare_op: vk::CompareOp::Less,

            depth_bound: DynamicableValue::Fixed { value : DepthBoundInfo::disable() },
        }
    }
}

#[derive(Debug, Clone)]
pub struct DepthBoundInfo {
    /// `enable` controls whether depth bounds testing is enabled.
    pub(crate) enable: Bool32,
    /// `min_bounds` define minimum value used in the depth bounds test.
    ///
    /// Default is 0.0. This value must be smaller than min_bounds and between 0.0 and 1.0.
    pub(crate) min_bound: c_float,
    /// `max_bounds` define the maximum value used in depth bounds test.
    ///
    /// Default is 1.0. This value must be bigger than depth_bounds and between 0.0 and 1.0.
    pub(crate) max_bound: c_float,
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
            enable    : vk::VK_TRUE,
            ..Default::default()
        }
    }

    /// Initialize Depth Bound value.
    ///
    /// `min_bounds` is the minimum value used in depth bounds test.
    ///
    /// `max_bounds` is the maximum value used in depth bounds test.
    pub fn setup(min: c_float, max: c_float) -> DepthBoundInfo {
        DepthBoundInfo {
            enable: vk::VK_TRUE,
            min_bound: min,
            max_bound: max,
        }
    }
}

impl Default for DepthBoundInfo {

    fn default() -> DepthBoundInfo {
        DepthBoundInfo {
            enable    : vk::VK_FALSE,
            min_bound: 0.0,
            max_bound: 1.0,
        }
    }
}

impl DynamicableValue<DepthBoundInfo> {

    pub(crate) fn to_depth_bound(&self) -> DepthBoundInfo {
        match self {
            | DynamicableValue::Fixed { value } => value.clone(),
            | DynamicableValue::Dynamic => DepthBoundInfo::enable(),
        }
    }
}
