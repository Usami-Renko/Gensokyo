
use winit;

use core::instance::HaInstance;
use core::debug::HaDebugger;
use core::physical::{ HaPhysicalDevice, PhysicalRequirement };
use core::surface::HaSurface;
use core::device::{ HaLogicalDevice, LogicalDeviceBuilder, PrefabQueue };
use core::device::QueueSubmitBundle;
use core::error::LogicalDeviceError;
use swapchain::{ HaSwapchain, SwapchainBuilder };
use swapchain::error::{ SwapchainError, SwapchainRuntimeError };
use pipeline::graphics::{ HaGraphicsPipeline, GraphicsPipelineConfig, GraphicsPipelineBuilder };
use pipeline::state::HaViewport;
use pipeline::shader::HaShaderInfo;
use pipeline::stages::PipelineStageFlag;
use resources::command::pool::HaCommandPool;
use resources::command::buffer::HaCommandBuffer;
use resources::command::CommandBufferUsage;
use resources::command::HaCommandRecorder;
use resources::error::CommandError;
use sync::{ HaFence, HaSemaphore };

use procedure::window::ProgramEnv;
use procedure::error::ProcedureError;

use constant::core::VALIDATION;
use constant::swapchain::SWAPCHAIN_IMAGE_COUNT;
use constant::sync::SYNCHRONOUT_FRAME;

use utility::time::TimePeriod;

pub trait ProgramProc {

    // TODO: Redesign the API to support multi-pipeline
    fn configure_shaders(&self)     -> Vec<HaShaderInfo>;
//    fn configure_render_pass(&self) -> HaRenderPass;
    fn configure_commands(&self, buffer: &HaCommandRecorder, frame_index: usize) -> Result<(), CommandError>;
}

pub struct CoreInfrastructure<'win> {

    instance  : HaInstance,
    debugger  : Option<HaDebugger>,
    surface   : HaSurface<'win>,
    physical  : HaPhysicalDevice,
    pub(super) device : HaLogicalDevice,
    command_pool: HaCommandPool,
}

pub struct HaResources {

    swapchain : HaSwapchain,
    graphics_pipelines: Vec<HaGraphicsPipeline>,
    command_buffers: Vec<HaCommandBuffer>,

    // sync
    image_awaits:  Vec<HaSemaphore>,
    render_awaits: Vec<HaSemaphore>,
    sync_fences:   Vec<HaFence>,
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
        //        let render_pass = self.procedure.configure_render_pass();
        use pipeline::pass::render_pass::temp_render_pass;
        let render_pass = temp_render_pass(&core.device);

        // swapchain
        let swapchain = SwapchainBuilder::init(&core.physical, &core.device, &core.surface)
            .map_err(|e| ProcedureError::Swapchain(SwapchainError::Init(e)))?
            .set_image_count(SWAPCHAIN_IMAGE_COUNT)
            .build(&core.instance, &render_pass)
            .map_err(|e| ProcedureError::Swapchain(SwapchainError::Init(e)))?;

        // pipeline
        let viewport = HaViewport::setup(swapchain.extent);
        let mut pipeline_config = GraphicsPipelineConfig::init(shaders, render_pass);
        pipeline_config.setup_viewport(viewport);
        pipeline_config.finish_config();

        let mut pipeline_builder = GraphicsPipelineBuilder::init();
        pipeline_builder.add_config(pipeline_config);

        let graphics_pipelines = pipeline_builder.build(&core.device)
            .map_err(|e| ProcedureError::Pipeline(e))?;

        // command buffers
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

        // sync
        let mut image_awaits  = vec![];
        let mut render_awaits = vec![];
        let mut sync_fences   = vec![];
        for _ in 0..SYNCHRONOUT_FRAME {
            let image_await = HaSemaphore::setup(&core.device)
                .map_err(|e| ProcedureError::Sync(e))?;
            let render_await = HaSemaphore::setup(&core.device)
                .map_err(|e| ProcedureError::Sync(e))?;
            let sync_fence = HaFence::setup(&core.device, true)
                .map_err(|e| ProcedureError::Sync(e))?;

            image_awaits.push(image_await);
            render_awaits.push(render_await);
            sync_fences.push(sync_fence);
        }

        let resources = HaResources {
            swapchain,
            graphics_pipelines,
            command_buffers,

            image_awaits,
            render_awaits,
            sync_fences,
        };
        Ok(resources)
    }

    pub(super) fn draw_frame(&mut self, current_frame: usize, core: &CoreInfrastructure, resources: &mut HaResources)
        -> Result<(), ProcedureError> {

        let fence_to_wait = &resources.sync_fences[current_frame];
        fence_to_wait.wait(&core.device, TimePeriod::Infinte)
            .map_err(|e| ProcedureError::Sync(e))?;

        let image_result = resources.swapchain.next_image(Some(&resources.image_awaits[current_frame]), None);
        let image_index = match image_result {
            | Ok(result) => result,
            | Err(e) =>
                match e {
                    | SwapchainRuntimeError::SurfaceOutOfDateError
                    | SwapchainRuntimeError::SurfaceSubOptimalError => {
                        resources.swapchain.recreate();
                        return Ok(())
                    }
                    | _ => return Err(ProcedureError::Swapchain(SwapchainError::Runtime(e)))
                }
        };

        fence_to_wait.reset(&core.device)
            .map_err(|e| ProcedureError::Sync(e))?;

        let submit_infos = [
            QueueSubmitBundle {
                wait_semaphores: &[&resources.image_awaits[current_frame]],
                sign_semaphores: &[&resources.render_awaits[current_frame]],
                wait_stages    : &[PipelineStageFlag::ColorAttachmentOutBit],
                commands       : &[&resources.command_buffers[current_frame]],
            },
        ];

        core.device.submit(&submit_infos, Some(fence_to_wait))
            .map_err(|e| ProcedureError::LogicalDevice(e))?;

        let graphics_queue = core.device.graphics_queue()
            .ok_or(ProcedureError::LogicalDevice(LogicalDeviceError::GraphicsQueueUnavailable))?;
        let present_result = resources.swapchain.present(&[&resources.render_awaits[current_frame]], image_index, graphics_queue);
        if let Err(e) = present_result {
            match e {
                | SwapchainRuntimeError::SurfaceOutOfDateError
                | SwapchainRuntimeError::SurfaceSubOptimalError => {
                    resources.swapchain.recreate();
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

    pub fn cleanup(&self, device: &HaLogicalDevice) {

        self.image_awaits.iter().for_each(|i| i.cleanup(device));
        self.render_awaits.iter().for_each(|r| r.cleanup(device));
        self.sync_fences.iter().for_each(|f| f.cleanup(device));
        self.graphics_pipelines.iter().for_each(|pipeline|{
            pipeline.cleanup(device);
        });
        self.swapchain.cleanup(device);
    }
}
