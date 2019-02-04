
use ash::vk;

use crate::core::instance::GsInstance;
use crate::core::platforms;
use crate::error::{ VkResult, VkError };

use crate::types::vkuint;

/// Wrapper class for `vk::Surface`.
pub struct GsSurface {

    /// the handle of `vk::SurfaceKHR`.
    pub(crate) handle: vk::SurfaceKHR,
    /// the extension loader provides functions for creation and destruction of `vk::SurfaceKHR` object.
    loader: ash::extensions::khr::Surface,
}

impl GsSurface {

    /// Initialize surface extension loader and `vk::Surface` object.
    pub fn new(instance: &GsInstance, window: &winit::Window) -> VkResult<GsSurface> {

        let handle = unsafe {
            platforms::generate_surface(&instance.entry, &instance.handle, window)
                .or(Err(VkError::unlink("Platform Surface")))?
        };

        let loader = ash::extensions::khr::Surface::new(&instance.entry, &instance.handle);

        let surface = GsSurface { handle, loader };
        Ok(surface)
    }

    /// Query whether a queue family of a physical device supports presentation to a given surface.
    ///
    /// return true if the queue family support presentation, or false if it doesn't support.
    pub fn query_is_family_presentable(&self, physical_device: vk::PhysicalDevice, queue_family_index: vkuint) -> bool {

        unsafe {
            self.loader.get_physical_device_surface_support(physical_device, queue_family_index, self.handle)
        }
    }

    /// Query the basic capabilities of a surface.
    ///
    /// capabilities usually needs in swapchain creation.
    pub fn query_capabilities(&self, physical_device: vk::PhysicalDevice) -> VkResult<vk::SurfaceCapabilitiesKHR> {

        unsafe {
            self.loader.get_physical_device_surface_capabilities(physical_device, self.handle)
                .or(Err(VkError::query("Surface Capabilities")))
        }
    }

    /// Query the supported swapchain format tuples for a surface.
    pub fn query_formats(&self, physical_device: vk::PhysicalDevice) -> VkResult<Vec<vk::SurfaceFormatKHR>> {

        unsafe {
            self.loader.get_physical_device_surface_formats(physical_device, self.handle)
                .or(Err(VkError::query("Surface Formats")))
        }
    }

    /// Query the supported presentation modes for a surface.
    pub fn query_present_modes(&self, physical_device: vk::PhysicalDevice) -> VkResult<Vec<vk::PresentModeKHR>> {

        unsafe {
            self.loader.get_physical_device_surface_present_modes(physical_device, self.handle)
                .or(Err(VkError::query("Surface Present Modes")))
        }
    }

    /// Destroy the `vk::SurfaceKHR` object.
    pub fn discard(&self) {

        unsafe {
            self.loader.destroy_surface(self.handle, None);
        }
    }
}
