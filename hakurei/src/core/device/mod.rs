
pub use self::device::DeviceQueueIdentifier;
pub use self::builder::PrefabQueuePriority;
pub type HaDevice = ::std::rc::Rc<HaLogicalDevice>;

pub(crate) use self::device::HaLogicalDevice;
pub(crate) use self::builder::{ LogicalDeviceBuilder, QueueRequestStrategy };

pub use self::queue::*;

mod builder;
mod queue;
mod device;
