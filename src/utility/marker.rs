

pub trait VulkanFlags {
    type FlagType;

    fn flags(&self) -> Self::FlagType;
}

