
use ash;
use ash::vk;

use winit;

use core::instance::Instance;
use core::platforms;
use core::error::SurfaceError;

use constant::VERBOSE;

pub struct Surface {

    handle: vk::SurfaceKHR,
    loader: ash::extensions::Surface,
}

impl Surface {

    pub fn new(instance: &Instance, window: &winit::Window) -> Result<Surface, SurfaceError> {

        let surface = unsafe {
            platforms::generate_surface(&instance.entry, &instance.handle, window)
                .or(Err(SurfaceError::SurfaceCreationError))?
        };

        let loader = ash::extensions::Surface::new(&instance.entry, &instance.handle)
            .or(Err(SurfaceError::SurfaceExtensionLoadError))?;


        let surface = Surface {
            handle: surface,
            loader,
        };

        Ok(surface)
    }

    pub fn is_present_support(&self, physical_device: vk::PhysicalDevice, queue_index: vk::uint32_t) -> bool {

        self.loader.get_physical_device_surface_support_khr(physical_device, queue_index, self.handle)
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
