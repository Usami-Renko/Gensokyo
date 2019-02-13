
pub mod config;
pub mod instance;
pub mod debug;
pub mod surface;
pub mod physical;
pub mod device;
pub mod swapchain;

mod platforms;


pub type GsDevice = ::std::rc::Rc<GsVirtualDevice>;

pub struct GsVirtualDevice {

    pub logic: self::device::GsLogicalDevice,
    pub phys : self::physical::GsPhysicalDevice,
}
