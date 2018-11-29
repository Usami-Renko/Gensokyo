
extern crate ash;
#[macro_use]
extern crate hakurei_macros;
extern crate hakurei_vulkan;
extern crate hakurei;

use ash::vk;
use hakurei::{
    procedure::{
        env::ProgramEnv,
        loader::AssetsLoader,
        workflow::GraphicsRoutine,
        error::ProcedureError,
    },
    toolkit::{ AllocatorKit, PipelineKit, CommandKit },
    input::{ ActionNerve, SceneAction, HaKeycode },
};

use hakurei_vulkan::{
    core::{
        device::{
            HaDevice, DeviceQueueIdentifier,
            queue::QueueSubmitBundle,
        },
        swapchain::HaSwapchain
    },
    buffer::{
        HaBufferRepository,
        allocator::types::BufferStorageType,
        instance::{ HaVertexBlock, VertexBlockInfo },
    },
    memory::types::Host,
    pipeline::{
        graphics::{ HaGraphicsPipeline, GraphicsPipelineConfig },
        shader::{ HaShaderInfo, VertexInputDescription, HaVertexInputBinding, HaVertexInputAttribute },
        state::viewport::{ HaViewportState, ViewportStateInfo, ViewportStateType },
        pass::{ RenderAttachement, RenderAttachementPrefab, AttachmentType, RenderDependency },
    },
    command::{ HaCommandPool, HaCommandBuffer, CmdBufferUsage },
    sync::{ HaSemaphore, HaFence },
    types::{ vkuint, vkbytes },
};

use std::path::{ Path, PathBuf };

const MANIFEST_PATH: &str = "src/01.triangle/hakurei.toml";
const VERTEX_SHADER_SPIRV_PATH  : &str = "src/01.triangle/triangle.vert.spv";
const FRAGMENT_SHADER_SPIRV_PATH: &str = "src/01.triangle/triangle.frag.spv";

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
    vertex_storage: HaBufferRepository<Host>,
    vertex_buffer : HaVertexBlock,

    graphics_pipeline: HaGraphicsPipeline,

    command_pool   : HaCommandPool,
    command_buffers: Vec<HaCommandBuffer>,

    present_availables: Vec<HaSemaphore>,
}

impl TriangleProcedure {

    fn new(loader: AssetsLoader) -> Result<TriangleProcedure, ProcedureError> {

        let vertex_data = VERTEX_DATA.to_vec();

        let (vertex_buffer, vertex_storage) = loader.assets(|kit| {
            TriangleProcedure::assets(kit, &vertex_data)
        })?;

        let graphics_pipeline = loader.pipelines(|kit, swapchain| {
            TriangleProcedure::pipelines(kit, swapchain)
        })?;

        let present_availables = loader.subresources(|device| {
            TriangleProcedure::subresources(device, &graphics_pipeline)
        })?;

        let (command_pool, command_buffers) = loader.commands(|kit| {
            TriangleProcedure::commands(kit, &graphics_pipeline, &vertex_buffer, &vertex_data)
        })?;

        let procecure = TriangleProcedure {
            vertex_data, vertex_storage, vertex_buffer,
            graphics_pipeline,
            command_pool, command_buffers,
            present_availables,
        };

        Ok(procecure)
    }

    fn assets(kit: AllocatorKit, vertex_data: &Vec<Vertex>) -> Result<(HaVertexBlock, HaBufferRepository<Host>), ProcedureError> {

        // vertex buffer
        let mut vertex_allocator = kit.buffer(BufferStorageType::HOST);

        let vertex_info = VertexBlockInfo::new(data_size!(vertex_data, Vertex));
        let block_index = vertex_allocator.append_buffer(vertex_info)?;

        let buffer_distributor = vertex_allocator.allocate()?;
        let vertex_buffer = buffer_distributor.acquire_vertex(block_index);

        let mut vertex_storage = buffer_distributor.into_repository();

        vertex_storage.data_uploader()?
            .upload(&vertex_buffer, vertex_data)?
            .finish()?;

        Ok((vertex_buffer, vertex_storage))
    }

    fn pipelines(kit: PipelineKit, swapchain: &HaSwapchain) -> Result<HaGraphicsPipeline, ProcedureError> {

        // shaders
        let vertex_shader = HaShaderInfo::from_spirv(
            vk::ShaderStageFlags::VERTEX,
            Path::new(VERTEX_SHADER_SPIRV_PATH),
            None);
        let fragment_shader = HaShaderInfo::from_spirv(
            vk::ShaderStageFlags::FRAGMENT,
            Path::new(FRAGMENT_SHADER_SPIRV_PATH),
            None);
        let shader_infos = vec![
            vertex_shader,
            fragment_shader,
        ];
        let vertex_input_desc = Vertex::desc();

        // pipeline
        let mut render_pass_builder = kit.pass_builder();
        let first_subpass = render_pass_builder.new_subpass();

        let color_attachment = RenderAttachement::setup(RenderAttachementPrefab::BackColorAttachment, swapchain.format())
            .layout(vk::ImageLayout::UNDEFINED, vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL, vk::ImageLayout::PRESENT_SRC_KHR);
        let _attachment_index = render_pass_builder.add_attachemnt(color_attachment, first_subpass, AttachmentType::Color);

        let dependency = RenderDependency::setup(vk::SUBPASS_EXTERNAL, first_subpass)
            .stage(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT, vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .access(vk::AccessFlags::empty(), vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE);
        render_pass_builder.add_dependenty(dependency);

        let render_pass = render_pass_builder.build(swapchain)?;
        let viewport = HaViewportState::single(ViewportStateInfo::new(swapchain.extent()));
        let pipeline_config = GraphicsPipelineConfig::new(shader_infos, vertex_input_desc, render_pass)
            .setup_viewport(ViewportStateType::Fixed { state: viewport })
            .finish();

        let mut pipeline_builder = kit.pipeline_graphics_builder()?;
        let pipeline_index = pipeline_builder.add_config(pipeline_config);

        let mut pipelines = pipeline_builder.build()?;
        let graphics_pipeline = pipelines.take_at(pipeline_index)?;

        Ok(graphics_pipeline)
    }

    fn subresources(device: &HaDevice, graphics_pipeline: &HaGraphicsPipeline) -> Result<Vec<HaSemaphore>, ProcedureError> {

        // sync
        let mut present_availables = vec![];
        for _ in 0..graphics_pipeline.frame_count() {
            let present_available = HaSemaphore::setup(device)?;
            present_availables.push(present_available);
        }

        Ok(present_availables)
    }

    fn commands(kit: CommandKit, graphics_pipeline: &HaGraphicsPipeline, vertex_buffer: &HaVertexBlock, data: &Vec<Vertex>) -> Result<(HaCommandPool, Vec<HaCommandBuffer>), ProcedureError> {

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
                .draw(data.len() as vkuint, 1, 0, 0)
                .end_render_pass();

            let command_recorded = recorder.end_record()?;
            command_buffers.push(command_recorded);
        }

        Ok((command_pool, command_buffers))
    }
}

impl GraphicsRoutine for TriangleProcedure {

    fn draw(&mut self, device: &HaDevice, device_available: &HaFence, image_available: &HaSemaphore, image_index: usize, _: f32) -> Result<&HaSemaphore, ProcedureError> {

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

    fn clean_resources(&mut self, _: &HaDevice) -> Result<(), ProcedureError> {

        self.present_availables.iter()
            .for_each(|semaphore| semaphore.cleanup());
        self.present_availables.clear();
        self.command_buffers.clear();
        self.graphics_pipeline.cleanup();
        self.command_pool.cleanup();

        Ok(())
    }

    fn reload_res(&mut self, loader: AssetsLoader) -> Result<(), ProcedureError> {

        self.graphics_pipeline = loader.pipelines(|kit, swapchain| {
            TriangleProcedure::pipelines(kit, swapchain)
        })?;

        self.present_availables = loader.subresources(|device| {
            TriangleProcedure::subresources(device, &self.graphics_pipeline)
        })?;

        let (command_pool, command_buffers) = loader.commands(|kit| {
            TriangleProcedure::commands(kit, &self.graphics_pipeline, &self.vertex_buffer, &self.vertex_data)
        })?;
        self.command_pool = command_pool;
        self.command_buffers = command_buffers;

        Ok(())
    }

    fn clean_routine(&mut self, _device: &HaDevice) {

        self.present_availables.iter()
            .for_each(|semaphore| semaphore.cleanup());
        self.graphics_pipeline.cleanup();
        self.command_pool.cleanup();
        self.vertex_storage.cleanup();
    }

    fn react_input(&mut self, inputer: &ActionNerve, _: f32) -> SceneAction {

        if inputer.is_key_pressed(HaKeycode::Escape) {
            return SceneAction::Terminal
        }

        SceneAction::Rendering
    }
}

fn main() {

    // TODO: handle unwrap().

    let manifest = PathBuf::from(MANIFEST_PATH);
    let mut program_env = ProgramEnv::new(Some(manifest)).unwrap();

    let mut routine_flow = {
        let builder = program_env.routine().unwrap();

        let routine = {
            let asset_loader = builder.assets_loader();

            TriangleProcedure::new(asset_loader).unwrap()
        };
        builder.build(routine)
    };

    match routine_flow.launch(program_env) {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        }
    }
}
