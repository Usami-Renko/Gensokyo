
pub(crate) use self::config::CoreConfig;

pub(crate) use self::config::{ APPLICATION_VERSION, ENGINE_VERSION, API_VERSION };
pub(crate) use self::config::{ APPLICATION_NAME, ENGINE_NAME };

pub(crate) use self::swapchain::SwapchainConfig;
pub(crate) use self::device::DeviceConfig;

mod config;
mod swapchain;
mod device;
