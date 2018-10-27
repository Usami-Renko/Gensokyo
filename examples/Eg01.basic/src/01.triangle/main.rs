
#[macro_use]
extern crate hakurei_macros;
extern crate hakurei;

use hakurei::prelude::*;
use hakurei::prelude::queue::*;
use hakurei::prelude::pipeline::*;
use hakurei::prelude::resources::*;
use hakurei::prelude::sync::*;
use hakurei::prelude::input::*;

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
    vertex_storage: HaBufferRepository,
    vertex_buffer : HaVertexBlock,

    graphics_pipeline: HaGraphicsPipeline,

    command_pool   : HaCommandPool,
    command_buffers: Vec<HaCommandBuffer>,

    present_availables: Vec<HaSemaphore>,
}

impl TriangleProcedure {

    fn new() -> TriangleProcedure {
        TriangleProcedure {
            vertex_data: VERTEX_DATA.to_vec(),
            vertex_storage: HaBufferRepository::empty(),
            vertex_buffer : HaVertexBlock::uninitialize(),

            graphics_pipeline: HaGraphicsPipeline::uninitialize(),

            command_pool: HaCommandPool::uninitialize(),
            command_buffers: vec![],

            present_availables: vec![],
        }
    }
}

impl ProgramProc for TriangleProcedure {

    fn assets(&mut self, kit: AllocatorKit) -> Result<(), ProcedureError> {

        // vertex buffer
        let mut vertex_allocator = kit.buffer(BufferStorageType::Cached);

        let vertex_info = VertexBlockInfo::new(data_size!(self.vertex_data, Vertex));
        self.vertex_buffer = vertex_allocator.append_vertex(vertex_info)?;

        self.vertex_storage = vertex_allocator.allocate()?;

        self.vertex_storage.data_uploader()?
            .upload(&self.vertex_buffer, &self.vertex_data)?
            .done()?;

        Ok(())
    }

    fn pipelines(&mut self, kit: PipelineKit, swapchain: &HaSwapchain) -> Result<(), ProcedureError> {
        // shaders
        let vertex_shader = HaShaderInfo::from_spirv(
            ShaderStageFlag::VertexStage,
            Path::new(VERTEX_SHADER_SPIRV_PATH),
            None);
        let fragment_shader = HaShaderInfo::from_spirv(
            ShaderStageFlag::FragmentStage,
            Path::new(FRAGMENT_SHADER_SPIRV_PATH),
            None);
        let shader_infos = vec![
            vertex_shader,
            fragment_shader,
        ];
        let vertex_input_desc = Vertex::desc();

        // pipeline
        let mut render_pass_builder = kit.pass_builder();
        let first_subpass = render_pass_builder.new_subpass(PipelineType::Graphics);

        let color_attachment = RenderAttachement::setup(RenderAttachementPrefab::BackColorAttachment, swapchain.format);
        let _attachment_index = render_pass_builder.add_attachemnt(color_attachment, first_subpass, AttachmentType::Color);

        let mut dependency = RenderDependency::setup(RenderDependencyPrefab::Common, SUBPASS_EXTERAL, first_subpass);
        dependency.set_stage(PipelineStageFlag::ColorAttachmentOutputBit, PipelineStageFlag::ColorAttachmentOutputBit);
        dependency.set_access(&[], &[
            AccessFlag::ColorAttachmentReadBit,
            AccessFlag::ColorAttachmentWriteBit,
        ]);
        render_pass_builder.add_dependenty(dependency);

        let render_pass = render_pass_builder.build(swapchain)?;
        let viewport = HaViewportState::single(ViewportStateInfo::new(swapchain.extent));
        let pipeline_config = GraphicsPipelineConfig::new(shader_infos, vertex_input_desc, render_pass)
            .setup_viewport(ViewportStateType::Fixed { state: viewport })
            .finish_config();

        let mut pipeline_builder = kit.pipeline_builder(PipelineType::Graphics)?;
        let pipeline_index = pipeline_builder.add_config(pipeline_config);

        let mut pipelines = pipeline_builder.build()?;
        self.graphics_pipeline = pipelines.take_at(pipeline_index)?;

        Ok(())
    }

    fn subresources(&mut self, device: &HaDevice) -> Result<(), ProcedureError> {
        // sync
        for _ in 0..self.graphics_pipeline.frame_count() {
            let present_available = HaSemaphore::setup(device)?;
            self.present_availables.push(present_available);
        }

        Ok(())
    }

    fn commands(&mut self, kit: CommandKit) -> Result<(), ProcedureError> {

        self.command_pool = kit.pool(DeviceQueueIdentifier::Graphics)?;

        let command_buffer_count = self.graphics_pipeline.frame_count();
        let raw_commands = self.command_pool
            .allocate(CommandBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command) in raw_commands.into_iter().enumerate() {
            let mut recorder = kit.recorder(command);

            recorder.begin_record(&[CommandBufferUsageFlag::SimultaneousUseBit])?
                .begin_render_pass(&self.graphics_pipeline, frame_index)
                .bind_pipeline(&self.graphics_pipeline)
                .bind_vertex_buffers(0, &[CmdVertexBindingInfo { block: &self.vertex_buffer, sub_block_index: None }])
                .draw(self.vertex_data.len() as uint32_t, 1, 0, 0)
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
                wait_stages    : &[PipelineStageFlag::ColorAttachmentOutputBit],
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

    fn cleanup(&mut self, _: &HaDevice) {

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
