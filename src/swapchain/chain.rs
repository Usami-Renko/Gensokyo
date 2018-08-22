
use ash;
use ash::vk;
use ash::version::DeviceV1_0;

use core::device::LogicalDevice;

use constant::VERBOSE;

pub struct Swapchain {

    handle: vk::SwapchainKHR,
    loader: ash::extensions::Swapchain,

    images: Vec<vk::Image>,
    views:  Vec<vk::ImageView>,

    format: vk::Format,
    extent: vk::Extent2D,
}

impl Swapchain {

    pub fn new(handle: vk::SwapchainKHR, loader: ash::extensions::Swapchain, images: Vec<vk::Image>, views: Vec<vk::ImageView>, format: vk::Format, extent: vk::Extent2D) -> Swapchain {
        Swapchain { handle, loader, images, views, format, extent }
    }

    pub fn cleanup(&self, device: &LogicalDevice) {
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


