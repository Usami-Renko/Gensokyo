
use std::rc::Rc;
pub type HaPhyDevice = Rc<self::target::HaPhysicalDevice>;

pub use self::requirement::PhysicalRequirement;
pub use self::features::PhysicalFeatureType;
pub use self::extension::DeviceExtensionType;
pub use self::target::{ HaPhysicalDevice, PhysicalDeviceType };
pub use self::family::QueueOperationType;

mod target;
mod features;
mod property;
mod memory;
mod family;
mod requirement;
mod extension;
mod formats;
mod limits;
