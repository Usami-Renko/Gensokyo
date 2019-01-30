
// TODO: Remove all #[allow(dead_code)]

use ash::vk;
use gs::prelude::*;
use gsvk::prelude::common::*;
use gsvk::prelude::buffer::*;
use gsvk::prelude::pipeline::*;
use gsvk::prelude::command::*;
use gsvk::prelude::sync::*;
use gsvk::prelude::api::*;

use gsma::{ define_input, offset_of, vk_format, vertex_rate, data_size };

use std::path::{ Path, PathBuf };

const MANIFEST_PATH: &'static str = "src/01.triangle/Gensokyo.toml";
const VERTEX_SHADER_SPIRV_PATH  : &'static str = "src/01.triangle/triangle.vert.spv";
const FRAGMENT_SHADER_SPIRV_PATH: &'static str = "src/01.triangle/triangle.frag.spv";

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

struct TriangleProcedure {

    vertex_data: Vec<Vertex>,
    #[allow(dead_code)]
    vertex_storage: GsBufferRepository<Host>,
    vertex_buffer : GsVertexBuffer,

    graphics_pipeline: GsPipeline<Graphics>,

    command_pool   : GsCommandPool,
    command_buffers: Vec<GsCommandBuffer>,

    present_availables: Vec<GsSemaphore>,
}

impl TriangleProcedure {

    fn new(initializer: AssetInitializer) -> GsResult<TriangleProcedure> {

        let vertex_data = VERTEX_DATA.to_vec();

        let (vertex_buffer, vertex_storage) = {
            TriangleProcedure::assets(&initializer, &vertex_data)
        }?;

        let graphics_pipeline = {
            TriangleProcedure::pipelines(&initializer)
        }?;

        let present_availables = {
            TriangleProcedure::sync_resources(&initializer, &graphics_pipeline)
        }?;

        let (command_pool, command_buffers) = {
            TriangleProcedure::commands(&initializer, &graphics_pipeline, &vertex_buffer, &vertex_data)
        }?;

        let procedure = TriangleProcedure {
            vertex_data, vertex_storage, vertex_buffer,
            graphics_pipeline,
            command_pool, command_buffers,
            present_availables,
        };

        Ok(procedure)
    }

    fn assets(initializer: &AssetInitializer, vertex_data: &Vec<Vertex>) -> GsResult<(GsVertexBuffer, GsBufferRepository<Host>)> {

        // vertex buffer
        let mut vertex_allocator = GsBufferAllocator::new(initializer, BufferStorageType::HOST);

        let vertex_info = GsVertexBuffer::new(data_size!(Vertex), vertex_data.len());
        let vertex_index = vertex_allocator.assign(vertex_info)?;

        let buffer_distributor = vertex_allocator.allocate()?;
        let vertex_buffer = buffer_distributor.acquire(vertex_index);

        let mut vertex_storage = buffer_distributor.into_repository();

        vertex_storage.data_uploader()?
            .upload(&vertex_buffer, vertex_data)?
            .finish()?;

        Ok((vertex_buffer, vertex_storage))
    }

    fn pipelines(initializer: &AssetInitializer) -> GsResult<GsPipeline<Graphics>> {

        // shaders
        let vertex_shader = GsShaderCI::from_spirv(
            GsPipelineStage::VERTEX,
            Path::new(VERTEX_SHADER_SPIRV_PATH),
            None);
        let fragment_shader = GsShaderCI::from_spirv(
            GsPipelineStage::FRAGMENT,
            Path::new(FRAGMENT_SHADER_SPIRV_PATH),
            None);
        let shader_infos = vec![vertex_shader, fragment_shader];
        let vertex_input_desc = Vertex::desc();

        // pipeline
        let mut render_pass_builder = GsRenderPass::new(initializer);
        let first_subpass = render_pass_builder.new_subpass();

        let color_attachment = RenderAttachmentCI::<Present>::new(initializer);
        let _attachment_index = render_pass_builder.add_attachment(color_attachment, first_subpass);

        let dependency0 = RenderDependencyCI::new(SubpassStage::BeginExternal, SubpassStage::AtIndex(first_subpass))
            .stage(vk::PipelineStageFlags::BOTTOM_OF_PIPE, vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .access(vk::AccessFlags::MEMORY_READ, vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
            .with_flags(vk::DependencyFlags::BY_REGION);
        render_pass_builder.add_dependency(dependency0);

        let dependency1 = RenderDependencyCI::new(SubpassStage::AtIndex(first_subpass), SubpassStage::EndExternal)
            .stage(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT, vk::PipelineStageFlags::BOTTOM_OF_PIPE)
            .access(vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE, vk::AccessFlags::MEMORY_READ)
            .with_flags(vk::DependencyFlags::BY_REGION);
        render_pass_builder.add_dependency(dependency1);

        let render_pass = render_pass_builder.build()?;

        let pipeline_config = GfxPipelineConfig::new(shader_infos, vertex_input_desc, render_pass, initializer.screen_dimension())
            .finish();

        let mut pipeline_builder = GfxPipelineBuilder::new(initializer)?;
        let graphics_pipeline = pipeline_builder.build(pipeline_config)?;

        Ok(graphics_pipeline)
    }

    fn sync_resources(initializer: &AssetInitializer, pipeline: &GsPipeline<Graphics>) -> GsResult<Vec<GsSemaphore>> {

        // sync
        let mut present_availables = Vec::with_capacity(pipeline.frame_count());
        for _ in 0..pipeline.frame_count() {
            let semaphore = GsSemaphore::new(initializer)?;
            present_availables.push(semaphore);
        }
        Ok(present_availables)
    }

    fn commands(initializer: &AssetInitializer, pipeline: &GsPipeline<Graphics>, vertex_buffer: &GsVertexBuffer, data: &Vec<Vertex>) -> GsResult<(GsCommandPool, Vec<GsCommandBuffer>)> {

        let command_pool = GsCommandPool::new(initializer, DeviceQueueIdentifier::Graphics)?;
        let mut command_buffers = vec![];

        let command_buffer_count = pipeline.frame_count();
        let raw_commands = command_pool.allocate(CmdBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command) in raw_commands.into_iter().enumerate() {
            let mut recorder = GsCmdRecorder::<Graphics>::new(initializer, pipeline, command);

            recorder.begin_record(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE)?
                .begin_render_pass(pipeline, frame_index)
                .bind_pipeline()
                .bind_vertex_buffers(0, &[vertex_buffer])
                .draw(data.len() as vkuint, 1, 0, 0)
                .end_render_pass();

            let command_recorded = recorder.end_record()?;
            command_buffers.push(command_recorded);
        }

        Ok((command_pool, command_buffers))
    }
}

impl GraphicsRoutine for TriangleProcedure {

    fn draw(&mut self, device: &GsDevice, device_available: &GsFence, image_available: &GsSemaphore, image_index: usize, _: f32) -> GsResult<&GsSemaphore> {

        let submit_info = QueueSubmitBundle {
            wait_semaphores: &[image_available],
            sign_semaphores: &[&self.present_availables[image_index]],
            wait_stages    : &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT],
            commands       : &[&self.command_buffers[image_index]],
        };

        device.logic.submit_single(&submit_info, Some(device_available), DeviceQueueIdentifier::Graphics)?;

        return Ok(&self.present_availables[image_index])
    }

    fn reload_res(&mut self, initializer: AssetInitializer) -> GsResult<()> {

        self.graphics_pipeline = TriangleProcedure::pipelines(&initializer)?;

        self.present_availables = TriangleProcedure::sync_resources(&initializer, &self.graphics_pipeline)?;

        let (command_pool, command_buffers) = TriangleProcedure::commands(&initializer, &self.graphics_pipeline, &self.vertex_buffer, &self.vertex_data)?;
        self.command_pool = command_pool;
        self.command_buffers = command_buffers;

        Ok(())
    }

    fn react_input(&mut self, inputer: &ActionNerve, _: f32) -> SceneAction {

        if inputer.is_key_pressed(GsKeycode::ESCAPE) {
            return SceneAction::Terminal
        }

        SceneAction::Rendering
    }
}

fn main() {

    // TODO: handle unwrap().

    let manifest = PathBuf::from(MANIFEST_PATH);
    let mut program_context = ProgramContext::new(Some(manifest)).unwrap();

    let builder = program_context.routine().unwrap();

    let asset_loader = builder.assets_loader();
    let routine = TriangleProcedure::new(asset_loader).unwrap();
    let routine_flow = builder.build(routine);

    match routine_flow.launch(program_context) {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        },
    }
}
