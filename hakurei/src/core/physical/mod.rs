
pub(crate) type HaPhyDevice = ::std::rc::Rc<HaPhysicalDevice>;
pub(crate) use self::object::{ HaPhysicalDevice, PhysicalDeviceType };
pub(crate) use self::memory::MemorySelector;
pub(crate) use self::requirement::PhysicalRequirement;
pub(crate) use self::features::PhysicalFeatureType;
pub(crate) use self::extension::DeviceExtensionType;
pub(crate) use self::family::QueueOperationType;

mod object;
mod features;
mod property;
mod memory;
mod family;
mod requirement;
mod extension;
mod formats;
mod limits;
