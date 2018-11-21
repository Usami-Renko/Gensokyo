
use ash;

use std::ffi::CString;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DeviceExtensionType {
    Swapchain,
}

impl DeviceExtensionType {

    pub(super) fn name(&self) -> CString {
        match self {
            | DeviceExtensionType::Swapchain => ash::extensions::Swapchain::name().to_owned()
        }
    }
}
