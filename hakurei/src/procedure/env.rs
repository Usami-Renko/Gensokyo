
use winit;

use config::engine::EngineConfig;

use procedure::window::WindowInfo;
use procedure::workflow::RoutineBuilder;
use procedure::error::{ RuntimeError, ProcedureError };

use gsvk::core::instance::HaInstance;
use gsvk::core::debug::HaDebugger;
use gsvk::core::surface::HaSurface;
use gsvk::core::device::{ HaDevice, LogicalDeviceBuilder };
use gsvk::core::physical::{ HaPhyDevice, PhysicalInspector };
use gsvk::core::swapchain::{ HaSwapchain, SwapchainConfig, SwapchainBuilder };

use std::rc::Rc;
use std::path::PathBuf;

pub struct ProgramEnv {

    pub(super) window_env: WindowEnv,
    pub(super) vulkan_env: VulkanEnv,
    pub(super) config: EngineConfig,
}

pub(super) struct WindowEnv {

    event_loop: winit::EventsLoop,

    window_info: WindowInfo,
    test_window: Option<winit::Window>,
}

impl<'a, 'env> ProgramEnv {

    pub fn new(manifest: Option<PathBuf>) -> Result<ProgramEnv, RuntimeError> {

        let event_loop = winit::EventsLoop::new();
        let config = EngineConfig::init(manifest)?;

        let window_info = WindowInfo::from(config.window.clone());

        let window = window_info.build(&event_loop)
            .map_err(|e| RuntimeError::Window(e))?;

        let vulkan_env = VulkanEnv::build(&config, &window)?;

        let window_env = WindowEnv {
            event_loop, window_info,
            test_window: Some(window),
        };

        let env = ProgramEnv {
            window_env, vulkan_env, config,
        };

        Ok(env)
    }

    pub fn routine(&'env mut self) -> Result<RoutineBuilder<'env>, RuntimeError> {

        RoutineBuilder::new(self)
    }

    pub(super) fn window(&mut self) -> Result<winit::Window, RuntimeError> {

        let window = if self.window_env.test_window.is_some() {
            self.window_env.test_window.take().unwrap()
        } else {
            self.window_env.window_info.build(&self.window_env.event_loop)
                .map_err(|e| RuntimeError::Window(e))?
        };

        Ok(window)
    }

    pub(super) fn split(self) -> (WindowEnv, VulkanEnv, EngineConfig) {
        (self.window_env, self.vulkan_env, self.config)
    }
}

impl WindowEnv {

    pub fn borrow_mut_loops(&mut self) -> &mut winit::EventsLoop {
        &mut self.event_loop
    }
}

pub(super) struct VulkanEnv {

    instance: HaInstance,
    debugger: HaDebugger,

    surface: HaSurface,

    pub(super) physical : HaPhyDevice,
    pub(super) device   : HaDevice,
}

impl VulkanEnv {

    pub fn build(config: &EngineConfig, win: &winit::Window) -> Result<VulkanEnv, ProcedureError> {
        let instance = HaInstance::new(&config.core.instance, &config.core.validation)?;

        let debugger = HaDebugger::new(&instance, &config.core.validation)?;

        let surface = HaSurface::new(&instance, win)?;

        let physical_inspector = PhysicalInspector::new(&config.core.physical);
        let physical = physical_inspector.inspect(&instance, &surface)?;
        // Initialize the device with default queues. (one graphics queue, one present queue, one transfer queue)
        let device = LogicalDeviceBuilder::init(&instance, &physical, &config.core.device)?
            .build()?;

        let env = VulkanEnv {
            instance, debugger, surface,
            physical : Rc::new(physical),
            device   : Rc::new(device),
        };

        Ok(env)
    }

    pub fn new_chain(&self, config: &SwapchainConfig, old_chain: Option<&HaSwapchain>, window: &winit::Window) -> Result<HaSwapchain, ProcedureError> {

        let chain = SwapchainBuilder::init(config, &self.physical, &self.device, &self.surface)?
            .build(&self.instance, old_chain, window)?;
        Ok(chain)
    }

    /// use cleanup function, so that the order of deinitialization can be customizable.
    pub fn cleanup(&self) {

        self.physical.cleanup();
        self.device.cleanup();

        self.surface.cleanup();

        self.debugger.cleanup();
        self.instance.clenaup();
    }
}
