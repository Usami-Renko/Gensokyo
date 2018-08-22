
use ash;
use ash::vk;

use winit;

use core::instance::Instance;
use core::platforms;
use core::error::SurfaceError;

use structures::Dimension2D;

use constant::VERBOSE;

pub struct Surface<'win> {

    pub handle: vk::SurfaceKHR,
    loader: ash::extensions::Surface,

    window: &'win winit::Window,
}

impl<'win> Surface<'win> {

    pub fn new(instance: &Instance, window: &'win winit::Window) -> Result<Surface<'win>, SurfaceError> {

        let surface = unsafe {
            platforms::generate_surface(&instance.entry, &instance.handle, window)
                .or(Err(SurfaceError::SurfaceCreationError))?
        };

        let loader = ash::extensions::Surface::new(&instance.entry, &instance.handle)
            .or(Err(SurfaceError::ExtensionLoadError))?;


        let surface = Surface {
            handle: surface,
            loader,

            window,
        };

        Ok(surface)
    }

    pub fn is_present_support(&self, physical_device: vk::PhysicalDevice, queue_family_index: vk::uint32_t) -> bool {

        self.loader.get_physical_device_surface_support_khr(physical_device, queue_family_index, self.handle)
    }

    pub fn capabilities(&self, physical_device: vk::PhysicalDevice) -> Result<vk::SurfaceCapabilitiesKHR, SurfaceError> {

        self.loader.get_physical_device_surface_capabilities_khr(physical_device, self.handle)
            .or(Err(SurfaceError::QueryCapabilitiesError))
    }

    pub fn formats(&self, physical_device: vk::PhysicalDevice) -> Result<Vec<vk::SurfaceFormatKHR>, SurfaceError> {

        self.loader.get_physical_device_surface_formats_khr(physical_device, self.handle)
            .or(Err(SurfaceError::QueryFormatsError))
    }

    pub fn present_modes(&self, physical_device: vk::PhysicalDevice) -> Result<Vec<vk::PresentModeKHR>, SurfaceError> {

        self.loader.get_physical_device_surface_present_modes_khr(physical_device, self.handle)
            .or(Err(SurfaceError::QueryPresentModeError))
    }

    pub fn window_size(&self) -> Dimension2D {
        let size = self.window.get_inner_size().unwrap();
        Dimension2D {
            width:  size.width  as u32,
            height: size.height as u32,
        }
    }

    pub fn cleanup(&self) {

        unsafe {
            self.loader.destroy_surface_khr(self.handle, None);
        }

        if VERBOSE {
            println!("[Info] Surface had been destroy.");
        }
    }
}
