
use winit;

use core::instance::Instance;
use core::debug::Debugger;
use core::physical::{ PhysicalDevice, PhysicalRequirement };
use core::surface::Surface;
use core::device::{ LogicalDevice, LogicalDeviceBuilder, PrefabQueue };
use swapchain::{ Swapchain, SwapchainBuilder };

use procedure::window::ProgramEnv;
use procedure::error::ProcedureError;

use constant::core::VALIDATION;
use constant::swapchain::SWAPCHAIN_IMAGE_COUNT;

pub trait ProgramProc {


}

pub struct CoreInfrastructure<'win> {

    instance  : Instance,
    debugger  : Option<Debugger>,
    surface   : Surface<'win>,
    physical  : PhysicalDevice,
    device    : LogicalDevice,
    swapchain : Swapchain,
}

impl<'win, T> ProgramEnv<T> where T: ProgramProc {

    pub fn initialize_core(&self, window: &'win winit::Window, requirement: PhysicalRequirement)
        -> Result<CoreInfrastructure<'win>, ProcedureError> {

        let instance = Instance::new()
            .map_err(|e| ProcedureError::Instance(e))?;

        let debugger = if VALIDATION.is_enable {
            let debugger = Debugger::setup(&instance)
                .map_err(|e| ProcedureError::Validation(e))?;
            Some(debugger)
        } else {
            None
        };

        let surface = Surface::new(&instance, window)
            .map_err(|e| ProcedureError::Surface(e))?;

        let physical = PhysicalDevice::new(&instance, &surface, requirement)
            .map_err(|e| ProcedureError::PhysicalDevice(e))?;

        let device = LogicalDeviceBuilder::init(&instance, &physical)
            .setup_prefab_queue(&[
                PrefabQueue::GraphicsQueue,
                PrefabQueue::PresentQueue,
            ]).build()
            .map_err(|e| ProcedureError::LogicalDevice(e))?;

        let swapchain = SwapchainBuilder::init(&instance, &physical, &device, &surface)
            .map_err(|e| ProcedureError::SwapchainCreation(e))?
            .set_image_count(SWAPCHAIN_IMAGE_COUNT)
            .build()
            .map_err(|e| ProcedureError::SwapchainCreation(e))?;

        let core = CoreInfrastructure {
            instance,
            debugger,
            surface,
            physical,
            device,
            swapchain,
        };

        Ok(core)
    }
}

impl<'w> Drop for CoreInfrastructure<'w> {

    /// use cleanup function, so that the order of deinitialization can be customizable.
    fn drop(&mut self) {
        self.swapchain.cleanup(&self.device);
        self.device.cleanup();
        self.physical.cleanup();
        self.surface.cleanup();

        if let Some(ref debugger) = self.debugger {
            debugger.cleanup();
        }

        self.instance.clenaup();
    }
}
