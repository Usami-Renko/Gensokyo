
use ash;
pub(crate) type EntryV1    = ash::Entry<ash::version::V1_0>;
pub(crate) type InstanceV1 = ash::Instance<ash::version::V1_0>;
pub(crate) use self::debug::ValidationInfo;

mod platforms;

pub(crate) mod instance;
pub(crate) mod debug;
pub(crate) mod surface;
pub(crate) mod physical;
pub(crate) mod device;
pub(crate) mod swapchain;
pub(crate) mod error;
