
use winit;

use core::instance::HaInstance;
use core::debug::HaDebugger;
use core::physical::{ HaPhysicalDevice, PhysicalRequirement };
use core::surface::HaSurface;
use core::device::{ HaLogicalDevice, LogicalDeviceBuilder };
use core::swapchain::chain::HaSwapchain;
use core::swapchain::builder::SwapchainBuilder;
use core::swapchain::error::{ SwapchainError, SwapchainRuntimeError };
use resources::allocator::ResourceGenerator;
use sync::fence::HaFence;
use sync::semaphore::HaSemaphore;

use procedure::window::ProgramEnv;
use procedure::error::ProcedureError;

use constant::core::VALIDATION;
use constant::swapchain::SWAPCHAIN_IMAGE_COUNT;
use constant::sync::SYNCHRONOUT_FRAME;

use utility::time::TimePeriod;

pub trait ProgramProc {

    fn configure_storage(&mut self, device: &HaLogicalDevice, generator: &ResourceGenerator) -> Result<(), ProcedureError>;
    fn configure_pipeline(&mut self, device: &HaLogicalDevice, swapchain: &HaSwapchain) -> Result<(), ProcedureError>;
    fn configure_resources(&mut self, device: &HaLogicalDevice) -> Result<(), ProcedureError>;
    fn configure_commands(&mut self, device: &HaLogicalDevice) -> Result<(), ProcedureError>;
    fn draw(&mut self, device: &HaLogicalDevice, device_available: &HaFence, image_available: &HaSemaphore, image_index: usize) -> Result<&HaSemaphore, ProcedureError>;
    fn cleanup(&self, device: &HaLogicalDevice);
}

pub struct CoreInfrastructure<'win> {

    instance  : HaInstance,
    debugger  : Option<HaDebugger>,
    surface   : HaSurface<'win>,
    pub(crate) physical: HaPhysicalDevice,
    pub(crate) device  : HaLogicalDevice,
    swapchain : HaSwapchain,
}

pub struct HaResources {

    // sync
    image_awaits:  Vec<HaSemaphore>,
    sync_fences:   Vec<HaFence>,
}

impl<'win, T> ProgramEnv<T> where T: ProgramProc {

    pub(super) fn initialize_core(&self, window: &'win winit::Window, requirement: PhysicalRequirement)
        -> Result<CoreInfrastructure<'win>, ProcedureError> {

        let instance = HaInstance::new()?;

        let debugger = if VALIDATION.is_enable {
            let debugger = HaDebugger::setup(&instance)?;
            Some(debugger)
        } else {
            None
        };

        let surface = HaSurface::new(&instance, window)?;
        let physical = HaPhysicalDevice::new(&instance, &surface, requirement)?;
        // Initialize the device with default queues. (one graphics queue, one present queue, one transfer queue)
        let device = LogicalDeviceBuilder::init(&instance, &physical)
            .build()?;
        let swapchain = SwapchainBuilder::init(&physical, &device, &surface)?
            .set_image_count(SWAPCHAIN_IMAGE_COUNT)
            .build(&instance)?;

        let core = CoreInfrastructure {
            instance, debugger, surface, physical, device, swapchain,
        };
        Ok(core)
    }

    pub(super) fn load_resources(&mut self, core: &CoreInfrastructure) -> Result<HaResources, ProcedureError> {

        let resource_generator = ResourceGenerator::init(&core.physical, &core.device);
        self.procedure.configure_storage(&core.device, &resource_generator)?;
        self.procedure.configure_pipeline(&core.device, &core.swapchain)?;
        self.procedure.configure_resources(&core.device)?;
        self.procedure.configure_commands(&core.device,)?;

        // sync
        let mut image_awaits = vec![];
        let mut sync_fences = vec![];
        for _ in 0..SYNCHRONOUT_FRAME {
            let image_await = HaSemaphore::setup(&core.device)?;
            let sync_fence = HaFence::setup(&core.device, true)?;

            image_awaits.push(image_await);
            sync_fences.push(sync_fence);
        }

        let resources = HaResources {

            image_awaits,
            sync_fences,
        };
        Ok(resources)
    }

    pub(super) fn draw_frame(&mut self, current_frame: usize, core: &mut CoreInfrastructure, resources: &mut HaResources)
        -> Result<(), ProcedureError> {

        let fence_to_wait = &resources.sync_fences[current_frame];
        fence_to_wait.wait(&core.device, TimePeriod::Infinte)?;

        let image_result = core.swapchain.next_image(Some(&resources.image_awaits[current_frame]), None);
        let image_index = match image_result {
            | Ok(result) => result,
            | Err(e) =>
                match e {
                    | SwapchainRuntimeError::SurfaceOutOfDateError
                    | SwapchainRuntimeError::SurfaceSubOptimalError => {
                        core.swapchain.recreate();
                        return Ok(())
                    }
                    | _ => return Err(ProcedureError::Swapchain(SwapchainError::Runtime(e)))
                }
        };

        fence_to_wait.reset(&core.device)?;

        let present_available = self.procedure.draw(&core.device, fence_to_wait, &resources.image_awaits[current_frame], current_frame)?;

        // FIXME: Use present queue will cause crash. Image ownership transfer is necessary,
        // see https://github.com/KhronosGroup/Vulkan-Docs/wiki/Synchronization-Examples.
        // or see https://software.intel.com/en-us/articles/api-without-secrets-introduction-to-vulkan-part-3
        let present_result = core.swapchain.present(
            &[present_available],
            image_index,
            &core.device.graphics_queue
        );
        if let Err(e) = present_result {
            match e {
                | SwapchainRuntimeError::SurfaceOutOfDateError
                | SwapchainRuntimeError::SurfaceSubOptimalError => {
                    core.swapchain.recreate();
                    return Ok(())
                }
                | _ => return Err(ProcedureError::Swapchain(SwapchainError::Runtime(e)))
            }
        }

        Ok(())
    }

    pub fn wait_idle(&self, device: &HaLogicalDevice) -> Result<(), ProcedureError> {
        device.wait_idle()
            .map_err(|e| ProcedureError::LogicalDevice(e))
    }
}

impl<'win> CoreInfrastructure<'win> {

    /// use cleanup function, so that the order of deinitialization can be customizable.
    pub fn cleanup(&self) {

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

impl HaResources {

    pub fn cleanup(&self, device: &HaLogicalDevice) {

        self.image_awaits.iter().for_each(|i| i.cleanup(device));
        self.sync_fences.iter().for_each(|f| f.cleanup(device));

    }
}
