
extern crate ash;
#[macro_use]
extern crate hakurei_macros;
extern crate hakurei_vulkan;
extern crate hakurei;

use ash::vk;
use hakurei::{
    procedure::{
        loops::ProgramEnv,
        workflow::ProgramProc,
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
        allocator::types::{ BufferStorageType, Host },
        instance::{ HaVertexBlock, VertexBlockInfo },
    },
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

// TODO: Fix all the unwrap.

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
    vertex_storage: Option<HaBufferRepository<Host>>,
    vertex_buffer : Option<HaVertexBlock>,

    graphics_pipeline: Option<HaGraphicsPipeline>,

    command_pool   : Option<HaCommandPool>,
    command_buffers: Vec<HaCommandBuffer>,

    present_availables: Vec<HaSemaphore>,
}

impl TriangleProcedure {

    fn new() -> TriangleProcedure {
        TriangleProcedure {
            vertex_data: VERTEX_DATA.to_vec(),
            vertex_storage: None,
            vertex_buffer : None,

            graphics_pipeline: None,

            command_pool: None,
            command_buffers: vec![],

            present_availables: vec![],
        }
    }
}

impl ProgramProc for TriangleProcedure {

    fn assets(&mut self, kit: AllocatorKit) -> Result<(), ProcedureError> {

        // vertex buffer
        let mut vertex_allocator = kit.buffer(BufferStorageType::HOST);

        let vertex_info = VertexBlockInfo::new(data_size!(self.vertex_data, Vertex));
        let block_index = vertex_allocator.append_buffer(vertex_info)?;

        let buffer_distributor = vertex_allocator.allocate()?;
        self.vertex_buffer = Some(buffer_distributor.acquire_vertex(block_index));

        self.vertex_storage = Some(buffer_distributor.into_repository());

        self.vertex_storage.as_mut().unwrap().data_uploader()?
            .upload(self.vertex_buffer.as_ref().unwrap(), &self.vertex_data)?
            .done()?;

        Ok(())
    }

    fn pipelines(&mut self, kit: PipelineKit, swapchain: &HaSwapchain) -> Result<(), ProcedureError> {

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
        self.graphics_pipeline = Some(pipelines.take_at(pipeline_index)?);

        Ok(())
    }

    fn subresources(&mut self, device: &HaDevice) -> Result<(), ProcedureError> {
        // sync
        for _ in 0..self.graphics_pipeline.as_ref().unwrap().frame_count() {
            let present_available = HaSemaphore::setup(device)?;
            self.present_availables.push(present_available);
        }

        Ok(())
    }

    fn commands(&mut self, kit: CommandKit) -> Result<(), ProcedureError> {

        self.command_pool = Some(kit.pool(DeviceQueueIdentifier::Graphics)?);

        let command_buffer_count = self.graphics_pipeline.as_ref().unwrap().frame_count();
        let raw_commands = self.command_pool.as_ref().unwrap()
            .allocate(CmdBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command) in raw_commands.into_iter().enumerate() {
            let mut recorder = kit.recorder(command);

            recorder.begin_record(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE)?
                .begin_render_pass(&self.graphics_pipeline.as_ref().unwrap(), frame_index)
                .bind_pipeline(&self.graphics_pipeline.as_ref().unwrap())
                .bind_vertex_buffers(0, &[self.vertex_buffer.as_ref().unwrap()])
                .draw(self.vertex_data.len() as vkuint, 1, 0, 0)
                .end_render_pass();

            let command_recorded = recorder.end_record()?;
            self.command_buffers.push(command_recorded);
        }

        Ok(())
    }

    fn draw(&mut self, device: &HaDevice, device_available: &HaFence, image_available: &HaSemaphore, image_index: usize, _: f32)
            -> Result<&HaSemaphore, ProcedureError> {

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
        self.graphics_pipeline.as_ref().unwrap().cleanup();
        self.command_pool.as_ref().unwrap().cleanup();

        Ok(())
    }

    fn cleanup(&mut self, _: &HaDevice) {

        self.present_availables.iter()
            .for_each(|semaphore| semaphore.cleanup());
        self.graphics_pipeline.as_ref().unwrap().cleanup();
        self.command_pool.as_ref().unwrap().cleanup();
        self.vertex_storage.as_mut().unwrap().cleanup();
    }

    fn react_input(&mut self, inputer: &ActionNerve, _: f32) -> SceneAction {

        if inputer.is_key_pressed(HaKeycode::Escape) {
            return SceneAction::Terminal
        }

        SceneAction::Rendering
    }
}

fn main() {

    let procecure = TriangleProcedure::new();

    let manifest = PathBuf::from(MANIFEST_PATH);
    // TODO: handle the Result.
    let mut program = ProgramEnv::new(Some(manifest), procecure).unwrap();

    match program.launch() {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        }
    }
}
