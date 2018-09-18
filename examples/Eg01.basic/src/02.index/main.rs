
#[macro_use]
extern crate hakurei_macros;
extern crate hakurei;

use hakurei::prelude::*;
use hakurei::prelude::config::*;
use hakurei::prelude::queue::*;
use hakurei::prelude::pipeline::*;
use hakurei::prelude::resources::*;
use hakurei::prelude::sync::*;
use hakurei::prelude::input::*;

use std::path::Path;

const WINDOW_TITLE: &'static str = "02.Index";
const WINDOW_WIDTH:  u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const VERTEX_SHADER_PATH  : &'static str = "shaders/index.vert.spv";
const FRAGMENT_SHADER_PATH: &'static str = "shaders/index.frag.spv";

define_input! {
    #[binding = 0, rate = vertex]
    struct Vertex {
        #[location = 0, format = vec2]
        pos:   [f32; 2],
        #[location = 1, format = vec4]
        color: [f32; 4],
    }
}

const VERTEX_DATA: [Vertex; 4] = [
    Vertex { pos: [-0.5, -0.5], color: [1.0, 0.0, 0.0, 1.0], },
    Vertex { pos: [ 0.5, -0.5], color: [0.0, 1.0, 0.0, 1.0], },
    Vertex { pos: [ 0.5,  0.5], color: [0.0, 0.0, 1.0, 1.0], },
    Vertex { pos: [-0.5,  0.5], color: [1.0, 1.0, 1.0, 1.0], },
];
const INDEX_DATA: [uint32_t; 6] = [
    0, 1, 2, 2, 3, 0,
];

struct DrawIndexProcedure {

    vertex_data   : Vec<Vertex>,
    index_data    : Vec<uint32_t>,

    buffer_storage: HaBufferRepository,
    vertex_item   : BufferSubItem,
    index_item    : BufferSubItem,

    graphics_pipeline: HaGraphicsPipeline,

    command_pool   : HaCommandPool,
    command_buffers: Vec<HaCommandBuffer>,

    present_availables: Vec<HaSemaphore>,
}

impl DrawIndexProcedure {

    fn new() -> DrawIndexProcedure {
        DrawIndexProcedure {
            vertex_data   : VERTEX_DATA.to_vec(),
            index_data    : INDEX_DATA.to_vec(),

            buffer_storage: HaBufferRepository::empty(),
            vertex_item   : BufferSubItem::unset(),
            index_item    : BufferSubItem::unset(),

            graphics_pipeline: HaGraphicsPipeline::uninitialize(),

            command_pool: HaCommandPool::uninitialize(),
            command_buffers: vec![],

            present_availables: vec![],
        }
    }
}

impl ProgramProc for DrawIndexProcedure {

    fn assets(&mut self, device: &HaLogicalDevice, generator: &ResourceGenerator) -> Result<(), ProcedureError> {

        // vertex buffer
        let mut vertex_buffer_config = BufferConfig::init(
            &[BufferUsageFlag::VertexBufferBit],
            &[
                MemoryPropertyFlag::HostVisibleBit,
                MemoryPropertyFlag::HostCoherentBit,
            ],
            &[]
        );
        let _ = vertex_buffer_config.add_item(data_size!(self.vertex_data, Vertex));

        // index buffer
        let mut index_buffer_config = BufferConfig::init(
            &[BufferUsageFlag::IndexBufferBit],
            &[
                MemoryPropertyFlag::HostVisibleBit,
                MemoryPropertyFlag::HostCoherentBit,
            ],
            &[]
        );
        let _ = index_buffer_config.add_item(data_size!(self.index_data, uint32_t));

        // allocate memory and transfer data.
        let mut buffer_allocator = generator.buffer();
        self.vertex_item = buffer_allocator.attach_buffer(vertex_buffer_config)?.pop().unwrap();
        self.index_item  = buffer_allocator.attach_buffer(index_buffer_config )?.pop().unwrap();

        self.buffer_storage = buffer_allocator.allocate()?;
        self.buffer_storage.tranfer_data(device, &self.vertex_data, &self.vertex_item)?;
        self.buffer_storage.tranfer_data(device, &self.index_data , &self.index_item)?;

        Ok(())
    }

    fn pipelines(&mut self, device: &HaLogicalDevice, swapchain: &HaSwapchain) -> Result<(), ProcedureError> {
        // shaders
        let vertex_shader = HaShaderInfo::setup(
            ShaderStageFlag::VertexStage,
            Path::new(VERTEX_SHADER_PATH),
            None);
        let fragment_shader = HaShaderInfo::setup(
            ShaderStageFlag::FragmentStage,
            Path::new(FRAGMENT_SHADER_PATH),
            None);
        let shader_infos = vec![
            vertex_shader,
            fragment_shader,
        ];
        let vertex_input_desc = Vertex::desc();

        // pipeline
        let mut render_pass_builder = RenderPassBuilder::new();
        let first_subpass = render_pass_builder.new_subpass(SubpassType::Graphics);

        let color_attachment = RenderAttachement::setup(RenderAttachementPrefab::Present, swapchain.format);
        let _attachment_index = render_pass_builder.add_attachemnt(color_attachment, first_subpass, AttachmentType::Color);

        let mut dependency = RenderDependency::setup(RenderDependencyPrefab::Common, SUBPASS_EXTERAL, first_subpass);
        dependency.set_stage(PipelineStageFlag::ColorAttachmentOutputBit, PipelineStageFlag::ColorAttachmentOutputBit);
        dependency.set_access(&[], &[
            AccessFlag::ColorAttachmentReadBit,
            AccessFlag::ColorAttachmentWriteBit,
        ]);
        render_pass_builder.add_dependenty(dependency);

        let render_pass = render_pass_builder.build(device, swapchain)?;
        let viewport = HaViewport::setup(swapchain.extent);
        let pipeline_config = GraphicsPipelineConfig::new(shader_infos, vertex_input_desc, render_pass)
            .setup_viewport(viewport)
            .finish_config();

        let mut pipeline_builder = GraphicsPipelineBuilder::init();
        pipeline_builder.add_config(pipeline_config);

        let mut graphics_pipelines = pipeline_builder.build(device)?;
        self.graphics_pipeline = graphics_pipelines.pop().unwrap();

        Ok(())
    }

    fn subresources(&mut self, device: &HaLogicalDevice) -> Result<(), ProcedureError> {

        // sync
        for _ in 0..self.graphics_pipeline.frame_count() {
            let present_available = HaSemaphore::setup(device)?;
            self.present_availables.push(present_available);
        }

        Ok(())
    }

    fn commands(&mut self, device: &HaLogicalDevice) -> Result<(), ProcedureError> {
        // command buffer
        let command_pool = HaCommandPool::setup(&device, DeviceQueueIdentifier::Graphics, &[])?;

        let command_buffer_count = self.graphics_pipeline.frame_count();
        let command_buffers = command_pool
            .allocate(device, CommandBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command_buffer) in command_buffers.iter().enumerate() {
            let recorder = command_buffer.setup_record(device);

            recorder.begin_record(&[CommandBufferUsageFlag::SimultaneousUseBit])?
                .begin_render_pass(&self.graphics_pipeline, frame_index)
                .bind_pipeline(&self.graphics_pipeline)
                .bind_vertex_buffers(0, &self.buffer_storage.vertex_binding_infos(&[&self.vertex_item]))
                .bind_index_buffers(&self.buffer_storage.index_binding_info(&self.index_item))
                .draw_indexed(self.index_data.len() as uint32_t, 1, 0, 0, 0)
                .end_render_pass()
                .finish()?;
        }
        self.command_pool    = command_pool;
        self.command_buffers = command_buffers;

        Ok(())
    }

    fn draw(&mut self, device: &HaLogicalDevice, device_available: &HaFence, image_available: &HaSemaphore, image_index: usize, _: f32)
            -> Result<&HaSemaphore, ProcedureError> {

        let submit_infos = [
            QueueSubmitBundle {
                wait_semaphores: &[image_available],
                sign_semaphores: &[&self.present_availables[image_index]],
                wait_stages    : &[PipelineStageFlag::ColorAttachmentOutputBit],
                commands       : &[&self.command_buffers[image_index]],
            },
        ];

        device.submit(&submit_infos, Some(device_available), DeviceQueueIdentifier::Graphics)?;

        return Ok(&self.present_availables[image_index])
    }

    fn clean_resources(&mut self, device: &HaLogicalDevice) -> Result<(), ProcedureError> {

        for semaphore in self.present_availables.iter() {
            semaphore.cleanup(device);
        }
        self.present_availables.clear();
        self.command_buffers.clear();

        self.graphics_pipeline.cleanup(device);
        self.command_pool.cleanup(device);

        Ok(())
    }

    fn cleanup(&mut self, device: &HaLogicalDevice) {

        for semaphore in self.present_availables.iter() {
            semaphore.cleanup(device);
        }

        self.graphics_pipeline.cleanup(device);
        self.command_pool.cleanup(device);
        self.buffer_storage.cleanup(device);
    }

    fn react_input(&mut self, inputer: &ActionNerve, _: f32) -> SceneAction {

        if inputer.is_key_pressed(HaKeycode::Escape) {
            return SceneAction::Terminal
        }

        SceneAction::Rendering
    }
}

fn main() {

    let procecure = DrawIndexProcedure::new();
    let mut config = EngineConfig::default();
    config.window.dimension = Dimension2D {
        width : WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
    };
    config.window.title = String::from(WINDOW_TITLE);

    let mut program = ProgramEnv::new(config, procecure);

    match program.launch() {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        }
    }
}
