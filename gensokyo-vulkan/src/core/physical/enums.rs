
use std::ffi::CString;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum DeviceExtensionType {
    Swapchain,
}

impl DeviceExtensionType {

    pub(super) fn name(&self) -> CString {
        match self {
            | DeviceExtensionType::Swapchain => {
                // FIXME: Use the comment code instead
//                ash::extensions::Swapchain::name()
                CString::new("VK_KHR_swapchain").unwrap()
            }
        }
    }
}
