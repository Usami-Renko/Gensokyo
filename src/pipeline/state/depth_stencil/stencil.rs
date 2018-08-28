
use ash::vk;
use ash::vk::Bool32;
use ash::vk::uint32_t;

pub struct StencilTest {

    /// enable controls whether stencil testing is enabled
    pub(super) enable: Bool32,
    /// Parameter of the stencil test.
    pub(super) front: StencilOpState,
    /// Parameter of the stencil test.
    pub(super) back: StencilOpState,
}

impl StencilTest {

    pub fn enalbe() -> StencilTest {
        StencilTest {
            enable: vk::VK_TRUE,
            ..Default::default()
        }
    }

    pub fn disable() -> StencilTest {
        StencilTest { ..Default::default() }
    }

    pub fn set_front(&mut self, front: StencilOpState) {
        self.front = front;
    }
    pub fn set_back(&mut self, back: StencilOpState) {
        self.back = back;
    }
}

impl Default for StencilTest {

    fn default() -> StencilTest {
        StencilTest {
            enable: vk::VK_FALSE,
            front: StencilOpState { ..Default::default() },
            back:  StencilOpState { ..Default::default() },
        }
    }
}

#[derive(Debug, Clone)]
pub struct StencilOpState {

    /// fail_op specifies the action performed on samples that fail the stencil test.
    pub fail_op       : vk::StencilOp,
    /// pass_op specifies the action performed on samples that pass both the depth and stencil tests.
    pub pass_op       : vk::StencilOp,
    /// depth_fail_op specifies the action performed on samples that pass the stencil test and fail the depth test.
    pub depth_fail_op : vk::StencilOp,
    /// compare_op specifies the comparison operator used in the stencil test.
    pub compare_op    : vk::CompareOp,
    /// compare_mask selects the bits of the unsigned integer stencil values participating in the stencil test.
    pub compare_mask  : uint32_t,
    /// write_mask selects the bits of the unsigned integer stencil values updated by the stencil test in the stencil framebuffer attachment.
    pub write_mask    : uint32_t,
    // reference is an integer reference value that is used in the unsigned stencil comparison.
    pub reference     : uint32_t,
}

impl StencilOpState {

    pub fn origin(&self) -> vk::StencilOpState {
        vk::StencilOpState {
            fail_op       : self.fail_op,
            pass_op       : self.pass_op,
            depth_fail_op : self.depth_fail_op,
            compare_op    : self.compare_op,
            compare_mask  : self.compare_mask,
            write_mask    : self.write_mask,
            reference     : self.reference,
        }
    }
}

impl Default for StencilOpState {

    fn default() -> StencilOpState {
        StencilOpState {
            fail_op       : vk::StencilOp::Keep,
            pass_op       : vk::StencilOp::Keep,
            depth_fail_op : vk::StencilOp::Keep,
            compare_op    : vk::CompareOp::Always,
            compare_mask  : 0,
            write_mask    : 0,
            reference     : 0,
        }
    }
}
