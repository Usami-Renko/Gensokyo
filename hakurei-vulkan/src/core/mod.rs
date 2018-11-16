
use ash;

type EntryV1    = ash::Entry<ash::version::V1_0>;
type InstanceV1 = ash::Instance<ash::version::V1_0>;
type DeviceV1   = ash::Device<ash::version::V1_0>;

pub mod config;
pub mod instance;
pub mod debug;
pub mod surface;
pub mod physical;
pub mod device;
pub mod swapchain;
pub mod error;

mod platforms;
