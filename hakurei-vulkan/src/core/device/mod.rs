
use std::rc::Rc;
pub type HaDevice = Rc<self::device::HaLogicalDevice>;

pub use self::device::HaLogicalDevice;
pub use self::builder::LogicalDeviceBuilder;
pub use self::device::DeviceConfig;
pub use self::enums::QueueRequestStrategy;
pub use self::enums::DeviceQueueIdentifier;

pub mod queue;

mod builder;
mod device;
mod enums;
