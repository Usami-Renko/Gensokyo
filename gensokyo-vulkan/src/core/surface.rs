
use ash::vk;

use winit;

use crate::core::instance::GsInstance;
use crate::core::platforms;
use crate::core::error::SurfaceError;

use crate::types::vkuint;

/// Wrapper class for `vk::Surface`.
pub struct GsSurface {

    /// the handle of `vk::SurfaceKHR`.
    pub(crate) handle: vk::SurfaceKHR,
    /// the extension loader provides functions for creation and destruction of `vk::SurfaceKHR` object.
    loader: ash::extensions::Surface,
}

impl GsSurface {

    /// initialize surface extension loader and `vk::Surface` object.
    pub fn new(instance: &GsInstance, window: &winit::Window) -> Result<GsSurface, SurfaceError> {

        let handle = unsafe {
            platforms::generate_surface(&instance.entry, &instance.handle, window)
                .or(Err(SurfaceError::SurfaceCreationError))?
        };

        let loader = ash::extensions::Surface::new(&instance.entry, &instance.handle);

        let surface = GsSurface {
            handle, loader,
        };

        Ok(surface)
    }

    /// query whether a queue family of a physical device supports presentation to a given surface.
    ///
    /// return true if the queue family support presentation, or false if it doesn't support.
    pub fn query_is_family_presentable(&self, physical_device: vk::PhysicalDevice, queue_family_index: vkuint) -> bool {

        unsafe {
            self.loader.get_physical_device_surface_support_khr(physical_device, queue_family_index, self.handle)
        }
    }

    /// query the basic capabilities of a surface.
    ///
    /// capabilities usually needs in swapchain creation.
    pub fn query_capabilities(&self, physical_device: vk::PhysicalDevice) -> Result<vk::SurfaceCapabilitiesKHR, SurfaceError> {

        unsafe {
            self.loader.get_physical_device_surface_capabilities_khr(physical_device, self.handle)
                .or(Err(SurfaceError::QueryCapabilitiesError))
        }
    }

    /// query the supported swapchain format tuples for a surface.
    pub fn query_formats(&self, physical_device: vk::PhysicalDevice) -> Result<Vec<vk::SurfaceFormatKHR>, SurfaceError> {

        unsafe {
            self.loader.get_physical_device_surface_formats_khr(physical_device, self.handle)
                .or(Err(SurfaceError::QueryFormatsError))
        }
    }

    /// query the supported presentation modes for a surface.
    pub fn query_present_modes(&self, physical_device: vk::PhysicalDevice) -> Result<Vec<vk::PresentModeKHR>, SurfaceError> {

        unsafe {
            self.loader.get_physical_device_surface_present_modes_khr(physical_device, self.handle)
                .or(Err(SurfaceError::QueryPresentModeError))
        }
    }

    /// Some cleaning operations before this object was uninitialized.
    ///
    /// For `GsSurface`, it destroy the `vk::SurfaceKHR` object.
    pub fn cleanup(&self) {

        unsafe {
            self.loader.destroy_surface_khr(self.handle, None);
        }
    }
}
