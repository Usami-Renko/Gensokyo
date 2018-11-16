
use ash;
use ash::vk;

use winit;

use core::instance::HaInstance;
use core::platforms;
use core::swapchain::{ SurfaceFormat, PresentMode };
use core::error::SurfaceError;

/// Wrapper class for `vk::Surface`.
pub struct HaSurface {

    /// the handle of `vk::SurfaceKHR`.
    pub handle: vk::SurfaceKHR,
    /// the extension loader provides functions for creation and destruction of `vk::SurfaceKHR` object.
    loader: ash::extensions::Surface,
}

impl HaSurface {

    /// initialize surface extension loader and `vk::Surface` object.
    pub fn new(instance: &HaInstance, window: &winit::Window) -> Result<HaSurface, SurfaceError> {

        let handle = unsafe {
            platforms::generate_surface(&instance.entry, &instance.handle, window)
                .or(Err(SurfaceError::SurfaceCreationError))?
        };

        let loader = ash::extensions::Surface::new(&instance.entry, &instance.handle)
            .or(Err(SurfaceError::ExtensionLoadError))?;

        let surface = HaSurface {
            handle, loader,
        };

        Ok(surface)
    }

    /// query whether a queue family of a physical device supports presentation to a given surface.
    ///
    /// return true if the queue family support presentation, or false if it doesn't support.
    pub fn is_present_support(&self, physical_device: vk::PhysicalDevice, queue_family_index: vk::uint32_t) -> bool {

        self.loader.get_physical_device_surface_support_khr(physical_device, queue_family_index, self.handle)
    }

    /// query the basic capabilities of a surface.
    ///
    /// capabilities usually needs in swapchain creation.
    pub fn capabilities(&self, physical_device: vk::PhysicalDevice) -> Result<vk::SurfaceCapabilitiesKHR, SurfaceError> {

        self.loader.get_physical_device_surface_capabilities_khr(physical_device, self.handle)
            .or(Err(SurfaceError::QueryCapabilitiesError))
    }

    /// query the supported swapchain format tuples for a surface.
    pub fn formats(&self, physical_device: vk::PhysicalDevice) -> Result<Vec<SurfaceFormat>, SurfaceError> {

        self.loader.get_physical_device_surface_formats_khr(physical_device, self.handle)
            .map(|formats| formats.into_iter()
                .map(|format| format.into())
                .collect()
            ).or(Err(SurfaceError::QueryFormatsError))
    }

    /// query the supported presentation modes for a surface.
    pub fn present_modes(&self, physical_device: vk::PhysicalDevice) -> Result<Vec<PresentMode>, SurfaceError> {

        self.loader.get_physical_device_surface_present_modes_khr(physical_device, self.handle)
            .map(|modes| modes.into_iter()
                .map(|mode| mode.into())
                .collect()
            ).or(Err(SurfaceError::QueryPresentModeError))
    }

    /// Some cleaning operations before this object was uninitialized.
    ///
    /// For HaSurface, it destroy the `vk::SurfaceKHR` object.
    pub fn cleanup(&self) {

        unsafe {
            self.loader.destroy_surface_khr(self.handle, None);
        }
    }
}
