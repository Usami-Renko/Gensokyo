
use ash::vk;
use ash::vk::Bool32;

pub struct HaLogicalOp {
    pub(in super) enable : Bool32,
    pub(in super) op     : vk::LogicOp,
}
impl HaLogicalOp {

    pub fn setup(op: vk::LogicOp) -> HaLogicalOp {
        HaLogicalOp { enable: vk::VK_TRUE, op, }
    }
    pub fn disable() -> HaLogicalOp {
        HaLogicalOp { enable: vk::VK_FALSE, op: vk::LogicOp::Copy, }
    }
}
