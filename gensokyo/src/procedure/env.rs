
use winit;

use crate::config::engine::EngineConfig;

use crate::procedure::window::WindowInfo;
use crate::procedure::workflow::RoutineBuilder;
use crate::procedure::error::{ RuntimeError, ProcedureError };

use gsvk::core::instance::GsInstance;
use gsvk::core::debug::GsDebugger;
use gsvk::core::surface::GsSurface;
use gsvk::core::device::{ GsDevice, LogicalDeviceBuilder };
use gsvk::core::physical::{ GsPhyDevice, PhysicalInspector };
use gsvk::core::swapchain::{ GsChain, SwapchainConfig, SwapchainBuilder };

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

impl<'env> ProgramEnv {

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

    instance: GsInstance,
    debugger: GsDebugger,

    surface: GsSurface,

    pub(super) physical : GsPhyDevice,
    pub(super) device   : GsDevice,
}

impl VulkanEnv {

    pub fn build(config: &EngineConfig, win: &winit::Window) -> Result<VulkanEnv, ProcedureError> {
        let instance = GsInstance::new(&config.core.instance, &config.core.validation)?;

        let debugger = GsDebugger::new(&instance, &config.core.validation)?;

        let surface = GsSurface::new(&instance, win)?;

        let physical_inspector = PhysicalInspector::new(&config.core.physical);
        let physical = physical_inspector.inspect(&instance, &surface)?;
        // Initialize the device with default queues. (one graphics queue, one present queue, one transfer queue)
        let (device, _custom_queues) = LogicalDeviceBuilder::init(&instance, &physical, &config.core.device)
            .build()?;

        let env = VulkanEnv {
            instance, debugger, surface,
            physical : Rc::new(physical),
            device   : Rc::new(device),
        };

        Ok(env)
    }

    pub fn new_chain(&self, config: &SwapchainConfig, old_chain: Option<&GsChain>, window: &winit::Window) -> Result<GsChain, ProcedureError> {

        let chain = SwapchainBuilder::init(config, &self.physical, &self.device, &self.surface)?
            .build(&self.instance, old_chain, window)?;
        Ok(Rc::new(chain))
    }

    /// use cleanup function, so that the order of deinitialization can be customizable.
    pub fn cleanup(&self) {

        self.physical.destroy();
        self.device.destroy();

        self.surface.destroy();

        self.debugger.destroy();
        self.instance.destroy();
    }
}
