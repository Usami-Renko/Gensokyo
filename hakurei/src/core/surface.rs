
use ash;
use ash::vk;

use winit;

use core::instance::HaInstance;
use core::platforms;
use core::error::SurfaceError;

use utility::dimension::Dimension2D;

/// Wrapper class for `vk::Surface`.
pub struct HaSurface<'win> {

    /// the handle of `vk::SurfaceKHR`.
    pub handle: vk::SurfaceKHR,
    /// the extension loader provides functions for creation and destruction of `vk::SurfaceKHR` object.
    loader: ash::extensions::Surface,
    /// an reference to window object.
    window: &'win winit::Window,
}

impl<'win> HaSurface<'win> {

    /// initialize surface extension loader and `vk::Surface` object.
    pub fn new(instance: &HaInstance, window: &'win winit::Window) -> Result<HaSurface<'win>, SurfaceError> {

        let surface = unsafe {
            platforms::generate_surface(&instance.entry, &instance.handle, window)
                .or(Err(SurfaceError::SurfaceCreationError))?
        };

        let loader = ash::extensions::Surface::new(&instance.entry, &instance.handle)
            .or(Err(SurfaceError::ExtensionLoadError))?;


        let surface = HaSurface {
            handle: surface,
            loader,

            window,
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
    pub fn formats(&self, physical_device: vk::PhysicalDevice) -> Result<Vec<vk::SurfaceFormatKHR>, SurfaceError> {

        self.loader.get_physical_device_surface_formats_khr(physical_device, self.handle)
            .or(Err(SurfaceError::QueryFormatsError))
    }

    /// query the supported presentation modes for a surface.
    pub fn present_modes(&self, physical_device: vk::PhysicalDevice) -> Result<Vec<vk::PresentModeKHR>, SurfaceError> {

        self.loader.get_physical_device_surface_present_modes_khr(physical_device, self.handle)
            .or(Err(SurfaceError::QueryPresentModeError))
    }

    /// query the dimension of current window.
    pub fn window_size(&self) -> Dimension2D {
        let size = self.window.get_inner_size().unwrap();
        Dimension2D {
            width:  size.width  as u32,
            height: size.height as u32,
        }
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
