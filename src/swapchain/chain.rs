
use ash;
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::HaLogicalDevice;

use constant::VERBOSE;

pub struct HaSwapchain {

    handle: vk::SwapchainKHR,
    loader: ash::extensions::Swapchain,

    images: Vec<vk::Image>,
    views:  Vec<vk::ImageView>,

    format: vk::Format,
    extent: vk::Extent2D,
}

impl HaSwapchain {

    pub fn new(handle: vk::SwapchainKHR, loader: ash::extensions::Swapchain, images: Vec<vk::Image>, views: Vec<vk::ImageView>, format: vk::Format, extent: vk::Extent2D) -> HaSwapchain {
        HaSwapchain { handle, loader, images, views, format, extent }
    }

    pub fn cleanup(&self, device: &HaLogicalDevice) {
        unsafe {
            for &view in self.views.iter() {
                device.handle.destroy_image_view(view, None);
            }

            self.loader.destroy_swapchain_khr(self.handle, None);
        }

        if VERBOSE {
            println!("[Info] Swapchain had been destroy.");
        }
    }
}


