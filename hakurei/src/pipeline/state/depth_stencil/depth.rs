
use ash::vk;
use ash::vk::Bool32;

use std::os::raw::c_float;

pub struct DepthTest {

    /// test_enable controls whether depth testing is enabled.
    pub(super) test_enable: Bool32,
    /// write_enable controls whether depth writes are enabled when write_enable is true. Depth writes are always disabled when test_enable is false.
    pub(super) write_enable: Bool32,
    /// compare_op is the comparison operator used in the depth test.
    pub(super) compare_op: vk::CompareOp,

    /// bounds_enable controls whether depth bounds testing is enabled.
    pub(super) bounds_enable: Bool32,
    /// min_bounds define minimum value used in the depth bounds test.
    ///
    /// Default is 0.0. This value must be smaller than min_bounds and between 0.0 and 1.0.
    pub(super) min_bounds: c_float,
    /// depth_bounds define the maximum value used in depth bounds test.
    ///
    /// Default is 1.0. This value must be bigger than depth_bounds and between 0.0 and 1.0.
    pub(super) max_bounds: c_float,
}

impl DepthTest {

    pub fn enable(wirte_enable: bool, compare_op: vk::CompareOp, bounds_enable: bool) -> DepthTest {
        DepthTest {
            test_enable: vk::VK_TRUE,
            write_enable: if wirte_enable { 1 } else { 0 },
            compare_op,
            bounds_enable: if bounds_enable { 1 } else { 0 },
            ..Default::default()
        }
    }

    pub fn disable() -> DepthTest {
        DepthTest { ..Default::default() }
    }

    pub fn set_bounds_range(&mut self, min: c_float, max: c_float) {
        self.min_bounds = min;
        self.max_bounds = max;
    }
}

impl Default for DepthTest {

    fn default() -> DepthTest {
        DepthTest {
            test_enable:  vk::VK_FALSE,
            write_enable: vk::VK_FALSE,
            compare_op: vk::CompareOp::Less,

            bounds_enable: vk::VK_FALSE,
            min_bounds: 0.0,
            max_bounds: 1.0,
        }
    }
}
