
use winit;

use core::instance::HaInstance;
use core::debug::HaDebugger;
use core::physical::{ HaPhysicalDevice, PhysicalRequirement };
use core::surface::HaSurface;
use core::device::{ HaDevice, HaLogicalDevice, LogicalDeviceBuilder };
use core::swapchain::HaSwapchain;
use core::swapchain::SwapchainBuilder;
use core::swapchain::{ SwapchainError, SwapchainRuntimeError };
use resources::toolkit::{ AllocatorKit, PipelineKit };
use sync::fence::HaFence;
use sync::semaphore::HaSemaphore;

use procedure::window::ProgramEnv;
use procedure::error::ProcedureError;

use input::{ ActionNerve, SceneAction };
use utility::time::TimePeriod;

use std::rc::Rc;

pub trait ProgramProc {

    fn assets(&mut self, device: &HaDevice, kit: AllocatorKit) -> Result<(), ProcedureError>;
    fn pipelines(&mut self, kit: PipelineKit, swapchain: &HaSwapchain) -> Result<(), ProcedureError>;
    fn subresources(&mut self, _device: &HaDevice) -> Result<(), ProcedureError> { Ok(())}
    fn commands(&mut self, device: &HaDevice) -> Result<(), ProcedureError>;
    fn ready(&mut self, _device: &HaDevice) -> Result<(), ProcedureError> { Ok(()) }
    fn draw(&mut self, device: &HaDevice, device_available: &HaFence, image_available: &HaSemaphore, image_index: usize, delta_time: f32) -> Result<&HaSemaphore, ProcedureError>;
    fn closure(&mut self, _device: &HaDevice) -> Result<(), ProcedureError> { Ok(()) }
    fn clean_resources(&mut self, device: &HaDevice) -> Result<(), ProcedureError>;
    fn cleanup(&mut self, device: &HaDevice);

    fn react_input(&mut self, inputer: &ActionNerve, delta_time: f32) -> SceneAction;
}

pub struct CoreInfrastructure<'win> {

    instance  : HaInstance,
    debugger  : Option<HaDebugger>,
    surface   : HaSurface<'win>,

    pub(crate) physical: Rc<HaPhysicalDevice>,
    pub(crate) device  : Rc<HaLogicalDevice>,
}

pub struct HaResources {

    swapchain : HaSwapchain,

    // sync
    image_awaits:  Vec<HaSemaphore>,
    sync_fences:   Vec<HaFence>,
}

impl<'win, T> ProgramEnv<T> where T: ProgramProc {

    pub(super) fn initialize_core(&self, window: &'win winit::Window, requirement: PhysicalRequirement)
        -> Result<CoreInfrastructure<'win>, ProcedureError> {

        let instance = HaInstance::new(&self.config)?;

        let debugger = if self.config.core.validation.is_enable {
            let debugger = HaDebugger::setup(&instance, &self.config.core.validation.flags)?;
            Some(debugger)
        } else {
            None
        };

        let surface = HaSurface::new(&instance, window)?;
        let physical = HaPhysicalDevice::new(&instance, &surface, requirement)?;
        // Initialize the device with default queues. (one graphics queue, one present queue, one transfer queue)
        let device = LogicalDeviceBuilder::init(&instance, &physical)
            .build(&self.config.core)?;

        let core = CoreInfrastructure {
            instance, debugger, surface,
            device  : Rc::new(device),
            physical: Rc::new(physical),
        };
        Ok(core)
    }

    pub(super) fn load_resources(&mut self, core: &CoreInfrastructure) -> Result<HaResources, ProcedureError> {

        let inner_resource = self.create_inner_resources(core, None)?;

        self.procedure.assets(&core.device, AllocatorKit::init(&core.physical, &core.device))?;
        self.procedure.pipelines(PipelineKit::init(&core.device), &inner_resource.swapchain)?;
        self.procedure.subresources(&core.device)?;
        self.procedure.commands(&core.device,)?;

        Ok(inner_resource)
    }

    pub(super) fn reload_resources(&mut self, core: &CoreInfrastructure, old_resource: &HaResources) -> Result<HaResources, ProcedureError> {

        let inner_resource = self.create_inner_resources(core, Some(&old_resource))?;

        self.procedure.pipelines(PipelineKit::init(&core.device), &inner_resource.swapchain)?;
        self.procedure.subresources(&core.device)?;
        self.procedure.commands(&core.device,)?;

        Ok(inner_resource)
    }

    pub(super) fn draw_frame(&mut self, current_frame: usize, core: &mut CoreInfrastructure, resources: &mut HaResources, delta_time: f32)
        -> Result<(), ProcedureError> {

        let fence_to_wait = &resources.sync_fences[current_frame];
        fence_to_wait.wait(TimePeriod::Infinte)?;

        let image_result = resources.swapchain.next_image(Some(&resources.image_awaits[current_frame]), None);
        let image_index = match image_result {
            | Ok(result) => result,
            | Err(e) =>
                match e {
                    | SwapchainRuntimeError::SurfaceOutOfDateError
                    | SwapchainRuntimeError::SurfaceSubOptimalError => {
                        return Err(ProcedureError::SwapchainRecreate)
                    }
                    | _ => return Err(ProcedureError::Swapchain(SwapchainError::Runtime(e)))
                }
        };

        fence_to_wait.reset()?;

        let present_available = self.procedure.draw(&core.device, fence_to_wait, &resources.image_awaits[current_frame], current_frame, delta_time)?;

        // FIXME: Use present queue will cause crash. Image ownership transfer is necessary,
        // see https://github.com/KhronosGroup/Vulkan-Docs/wiki/Synchronization-Examples.
        // or see https://software.intel.com/en-us/articles/api-without-secrets-introduction-to-vulkan-part-3#inpage-nav-6-3
        let present_result = resources.swapchain.present(
            &[present_available],
            image_index,
            &core.device.graphics_queue
        );
        if let Err(e) = present_result {
            match e {
                | SwapchainRuntimeError::SurfaceOutOfDateError
                | SwapchainRuntimeError::SurfaceSubOptimalError => {
                    return Err(ProcedureError::SwapchainRecreate)
                }
                | _ => return Err(ProcedureError::Swapchain(SwapchainError::Runtime(e)))
            }
        }

        Ok(())
    }

    pub fn wait_idle(&self, device: &HaDevice) -> Result<(), ProcedureError> {
        device.wait_idle()
            .map_err(|e| ProcedureError::LogicalDevice(e))
    }

    fn create_inner_resources(&self, core: &CoreInfrastructure, old_resource: Option<&HaResources>) -> Result<HaResources, ProcedureError> {

        let swapchain = SwapchainBuilder::init(&self.config, &core.physical, &core.device, &core.surface)?
            .build(&core.instance, old_resource.and_then(|re| Some(&re.swapchain)))?;

        // sync
        let mut image_awaits = vec![];
        let mut sync_fences = vec![];
        for _ in 0..self.config.swapchain.image_count {
            let image_await = HaSemaphore::setup(&core.device)?;
            let sync_fence = HaFence::setup(&core.device, true)?;

            image_awaits.push(image_await);
            sync_fences.push(sync_fence);
        }

        let resources = HaResources {
            swapchain,

            image_awaits,
            sync_fences,
        };
        Ok(resources)
    }
}

impl<'win> CoreInfrastructure<'win> {

    /// use cleanup function, so that the order of deinitialization can be customizable.
    pub fn cleanup(&self) {

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

    pub fn cleanup(&self, device: &HaDevice) {

        self.swapchain.cleanup(device);
        self.image_awaits.iter().for_each(|i| i.cleanup());
        self.sync_fences.iter().for_each(|f| f.cleanup());
    }

    pub fn clear(&mut self) {

        self.image_awaits.clear();
        self.sync_fences.clear();
    }
}
