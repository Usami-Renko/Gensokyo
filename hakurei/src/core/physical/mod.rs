
pub(crate) type HaPhyDevice = ::std::rc::Rc<HaPhysicalDevice>;
pub(crate) use self::object::HaPhysicalDevice;
pub(crate) use self::memory::MemorySelector;
pub(crate) use self::requirement::PhysicalRequirement;

pub use self::object::PhysicalDeviceType;
pub use self::features::PhysicalFeatureType;
pub use self::extension::DeviceExtensionType;
pub use self::family::QueueOperationType;

mod object;
mod features;
mod property;
mod memory;
mod family;
mod requirement;
mod extension;
mod formats;
mod limits;
