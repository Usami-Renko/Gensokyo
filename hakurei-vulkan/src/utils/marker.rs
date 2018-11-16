
pub(crate) trait VulkanFlags {
    type FlagType;

    /// Convenient method to combine flags.
    fn flags(&self) -> Self::FlagType;
}

pub(crate) trait Handles {
    type HandleType;

    /// Get a group of handles from wrapper class with 'Ha' prefix.
    fn handles(&self) -> Vec<Self::HandleType>;
}

pub(crate) trait VulkanEnum {
    type EnumType;

    fn value(&self) -> Self::EnumType;
}

pub(crate) trait Prefab {
    type PrefabType;

    fn generate(&self) -> Self::PrefabType;
}
