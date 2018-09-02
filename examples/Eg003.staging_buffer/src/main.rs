
#[macro_use]
extern crate hakurei;

use hakurei::prelude::*;
use hakurei::pipeline::shader::prelude::*;
use hakurei::pipeline::graphics::prelude::*;
use hakurei::pipeline::pass::prelude::*;
use hakurei::pipeline::state::prelude::*;
use hakurei::resources::command::*;
use hakurei::resources::allocator::*;
use hakurei::resources::buffer::*;
use hakurei::resources::memory::*;
use hakurei::resources::repository::*;
use hakurei::sync::prelude::*;

use std::path::Path;

const WINDOW_TITLE: &'static str = "Staging Buffer Example";
const WINDOW_WIDTH:  u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

define_input! {
    #[binding = 0, rate = vertex]
    struct Vertex {
        #[location = 0, format = vec2]
        pos:   [f32; 2],
        #[location = 1, format = vec4]
        color: [f32; 4],
    }
}

const VERTEX_DATA: [Vertex; 3] = [
    Vertex { pos: [ 0.0, -0.5], color: [1.0, 0.0, 0.0, 1.0], },
    Vertex { pos: [ 0.5,  0.5], color: [0.0, 1.0, 0.0, 1.0], },
    Vertex { pos: [-0.5,  0.5], color: [0.0, 0.0, 1.0, 1.0], },
];

struct StagingBufferProcedure {

    vertex_data  : Vec<Vertex>,
    vertex_buffer: HaBufferRepository,
    vertex_item  : BufferItem,

    graphics_pipeline: HaGraphicsPipeline,

    command_pool   : HaCommandPool,
    command_buffers: Vec<HaCommandBuffer>,

    present_availables: Vec<HaSemaphore>,
}

impl StagingBufferProcedure {

    fn new() -> StagingBufferProcedure {
        StagingBufferProcedure {
            vertex_data  : VERTEX_DATA.to_vec(),
            vertex_buffer: HaBufferRepository::empty(),
            vertex_item  : BufferItem::unset(),

            graphics_pipeline: HaGraphicsPipeline::uninitialize(),

            command_pool: HaCommandPool::uninitialize(),
            command_buffers: vec![],

            present_availables: vec![],
        }
    }
}

impl ProgramProc for StagingBufferProcedure {

    fn configure_storage(&mut self, device: &HaLogicalDevice, generator: &ResourceGenerator) -> Result<(), ProcedureError> {

        // vertex buffer
        let mut staging_buffer_config = BufferConfig::init(
            &[BufferUsageFlag::TransferSrcBit],
            &[
                MemoryPropertyFlag::HostVisibleBit,
                MemoryPropertyFlag::HostCoherentBit,
            ],
            &[]
        );
        let _ = staging_buffer_config.add_item(data_size!(self.vertex_data, Vertex));

        let mut vertex_buffer_config = BufferConfig::init(
            &[
                BufferUsageFlag::TransferDstBit,
                BufferUsageFlag::VertexBufferBit,
            ],
            &[MemoryPropertyFlag::DeviceLocalBit],
            &[]
        );
        let _ = vertex_buffer_config.add_item(data_size!(self.vertex_data, Vertex));

        let mut staging_allocator = generator.buffer_allocator();
        let stage_buffer_item = staging_allocator.attach_buffer(staging_buffer_config)?.pop().unwrap();

        let staging_repository = staging_allocator.allocate()?;
        staging_repository.tranfer_data(device, &self.vertex_data, &stage_buffer_item)?;

        let mut vertex_allocator = generator.buffer_allocator();
        self.vertex_item = vertex_allocator.attach_buffer(vertex_buffer_config)?.pop().unwrap();
        self.vertex_buffer = vertex_allocator.allocate()?;
        self.vertex_buffer.copy_data(
            device,
            &staging_repository,
            &stage_buffer_item,
            &self.vertex_item)?;
        staging_repository.cleanup(device);

        Ok(())
    }

    fn configure_pipeline(&mut self, device: &HaLogicalDevice, swapchain: &HaSwapchain) -> Result<(), ProcedureError> {
        // shaders
        let vertex_shader = HaShaderInfo::setup(
            ShaderStageFlag::VertexStage,
            Path::new("shaders/staging.vert.spv"),
            None);
        let fragment_shader = HaShaderInfo::setup(
            ShaderStageFlag::FragmentStage,
            Path::new("shaders/staging.frag.spv"),
            None);
        let shader_infos = vec![
            vertex_shader,
            fragment_shader,
        ];
        let vertex_input_desc = Vertex::desc();

        // pipeline
        let mut render_pass_builder = RenderPassBuilder::new();
        let first_subpass = render_pass_builder.new_subpass(SubpassType::Graphics);

        let color_attachment = RenderAttachement::setup(RenderAttachementPrefab::Common, swapchain.format);
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
        let pipeline_config = GraphicsPipelineConfig::init(shader_infos, vertex_input_desc, render_pass)
            .setup_viewport(viewport)
            .finish_config();

        let mut pipeline_builder = GraphicsPipelineBuilder::init();
        pipeline_builder.add_config(pipeline_config);

        let mut graphics_pipelines = pipeline_builder.build(device)?;
        self.graphics_pipeline = graphics_pipelines.pop().unwrap();

        Ok(())
    }

    fn configure_resources(&mut self, device: &HaLogicalDevice) -> Result<(), ProcedureError> {
        // sync
        for _ in 0..self.graphics_pipeline.frame_count() {
            let present_available = HaSemaphore::setup(device)?;
            self.present_availables.push(present_available);
        }
        Ok(())
    }

    fn configure_commands(&mut self, device: &HaLogicalDevice) -> Result<(), ProcedureError> {
        // command buffer
        let command_pool = HaCommandPool::setup(&device, &[])?;

        let command_buffer_count = self.graphics_pipeline.frame_count();
        let mut command_buffers = command_pool
            .allocate(device, CommandBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command_buffer) in command_buffers.iter_mut().enumerate() {
            let recorder = command_buffer.setup_record(device);

            recorder.begin_record(&[CommandBufferUsageFlag::SimultaneousUseBit])?
                .begin_render_pass(&self.graphics_pipeline, frame_index)
                .bind_pipeline(&self.graphics_pipeline)
                .bind_vertex_buffers(0, &self.vertex_buffer.vertex_binding_infos(&[&self.vertex_item]))
                .draw(self.vertex_data.len() as uint32_t, 1, 0, 0)
                .end_render_pass()
                .finish()?;
        }
        self.command_pool    = command_pool;
        self.command_buffers = command_buffers;

        Ok(())
    }

    fn draw(&mut self, device: &HaLogicalDevice, device_available: &HaFence, image_available: &HaSemaphore, image_index: usize)
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

    fn cleanup(&self, device: &HaLogicalDevice) {

        for semaphore in self.present_availables.iter() {
            semaphore.cleanup(device);
        }

        self.graphics_pipeline.cleanup(device);
        self.command_pool.cleanup(device);
        self.vertex_buffer.cleanup(device);
    }
}

fn main() {

    let procecure = StagingBufferProcedure::new();
    let mut program = ProgramBuilder::new(procecure)
        .title(WINDOW_TITLE)
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .build();

    match program.launch() {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        }
    }
}
