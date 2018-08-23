
use winit;

use core::instance::HaInstance;
use core::debug::HaDebugger;
use core::physical::{ HaPhysicalDevice, PhysicalRequirement };
use core::surface::HaSurface;
use core::device::{ HaLogicalDevice, LogicalDeviceBuilder, PrefabQueue };
use swapchain::{ HaSwapchain, SwapchainBuilder };

use pipeline::graphics::GraphicsPipeline;
use pipeline::graphics::builder::{ GraphicsPipelineConfig, GraphicsPipelineBuilder };
use pipeline::{ HaShaderInfo, HaInputAssembly, HaViewport };

use procedure::window::ProgramEnv;
use procedure::error::ProcedureError;

use constant::core::VALIDATION;
use constant::swapchain::SWAPCHAIN_IMAGE_COUNT;

pub trait ProgramProc {

    fn configure_shaders(&self) -> Vec<HaShaderInfo>;
    fn configure_inputs(&self)  -> HaInputAssembly;
}

pub struct CoreInfrastructure<'win> {

    instance  : HaInstance,
    debugger  : Option<HaDebugger>,
    surface   : HaSurface<'win>,
    physical  : HaPhysicalDevice,
    device    : HaLogicalDevice,
    swapchain : HaSwapchain,
    graphics_pipelines: Vec<GraphicsPipeline>,
}

impl<'win, T> ProgramEnv<T> where T: ProgramProc {

    pub fn initialize_core(&self, window: &'win winit::Window, requirement: PhysicalRequirement)
        -> Result<CoreInfrastructure<'win>, ProcedureError> {

        let instance = HaInstance::new()
            .map_err(|e| ProcedureError::Instance(e))?;

        let debugger = if VALIDATION.is_enable {
            let debugger = HaDebugger::setup(&instance)
                .map_err(|e| ProcedureError::Validation(e))?;
            Some(debugger)
        } else {
            None
        };

        let surface = HaSurface::new(&instance, window)
            .map_err(|e| ProcedureError::Surface(e))?;

        let physical = HaPhysicalDevice::new(&instance, &surface, requirement)
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

        // TODO: Currently just configuration a single pipeline.
        let shaders = self.procedure.configure_shaders();
        let inputs = self.procedure.configure_inputs();

        let viewport = HaViewport::setup(swapchain.extent);

        let mut pipeline_config = GraphicsPipelineConfig::init(shaders, inputs);
        pipeline_config.setup_viewport(viewport);
        pipeline_config.finish_config();

        let mut pipeline_builder = GraphicsPipelineBuilder::init();
        pipeline_builder.add_config(pipeline_config);

        let graphics_pipelines = pipeline_builder.build(&device)
            .map_err(|e| ProcedureError::Pipeline(e))?;

        let core = CoreInfrastructure {
            instance,
            debugger,
            surface,
            physical,
            device,
            swapchain,
            graphics_pipelines,
        };

        Ok(core)
    }
}

impl<'win> Drop for CoreInfrastructure<'win> {

    /// use cleanup function, so that the order of deinitialization can be customizable.
    fn drop(&mut self) {

        self.graphics_pipelines.iter().for_each(|pipeline|{
            pipeline.clean(&self.device);
        });
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
