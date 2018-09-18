
use ash::vk;
use utility::marker::VulkanEnum;

/// The logical operations supported by Vulkan are summarized in the following table in which:
///
/// ¬ is bitwise invert.
///
/// ∧ is bitwise and.
///
/// ∨ is bitwise or.
///
/// ⊕ is bitwise exclusive or.
///
/// s is the fragment’s Rs0, Gs0, Bs0 or As0 component value for the fragment output corresponding to the color attachment being updated.
///
/// d is the color attachment’s R, G, B or A component value.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum LogicalOp {
    /// 0
    Clear,
    /// s ∧ d
    AND,
    /// s ∧ ¬ d
    ANDReverse,
    /// s
    Copy,
    /// ¬ s ∧ d
    ANDInverted,
    /// d
    NoOp,
    /// s ⊕ d
    XOR,
    /// s ∨ d
    OR,
    /// ¬ (s ∨ d)
    NOR,
    /// ¬ (s ⊕ d)
    Equivalent,
    /// ¬ d
    Invert,
    /// s ∨ ¬ d
    ORReverse,
    /// ¬ s
    CopyInverted,
    /// ¬ s ∨ d
    ORInverted,
    /// ¬ (s ∧ d)
    Nand,
    /// all 1s
    Set,
}


impl VulkanEnum for LogicalOp {
    type EnumType = vk::LogicOp;

    fn value(&self) -> Self::EnumType {
        match *self {
            | LogicalOp::Clear        => vk::LogicOp::Clear,
            | LogicalOp::AND          => vk::LogicOp::And,
            | LogicalOp::ANDReverse   => vk::LogicOp::AndReverse,
            | LogicalOp::Copy         => vk::LogicOp::Copy,
            | LogicalOp::ANDInverted  => vk::LogicOp::AndInverted,
            | LogicalOp::NoOp         => vk::LogicOp::No,
            | LogicalOp::XOR          => vk::LogicOp::Xor,
            | LogicalOp::OR           => vk::LogicOp::Or,
            | LogicalOp::NOR          => vk::LogicOp::Nor,
            | LogicalOp::Equivalent   => vk::LogicOp::Equivalent,
            | LogicalOp::Invert       => vk::LogicOp::Invert,
            | LogicalOp::ORReverse    => vk::LogicOp::OrReverse,
            | LogicalOp::CopyInverted => vk::LogicOp::CopyInverted,
            | LogicalOp::ORInverted   => vk::LogicOp::OrInverted,
            | LogicalOp::Nand         => vk::LogicOp::Nand,
            | LogicalOp::Set          => vk::LogicOp::Set,
        }
    }
}


/// CompareOp specifies the stencil comparison function.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum CompareOp {
    /// Never specifies that the test never passes.
    Never,
    /// Less specifies that the test passes when R < S.
    Less,
    /// Equal specifies that the test passes when R = S.
    Equal,
    /// LessOrEqual  specifies that the test passes when R ≤ S.
    LessOrEqual,
    /// Greater specifies that the test passes when R > S.
    Greater,
    /// NotEqual specifies that the test passes when R ≠ S.
    NotEqual,
    /// GreaterOrEqual specifies that the test passes when R ≥ S.
    GreaterOrEqual,
    /// Always specifies that the test always passes.
    Always,
}

impl VulkanEnum for CompareOp {
    type EnumType = vk::CompareOp;

    fn value(&self) -> Self::EnumType {
        match *self {
            | CompareOp::Never          => vk::CompareOp::Never,
            | CompareOp::Less           => vk::CompareOp::Less,
            | CompareOp::Equal          => vk::CompareOp::Equal,
            | CompareOp::LessOrEqual    => vk::CompareOp::LessOrEqual,
            | CompareOp::Greater        => vk::CompareOp::Greater,
            | CompareOp::NotEqual       => vk::CompareOp::NotEqual,
            | CompareOp::GreaterOrEqual => vk::CompareOp::GreaterOrEqual,
            | CompareOp::Always         => vk::CompareOp::Always,
        }
    }
}
