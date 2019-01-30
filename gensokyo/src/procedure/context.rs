
use winit;

use crate::config::engine::EngineConfig;

use crate::procedure::window::WindowInfo;
use crate::procedure::workflow::RoutineBuilder;
use crate::error::{ GsResult, GsError };

use gsvk::core::instance::GsInstance;
use gsvk::core::debug::GsDebugger;
use gsvk::core::surface::GsSurface;
use gsvk::core::{ GsDevice, GsVirtualDevice };
use gsvk::core::device::LogicalDeviceBuilder;
use gsvk::core::physical::PhysicalInspector;
use gsvk::core::swapchain::{ GsChain, GsSwapchain, SwapchainConfig };
use gsvk::types::vkDim2D;

use std::rc::Rc;
use std::path::PathBuf;

pub struct ProgramContext {

    pub(super) window_context: WindowContext,
    pub(super) vulkan_context: VulkanContext,
    pub(super) config: EngineConfig,
}

pub(super) struct WindowContext {

    pub(super) event_loop: winit::EventsLoop,

    window_info: WindowInfo,
    test_window: Option<winit::Window>,
}

impl<'env> ProgramContext {

    pub fn new(manifest: Option<PathBuf>) -> GsResult<ProgramContext> {

        let event_loop = winit::EventsLoop::new();
        let config = EngineConfig::init(manifest)?;

        let window_info = WindowInfo::from(config.window.clone());

        let window = window_info.build(&event_loop)
            .or(Err(GsError::window("Failed to create Window.")))?;

        let vulkan_context = VulkanContext::build(&config, &window)?;

        let window_context = WindowContext {
            event_loop, window_info,
            test_window: Some(window),
        };

        let env = ProgramContext { window_context, vulkan_context, config };
        Ok(env)
    }

    pub fn routine(&'env mut self) -> GsResult<RoutineBuilder<'env>> {

        RoutineBuilder::new(self)
    }

    pub(super) fn window(&mut self) -> GsResult<winit::Window> {

        let window = if self.window_context.test_window.is_some() {
            self.window_context.test_window.take().unwrap()
        } else {
            self.window_context.window_info.build(&self.window_context.event_loop)
                .or(Err(GsError::window("Failed to create Window.")))?
        };

        Ok(window)
    }

    pub(super) fn take(self) -> (WindowContext, VulkanContext, EngineConfig) {
        (self.window_context, self.vulkan_context, self.config)
    }
}

pub(super) struct VulkanContext {

    instance: GsInstance,
    debugger: GsDebugger,

    surface: GsSurface,

    pub(super) device   : GsDevice,
}

impl VulkanContext {

    pub fn build(config: &EngineConfig, win: &winit::Window) -> GsResult<VulkanContext> {

        let instance = GsInstance::new(&config.core.instance, &config.core.validation)?;
        let debugger = GsDebugger::new(&instance, &config.core.validation)?;

        let surface = GsSurface::new(&instance, win)?;

        let physical_inspector = PhysicalInspector::new(&config.core.physical);
        let physical_device = physical_inspector.inspect(&instance, &surface)?;
        // Initialize the device with default queues. (one graphics queue, one present queue, one transfer queue)
        let (logical_device, _custom_queues) = LogicalDeviceBuilder::init(&instance, &physical_device, &config.core.device)
            .build()?;
        let virtual_device = GsVirtualDevice {
            phys : physical_device,
            logic: logical_device,
        };

        let env = VulkanContext {
            instance, debugger, surface,
            device: Rc::new(virtual_device),
        };
        Ok(env)
    }

    pub fn new_chain(&self, config: &SwapchainConfig, old_chain: Option<&GsChain>, window: &winit::Window) -> GsResult<GsChain> {

        let win_dimension = window.get_inner_size()
            .ok_or(GsError::window("Failed to get Window size."))?;
        let window_dimension = vkDim2D { width: win_dimension.width as _, height: win_dimension.height as _ };

        let chain = GsSwapchain::new(&self.device, config, &self.surface)?
            .build(&self.instance, old_chain, &window_dimension)?;
        Ok(Rc::new(chain))
    }

    /// use destroy function, so that the order of deinitialization can be customizable.
    pub fn destroy(&self) {

        self.device.logic.destroy();
        self.device.phys.destroy();

        self.surface.destroy();

        self.debugger.destroy();
        self.instance.destroy();
    }
}
