
use ash::vk;
use ash::vk::Bool32;

pub struct HaLogicalOp {
    pub(crate) enable : Bool32,
    pub(crate) op     : vk::LogicOp,
}
impl HaLogicalOp {

    pub fn setup(op: vk::LogicOp) -> HaLogicalOp {
        HaLogicalOp { enable: vk::VK_TRUE, op, }
    }
    pub fn disable() -> HaLogicalOp {
        HaLogicalOp { enable: vk::VK_FALSE, op: vk::LogicOp::Copy, }
    }
}
