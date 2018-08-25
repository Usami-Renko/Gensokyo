
use winit;

use core::instance::HaInstance;
use core::debug::HaDebugger;
use core::physical::{ HaPhysicalDevice, PhysicalRequirement };
use core::surface::HaSurface;
use core::device::{ HaLogicalDevice, LogicalDeviceBuilder, PrefabQueue };
use swapchain::{ HaSwapchain, SwapchainBuilder };
use pipeline::graphics::HaGraphicsPipeline;
use pipeline::graphics::builder::{ GraphicsPipelineConfig, GraphicsPipelineBuilder };
use pipeline::{ HaShaderInfo, HaInputAssembly, HaViewport };
use resources::command::pool::HaCommandPool;
use resources::command::buffer::HaCommandBuffer;
use resources::command::CommandBufferUsage;
use resources::command::HaCommandRecorder;
use resources::error::CommandError;

use procedure::window::ProgramEnv;
use procedure::error::ProcedureError;

use constant::core::VALIDATION;
use constant::swapchain::SWAPCHAIN_IMAGE_COUNT;

pub trait ProgramProc {

    // TODO: Redesign the API to support multi-pipeline
    fn configure_shaders(&self)     -> Vec<HaShaderInfo>;
    fn configure_inputs(&self)      -> HaInputAssembly;
//    fn configure_render_pass(&self) -> HaRenderPass;
    fn configure_commands(&self, buffer: &HaCommandRecorder, frame_index: usize) -> Result<(), CommandError>;
}

pub struct CoreInfrastructure<'win> {

    instance  : HaInstance,
    debugger  : Option<HaDebugger>,
    surface   : HaSurface<'win>,
    physical  : HaPhysicalDevice,
    device    : HaLogicalDevice,
    command_pool: HaCommandPool,
}

pub struct HaResources {

    swapchain : HaSwapchain,
    graphics_pipelines: Vec<HaGraphicsPipeline>,
    command_buffers: Vec<HaCommandBuffer>,
}

impl<'win, T> ProgramEnv<T> where T: ProgramProc {

    pub(super) fn initialize_core(&self, window: &'win winit::Window, requirement: PhysicalRequirement)
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

        let command_pool = HaCommandPool::setup(&device, &[])
            .map_err(|e| ProcedureError::Command(e))?;

        let core = CoreInfrastructure {
            instance,
            debugger,
            surface,
            physical,
            device,
            command_pool,
        };

        Ok(core)
    }

    pub(super) fn load_resources(&self, core: &CoreInfrastructure) -> Result<HaResources, ProcedureError> {

        // TODO: Currently just configuration a single pipeline.
        let shaders = self.procedure.configure_shaders();
        let inputs = self.procedure.configure_inputs();
        //        let render_pass = self.procedure.configure_render_pass();
        use pipeline::pass::render_pass::temp_render_pass;
        let render_pass = temp_render_pass(&core.device);

        let swapchain = SwapchainBuilder::init(&core.physical, &core.device, &core.surface)
            .map_err(|e| ProcedureError::SwapchainCreation(e))?
            .set_image_count(SWAPCHAIN_IMAGE_COUNT)
            .build(&core.instance, &render_pass)
            .map_err(|e| ProcedureError::SwapchainCreation(e))?;

        let viewport = HaViewport::setup(swapchain.extent);
        let mut pipeline_config = GraphicsPipelineConfig::init(shaders, inputs, render_pass);
        pipeline_config.setup_viewport(viewport);
        pipeline_config.finish_config();

        let mut pipeline_builder = GraphicsPipelineBuilder::init();
        pipeline_builder.add_config(pipeline_config);

        let graphics_pipelines = pipeline_builder.build(&core.device)
            .map_err(|e| ProcedureError::Pipeline(e))?;

        let mut command_buffers = core.command_pool
            .allocate(&core.device, CommandBufferUsage::UnitaryCommand, swapchain.framebuffers.len())
            .map_err(|e| ProcedureError::Command(e))?;
        for (frame_index, command_buffer) in command_buffers.iter_mut().enumerate() {
            // TODO: Fixed the configure to only one pipeline.
            let recorder = command_buffer.setup_record(&core.device, &swapchain, &graphics_pipelines[0])
                .map_err(|e| ProcedureError::Command(e))?;
            self.procedure.configure_commands(&recorder, frame_index)
                .map_err(|e| ProcedureError::Command(e))?;
        }

        let resources = HaResources {
            swapchain,
            graphics_pipelines,
            command_buffers,
        };
        Ok(resources)
    }
}

impl<'win> CoreInfrastructure<'win> {

    /// use cleanup function, so that the order of deinitialization can be customizable.
    pub fn cleanup(&self) {

        self.command_pool.cleanup(&self.device);
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

    pub fn cleanup(&self, core: &CoreInfrastructure) {
        self.graphics_pipelines.iter().for_each(|pipeline|{
            pipeline.clean(&core.device);
        });
        self.swapchain.cleanup(&core.device);
    }
}
