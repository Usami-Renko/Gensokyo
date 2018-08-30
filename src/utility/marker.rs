
pub trait VulkanFlags {
    type FlagType;

    /// Convenient method to combine flags.
    fn flags(&self) -> Self::FlagType;
}

pub trait Handles {
    type HandleType;

    /// Get a group of handles from wrapper class with 'Ha' prefix.
    fn handles(&self) -> Vec<Self::HandleType>;
}

pub trait VulkanEnum {
    type EnumType;

    fn value(&self) -> Self::EnumType;
}
