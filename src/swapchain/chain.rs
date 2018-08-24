
use ash;
use ash::vk;

use core::device::HaLogicalDevice;

use resources::image::{ HaImage, HaImageView };
use resources::framebuffer::HaFramebuffer;

pub struct HaSwapchain {

    pub(super) handle: vk::SwapchainKHR,
    pub(super) loader: ash::extensions::Swapchain,

    pub(super) images       : Vec<HaImage>,
    pub(super) views        : Vec<HaImageView>,
    pub(super) framebuffers : Vec<HaFramebuffer>,

    pub format: vk::Format,
    pub extent: vk::Extent2D,
}

impl HaSwapchain {

    pub fn cleanup(&self, device: &HaLogicalDevice) {

        self.framebuffers.iter().for_each(|f| f.cleanup(device));
        self.views.iter().for_each(|v| v.cleanup(device));

        unsafe {
            self.loader.destroy_swapchain_khr(self.handle, None);
        }
    }
}


