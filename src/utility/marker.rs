
pub trait VulkanFlags {
    type FlagType;

    /// Convenient method to combine flags.
    fn flags(&self) -> Self::FlagType;
}
