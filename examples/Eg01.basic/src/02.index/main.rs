
extern crate ash;
#[macro_use]
extern crate gensokyo_macros;
extern crate gensokyo_vulkan as gsvk;
extern crate gensokyo as gs;

use ash::vk;
use gs::prelude::*;
use gsvk::prelude::common::*;
use gsvk::prelude::buffer::*;
use gsvk::prelude::pipeline::*;
use gsvk::command::*;
use gsvk::sync::*;

use std::path::{ Path, PathBuf };

const MANIFEST_PATH: &str = "src/02.index/gensokyo.toml";
const VERTEX_SHADER_SOURCE_PATH  : &str = "src/02.index/index.vert";
const FRAGMENT_SHADER_SOURCE_PATH: &str = "src/02.index/index.frag";

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
const INDEX_DATA: [vkuint; 6] = [
    0, 1, 2, 2, 3, 0,
];

struct DrawIndexProcedure {

    index_data    : Vec<vkuint>,

    buffer_storage: GsBufferRepository<Cached>,
    vertex_buffer : GsVertexBlock,
    index_buffer  : GsIndexBlock,

    graphics_pipeline: GsGraphicsPipeline,

    command_pool   : GsCommandPool,
    command_buffers: Vec<GsCommandBuffer>,

    present_availables: Vec<GsSemaphore>,
}

impl DrawIndexProcedure {

    fn new(loader: AssetsLoader) -> Result<DrawIndexProcedure, ProcedureError> {

        let vertex_data = VERTEX_DATA.to_vec();
        let index_data    = INDEX_DATA.to_vec();

        let (vertex_buffer, index_buffer, buffer_storage) = loader.assets(|kit| {
            DrawIndexProcedure::assets(kit, &vertex_data, &index_data)
        })?;

        let graphics_pipeline = loader.pipelines(|kit| {
            DrawIndexProcedure::pipelines(kit)
        })?;

        let present_availables = loader.syncs(|kit| {
            DrawIndexProcedure::sync_resources(kit, &graphics_pipeline)
        })?;

        let (command_pool, command_buffers) = loader.commands(|kit| {
            DrawIndexProcedure::commands(kit, &graphics_pipeline, &vertex_buffer, &index_buffer, index_data.len())
        })?;

        let procecure = DrawIndexProcedure {
            index_data,
            buffer_storage, vertex_buffer, index_buffer,
            graphics_pipeline,
            command_pool, command_buffers,
            present_availables,
        };

        Ok(procecure)
    }

    fn assets(kit: AllocatorKit, vertex_data: &Vec<Vertex>, index_data: &Vec<vkuint>) -> Result<(GsVertexBlock, GsIndexBlock, GsBufferRepository<Cached>), ProcedureError> {

        // vertex & index buffer
        let mut buffer_allocator = kit.buffer(BufferStorageType::CACHED);

        let vertex_info = VertexBlockInfo::new(data_size!(vertex_data, Vertex));
        let vertex_index = buffer_allocator.append_buffer(vertex_info)?;

        let index_info = IndexBlockInfo::new(data_size!(index_data, vkuint));
        let index_index = buffer_allocator.append_buffer(index_info)?;

        let buffer_distributor = buffer_allocator.allocate()?;
        let vertex_buffer = buffer_distributor.acquire_vertex(vertex_index);
        let index_buffer = buffer_distributor.acquire_index(index_index);

        let mut buffer_storage = buffer_distributor.into_repository();

        buffer_storage.data_uploader()?
            .upload(&index_buffer, index_data)?
            .upload(&vertex_buffer, vertex_data)?
            .finish()?;

        Ok((vertex_buffer, index_buffer, buffer_storage))
    }

    fn pipelines(kit: PipelineKit) -> Result<GsGraphicsPipeline, ProcedureError> {

        // shaders
        let vertex_shader = GsShaderInfo::from_source(
            vk::ShaderStageFlags::VERTEX,
            Path::new(VERTEX_SHADER_SOURCE_PATH),
            None,
            "[Vertex Shader]");
        let fragment_shader = GsShaderInfo::from_source(
            vk::ShaderStageFlags::FRAGMENT,
            Path::new(FRAGMENT_SHADER_SOURCE_PATH),
            None,
            "[Fragment Shader]");
        let shader_infos = vec![
            vertex_shader,
            fragment_shader,
        ];
        let vertex_input_desc = Vertex::desc();

        // pipeline
        let mut render_pass_builder = kit.pass_builder();
        let first_subpass = render_pass_builder.new_subpass();

        let color_attachment = kit.subpass_attachment(RenderAttachementPrefab::PresentAttachment);
        let _attachment_index = render_pass_builder.add_attachemnt(color_attachment, first_subpass, AttachmentType::Color);

        let dependency = kit.subpass_dependency(vk::SUBPASS_EXTERNAL, first_subpass)
            .stage(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT, vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .access(vk::AccessFlags::empty(), vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE);
        render_pass_builder.add_dependenty(dependency);

        let render_pass = render_pass_builder.build()?;

        let pipeline_config = kit.pipeline_config(shader_infos, vertex_input_desc, render_pass)
            .finish();

        let mut pipeline_builder = kit.pipeline_graphics_builder()?;
        let pipeline_index = pipeline_builder.add_config(pipeline_config);

        let mut pipelines = pipeline_builder.build()?;
        let graphics_pipeline = pipelines.take_at(pipeline_index)?;

        Ok(graphics_pipeline)
    }

    fn sync_resources(kit: SyncKit, graphics_pipeline: &GsGraphicsPipeline) -> Result<Vec<GsSemaphore>, ProcedureError> {

        // sync
        let mut present_availables = vec![];
        for _ in 0..graphics_pipeline.frame_count() {
            let present_available = kit.semaphore()?;
            present_availables.push(present_available);
        }

        Ok(present_availables)
    }

    fn commands(kit: CommandKit, graphics_pipeline: &GsGraphicsPipeline, vertex_buffer: &GsVertexBlock, index_buffer: &GsIndexBlock, index_count: usize) -> Result<(GsCommandPool, Vec<GsCommandBuffer>), ProcedureError> {

        let command_pool = kit.pool(DeviceQueueIdentifier::Graphics)?;
        let mut command_buffers = vec![];

        let command_buffer_count = graphics_pipeline.frame_count();
        let raw_commands = command_pool
            .allocate(CmdBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command) in raw_commands.into_iter().enumerate() {
            let mut recorder = kit.recorder(command);

            recorder.begin_record(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE)?
                .begin_render_pass(graphics_pipeline, frame_index)
                .bind_pipeline(graphics_pipeline)
                .bind_vertex_buffers(0, &[vertex_buffer])
                .bind_index_buffer(index_buffer, 0)
                .draw_indexed(index_count as vkuint, 1, 0, 0, 0)
                .end_render_pass();

            let command_recorded = recorder.end_record()?;
            command_buffers.push(command_recorded);
        }

        Ok((command_pool, command_buffers))
    }
}


impl GraphicsRoutine for DrawIndexProcedure {

    fn draw(&mut self, device: &GsDevice, device_available: &GsFence, image_available: &GsSemaphore, image_index: usize, _: f32) -> Result<&GsSemaphore, ProcedureError> {

        let submit_infos = [
            QueueSubmitBundle {
                wait_semaphores: &[image_available],
                sign_semaphores: &[&self.present_availables[image_index]],
                wait_stages    : &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT],
                commands       : &[&self.command_buffers[image_index]],
            },
        ];

        device.submit(&submit_infos, Some(device_available), DeviceQueueIdentifier::Graphics)?;

        return Ok(&self.present_availables[image_index])
    }

    fn clean_resources(&mut self, _: &GsDevice) -> Result<(), ProcedureError> {

        self.present_availables.iter()
            .for_each(|semaphore| semaphore.cleanup());
        self.present_availables.clear();
        self.command_buffers.clear();
        self.command_pool.cleanup();
        self.graphics_pipeline.cleanup();

        Ok(())
    }

    fn reload_res(&mut self, loader: AssetsLoader) -> Result<(), ProcedureError> {

        self.graphics_pipeline = loader.pipelines(|kit| {
            DrawIndexProcedure::pipelines(kit)
        })?;

        self.present_availables = loader.syncs(|kit| {
            DrawIndexProcedure::sync_resources(kit, &self.graphics_pipeline)
        })?;

        let (command_pool, command_buffers) = loader.commands(|kit| {
            DrawIndexProcedure::commands(kit, &self.graphics_pipeline, &self.vertex_buffer, &self.index_buffer, self.index_data.len())
        })?;
        self.command_pool = command_pool;
        self.command_buffers = command_buffers;

        Ok(())
    }

    fn clean_routine(&mut self, _device: &GsDevice) {

        self.present_availables.iter()
            .for_each(|semaphore| semaphore.cleanup());
        self.graphics_pipeline.cleanup();
        self.command_pool.cleanup();
        self.buffer_storage.cleanup();
    }

    fn react_input(&mut self, inputer: &ActionNerve, _: f32) -> SceneAction {

        if inputer.is_key_pressed(GsKeycode::Escape) {
            return SceneAction::Terminal
        }

        SceneAction::Rendering
    }
}

fn main() {

    let manifest = PathBuf::from(MANIFEST_PATH);
    let mut program_env = ProgramEnv::new(Some(manifest)).unwrap();

    let mut routine_flow = {
        let builder = program_env.routine().unwrap();

        let asset_loader = builder.assets_loader();
        let routine = DrawIndexProcedure::new(asset_loader).unwrap();
        builder.build(routine)
    };

    match routine_flow.launch(program_env) {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        },
    }
}
