

mod error;
mod platforms;

pub mod instance;
pub mod debug;

pub use self::debug::ValidationInfo;

use ash;
use ash::version::V1_0;
type EntryV1    = ash::Entry<V1_0>;
type InstanceV1 = ash::Instance<V1_0>;

