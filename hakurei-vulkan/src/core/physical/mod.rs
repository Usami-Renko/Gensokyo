
use std::rc::Rc;
pub type HaPhyDevice = Rc<self::target::HaPhysicalDevice>;

pub use self::enums::DeviceExtensionType;
pub use self::target::HaPhysicalDevice;
pub use self::inspector::PhysicalInspector;

pub use self::features::PhysicalFeatureConfig;
pub use self::property::PhysicalPropertiesConfig;
pub use self::family::PhysicalQueueFamilyConfig;
pub use self::extension::PhysicalExtensionConfig;

mod inspector;
mod target;

mod features;
mod property;
mod memory;
mod family;
mod extension;
mod limits;

mod config;
mod enums;
