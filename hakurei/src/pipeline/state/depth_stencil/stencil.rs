
use ash::vk;
use ash::vk::Bool32;
use ash::vk::uint32_t;

use pipeline::state::DynamicableValue;
use utility::marker::VulkanEnum;

pub struct StencilTest {

    /// `enable` controls whether stencil testing is enabled.
    pub(super) enable: Bool32,
    /// Parameter of the stencil test.
    pub(super) front : StencilOpState,
    /// Parameter of the stencil test.
    pub(super) back  : StencilOpState,
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

    pub fn set_compare_mask(&mut self, mask: DynamicableValue<uint32_t>) {
        self.front.compare_mask = mask.clone();
        self.back.compare_mask  = mask.clone();
    }
    pub fn set_write_mask(&mut self, mask: DynamicableValue<uint32_t>) {
        self.front.write_mask = mask.clone();
        self.back.write_mask  = mask.clone();
    }
    pub fn set_reference(&mut self, reference: DynamicableValue<uint32_t>) {
        self.front.reference = reference.clone();
        self.back.reference  = reference.clone();
    }

    pub(crate) fn is_dynamic_compare_mask(&self) -> bool {
        self.front.compare_mask.is_dynamic()
    }
    pub(crate) fn is_dynamic_write_mask(&self) -> bool {
        self.front.write_mask.is_dynamic()
    }
    pub(crate) fn is_dynamic_reference(&self) -> bool {
        self.front.reference.is_dynamic()
    }
}

impl Default for StencilTest {

    fn default() -> StencilTest {
        StencilTest {
            enable: vk::VK_FALSE,
            front : StencilOpState { ..Default::default() },
            back  : StencilOpState { ..Default::default() },
        }
    }
}

#[derive(Debug)]
pub struct StencilOpState {

    /// `fail_op` specifies the action performed on samples that fail the stencil test.
    pub fail_op      : vk::StencilOp,
    /// `pass_op` specifies the action performed on samples that pass both the depth and stencil tests.
    pub pass_op      : vk::StencilOp,
    /// `depth_fail_op` specifies the action performed on samples that pass the stencil test and fail the depth test.
    pub depth_fail_op: vk::StencilOp,
    /// `compare_op` specifies the comparison operator used in the stencil test.
    pub compare_op   : vk::CompareOp,
    /// `compare_mask` selects the bits of the unsigned integer stencil values participating in the stencil test.
    pub compare_mask : DynamicableValue<uint32_t>,
    /// `write_mask` selects the bits of the unsigned integer stencil values updated by the stencil test in the stencil framebuffer attachment.
    pub write_mask   : DynamicableValue<uint32_t>,
    // `reference` is an integer reference value that is used in the unsigned stencil comparison.
    pub reference    : DynamicableValue<uint32_t>,
}

impl StencilOpState {

    pub fn origin(&self) -> vk::StencilOpState {
        vk::StencilOpState {
            fail_op      : self.fail_op,
            pass_op      : self.pass_op,
            depth_fail_op: self.depth_fail_op,
            compare_op   : self.compare_op,
            compare_mask : self.compare_mask.to_stencil_mask(),
            write_mask   : self.write_mask.to_stencil_mask(),
            reference    : self.reference.to_stencil_mask(),
        }
    }
}

impl Default for StencilOpState {

    fn default() -> StencilOpState {
        StencilOpState {
            fail_op      : vk::StencilOp::Keep,
            pass_op      : vk::StencilOp::Keep,
            depth_fail_op: vk::StencilOp::Keep,
            compare_op   : vk::CompareOp::Always,
            compare_mask : DynamicableValue::Fixed { value: 0 },
            write_mask   : DynamicableValue::Fixed { value: 0 },
            reference    : DynamicableValue::Fixed { value: 0 },
        }
    }
}

impl DynamicableValue<uint32_t> {

    fn to_stencil_mask(&self) -> uint32_t {
        match self {
            | DynamicableValue::Fixed { value } => value.clone(),
            | DynamicableValue::Dynamic => 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum StencilFaceFlag {
    /// `FrontBit` specifies that only the front set of stencil state is updated.
    FrontBit,
    /// `Back` specifies that only the back set of stencil state is updated
    BackBit,
    /// `FrontAndBack` is the combination of StenciFaceFlag::FrontBit and StenciFaceFlag::Back, and specifies that both sets of stencil state are updated.
    FrontAndBackBit,
}

impl VulkanEnum for StencilFaceFlag {
    type EnumType = vk::StencilFaceFlags;

    fn value(&self) -> Self::EnumType {
        match self {
            | StencilFaceFlag::FrontBit        => vk::STENCIL_FACE_FRONT_BIT,
            | StencilFaceFlag::BackBit         => vk::STENCIL_FACE_BACK_BIT,
            | StencilFaceFlag::FrontAndBackBit => vk::STENCIL_FRONT_AND_BACK,
        }
    }
}
