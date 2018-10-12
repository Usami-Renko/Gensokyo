
pub use self::device::DeviceQueueIdentifier;
pub use self::builder::{ PrefabQueuePriority, QueueRequestStrategy };
pub type HaDevice = ::std::rc::Rc<HaLogicalDevice>;

pub(crate) use self::device::HaLogicalDevice;
pub(crate) use self::builder::LogicalDeviceBuilder;

pub use self::queue::*;

mod builder;
mod queue;
mod device;
