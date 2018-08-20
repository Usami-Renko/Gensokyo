
use winit;

use core::instance::Instance;
use core::debug::Debugger;
use core::physical::{ PhysicalDevice, PhysicalRequirement };
use core::surface::Surface;

use procedure::window::ProgramEnv;

use constant::core::VALIDATION;

pub trait ProgramProc {


}

pub struct CoreInfrastructure {

    instance: Instance,
    debugger: Option<Debugger>,
    surface:  Surface,
    physical: PhysicalDevice,
}

impl<T> ProgramEnv<T> where T: ProgramProc {

    pub fn initialize_core(&self, window: &winit::Window, requirement: PhysicalRequirement) -> CoreInfrastructure {

        let instance = match Instance::new() {
            | Ok(instance) => instance,
            | Err(err) => panic!(format!("[Error] {}", err.to_string())),
        };

        let debugger = if VALIDATION.is_enable {
            match Debugger::setup(&instance) {
                | Ok(debugger) => Some(debugger),
                | Err(err) => panic!(format!("[Error] {}", err.to_string())),
            }
        } else {
            None
        };

        let surface = match Surface::new(&instance, window) {
            | Ok(surface) => surface,
            | Err(err) => panic!(format!("[Error] {}", err.to_string())),
        };

        let physical = match PhysicalDevice::new(&instance, &surface, requirement) {
            | Ok(physical_device) => physical_device,
            | Err(err) => panic!(format!("[Error] {}", err.to_string())),
        };

        CoreInfrastructure {
            instance,
            debugger,
            surface,
            physical,
        }
    }
}

impl Drop for CoreInfrastructure {

    /// use cleanup function, so that the order of deinitialization can be customizable.
    fn drop(&mut self) {
        self.physical.cleanup();
        self.surface.cleanup();

        if let Some(ref debugger) = self.debugger {
            debugger.cleanup();
        }

        self.instance.clenaup();
    }
}
