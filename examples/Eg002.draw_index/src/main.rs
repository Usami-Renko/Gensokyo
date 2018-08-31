
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

const WINDOW_TITLE: &'static str = "Index Drawing Example";
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

    vertex_data  : Vec<Vertex>,
    index_data   : Vec<uint32_t>,
    vertex_buffer: HaBufferRepository,
    index_buffer : HaBufferRepository,

    graphics_pipeline: HaGraphicsPipeline,

    command_pool   : HaCommandPool,
    command_buffers: Vec<HaCommandBuffer>,

    present_availables: Vec<HaSemaphore>,
}

impl DrawIndexProcedure {

    fn new() -> DrawIndexProcedure {
        DrawIndexProcedure {
            vertex_data  : VERTEX_DATA.to_vec(),
            index_data   : INDEX_DATA.to_vec(),
            vertex_buffer: HaBufferRepository::empty(),
            index_buffer : HaBufferRepository::empty(),

            graphics_pipeline: HaGraphicsPipeline::uninitialize(),

            command_pool: HaCommandPool::uninitialize(),
            command_buffers: vec![],

            present_availables: vec![],
        }
    }
}

impl ProgramProc for DrawIndexProcedure {

    fn configure_pipeline(&mut self, device: &HaLogicalDevice, swapchain: &HaSwapchain) -> Result<(), ProcedureError> {
        // shaders
        let vertex_shader = HaShaderInfo::setup(
            ShaderStageType::VertexStage,
            Path::new("shaders/index.vert.spv"),
            None);
        let fragment_shader = HaShaderInfo::setup(
            ShaderStageType::FragmentStage,
            Path::new("shaders/index.frag.spv"),
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

    fn configure_resources(&mut self, device: &HaLogicalDevice, generator: &ResourceGenerator) -> Result<(), ProcedureError> {

        // vertex buffer
        let staging_buffer_config = BufferConfig {
            estimate_size: data_size!(self.vertex_data, Vertex),
            usages: &[BufferUsageFlag::TransferSrcBit],
            buffer_flags: &[],
            memory_flags: &[
                MemoryPropertyFlag::HostVisibleBit,
                MemoryPropertyFlag::HostCoherentBit,
            ],
        };

        let vertex_buffer_config = BufferConfig {
            estimate_size: data_size!(self.vertex_data, Vertex),
            usages: &[
                BufferUsageFlag::TransferDstBit,
                BufferUsageFlag::VertexBufferBit,
            ],
            buffer_flags: &[],
            memory_flags: &[MemoryPropertyFlag::DeviceLocalBit],
        };

        let mut staging_allocator = generator.buffer_allocator();
        let staging_buffer_index = staging_allocator.attach_buffer(staging_buffer_config)?;

        let staging_repository = staging_allocator.allocate()?;
        staging_repository.tranfer_data(device, &self.vertex_data, staging_buffer_index)?;

        let mut vertex_allocator = generator.buffer_allocator();
        let vertex_buffer_index = vertex_allocator.attach_buffer(vertex_buffer_config)?;
        self.vertex_buffer = vertex_allocator.allocate()?;
        self.vertex_buffer.copy_data(device, &staging_repository, staging_buffer_index, vertex_buffer_index)?;
        staging_repository.cleanup(device);

        // index buffer
        let staging_buffer_config = BufferConfig {
            estimate_size: data_size!(self.index_data, uint32_t),
            usages: &[BufferUsageFlag::TransferSrcBit],
            buffer_flags: &[],
            memory_flags: &[
                MemoryPropertyFlag::HostVisibleBit,
                MemoryPropertyFlag::HostCoherentBit,
            ],
        };

        let index_buffer_config = BufferConfig {
            estimate_size: data_size!(self.index_data, uint32_t),
            usages: &[
                BufferUsageFlag::TransferDstBit,
                BufferUsageFlag::IndexBufferBit,
            ],
            buffer_flags: &[],
            memory_flags: &[MemoryPropertyFlag::DeviceLocalBit],
        };
        let mut staging_allocator = generator.buffer_allocator();
        let staging_buffer_index = staging_allocator.attach_buffer(staging_buffer_config)?;

        let staging_repository = staging_allocator.allocate()?;
        staging_repository.tranfer_data(device, &self.index_data, staging_buffer_index)?;

        let mut index_allocator = generator.buffer_allocator();
        let index_buffer_index = index_allocator.attach_buffer(index_buffer_config)?;
        self.index_buffer = index_allocator.allocate()?;
        self.index_buffer.copy_data(device, &staging_repository, staging_buffer_index, index_buffer_index)?;
        staging_repository.cleanup(device);

        // command buffer
        let command_pool = HaCommandPool::setup(&device, &[])?;

        let command_buffer_count = self.graphics_pipeline.frame_count();
        let mut command_buffers = command_pool
            .allocate(device, CommandBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command_buffer) in command_buffers.iter_mut().enumerate() {
            let recorder = command_buffer.setup_record(device);
            let usage_flags = [
                CommandBufferUsageFlag::SimultaneousUseBit
            ];

            recorder.begin_record(&usage_flags)?
                .begin_render_pass(&self.graphics_pipeline, frame_index)
                .bind_pipeline(&self.graphics_pipeline)
                .bind_vertex_buffers(0, &self.vertex_buffer.vertex_binding_infos(&[vertex_buffer_index]))
                .bind_index_buffers(&self.index_buffer.index_binding_info(index_buffer_index))
                .draw_indexed(self.index_data.len() as uint32_t, 1, 0, 0, 0)
                .end_render_pass()
                .finish()?;
        }
        self.command_pool    = command_pool;
        self.command_buffers = command_buffers;

        // sync
        for _ in 0..self.graphics_pipeline.frame_count() {
            let present_available = HaSemaphore::setup(device)?;
            self.present_availables.push(present_available);
        }

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
        self.index_buffer.cleanup(device);
        self.vertex_buffer.cleanup(device);
    }
}

fn main() {

    let procecure = DrawIndexProcedure::new();
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
