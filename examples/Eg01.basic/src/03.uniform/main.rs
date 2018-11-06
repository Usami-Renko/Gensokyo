
#[macro_use]
extern crate hakurei_macros;
extern crate hakurei;

extern crate cgmath;
use cgmath::{ Matrix4, Vector3, Deg };

use hakurei::prelude::*;
use hakurei::prelude::queue::*;
use hakurei::prelude::pipeline::*;
use hakurei::prelude::resources::*;
use hakurei::prelude::sync::*;
use hakurei::prelude::input::*;

use std::path::{ Path, PathBuf };

const MANIFEST_PATH: &str = "src/03.uniform/hakurei.toml";
const VERTEX_SHADER_SOURCE_PATH  : &str = "src/03.uniform/uniform.vert";
const FRAGMENT_SHADER_SOURCE_PATH: &str = "src/03.uniform/uniform.frag";

define_input! {
    #[binding = 0, rate = vertex]
    struct Vertex {
        #[location = 0, format = vec2]
        pos:   [f32; 2],
        #[location = 1, format = vec4]
        color: [f32; 4],
    }
}

#[derive(Debug, Clone, Copy)]
struct UboObject {
    rotate: Matrix4<f32>,
}

struct UniformBufferProcedure {

    vertex_data   : Vec<Vertex>,
    buffer_storage: HaBufferRepository,
    vertex_buffer : HaVertexBlock,

    ubo_data  : Vec<UboObject>,
    ubo_buffer: HaUniformBlock,

    desc_storage: HaDescriptorRepository,
    ubo_set: DescriptorSet,

    graphics_pipeline: HaGraphicsPipeline,

    command_pool   : HaCommandPool,
    command_buffers: Vec<HaCommandBuffer>,

    present_availables: Vec<HaSemaphore>,
}

impl UniformBufferProcedure {

    fn new() -> UniformBufferProcedure {
        UniformBufferProcedure {
            vertex_data: vec![
                Vertex { pos: [ 0.0, -0.5], color: [1.0, 0.0, 0.0, 1.0], },
                Vertex { pos: [ 0.5,  0.5], color: [0.0, 1.0, 0.0, 1.0], },
                Vertex { pos: [-0.5,  0.5], color: [0.0, 0.0, 1.0, 1.0], },
            ],
            buffer_storage: HaBufferRepository::empty(),
            vertex_buffer : HaVertexBlock::uninitialize(),

            ubo_data: vec![
                UboObject {
                    rotate: Matrix4::from_axis_angle(Vector3::new(0.0, 0.0, 1.0), Deg(90.0))
                },
            ],
            ubo_buffer: HaUniformBlock::uninitialize(),

            desc_storage: HaDescriptorRepository::empty(),
            ubo_set: DescriptorSet::unset(),

            graphics_pipeline: HaGraphicsPipeline::uninitialize(),

            command_pool: HaCommandPool::uninitialize(),
            command_buffers: vec![],

            present_availables: vec![],
        }
    }
}

impl ProgramProc for UniformBufferProcedure {

    fn assets(&mut self, kit: AllocatorKit) -> Result<(), ProcedureError> {

        // vertex and uniform buffer
        let mut buffer_allocator = kit.buffer(BufferStorageType::Host);

        let vertex_info = VertexBlockInfo::new(data_size!(self.vertex_data, Vertex));
        self.vertex_buffer = buffer_allocator.append_vertex(vertex_info)?;

        let uniform_info = UniformBlockInfo::new(0, 1, data_size!(self.ubo_data, UboObject));
        self.ubo_buffer = buffer_allocator.append_uniform(uniform_info)?;

        self.buffer_storage = buffer_allocator.allocate()?;
        self.buffer_storage.data_uploader()?
            .upload(&self.vertex_buffer, &self.vertex_data)?
            .upload(&self.ubo_buffer, &self.ubo_data)?
            .done()?;

        // descriptor
        let mut descriptor_set_config = DescriptorSetConfig::init(&[]);
        descriptor_set_config.add_buffer_binding(
            &self.ubo_buffer,
            &[
            ShaderStageFlag::VertexStage,
        ]);

        let mut descriptor_allocator = kit.descriptor(&[]);
        let descriptor_index = descriptor_allocator.append_set(descriptor_set_config);

        let mut descriptor_distributor = descriptor_allocator.allocate()?;
        self.ubo_set = descriptor_distributor.acquire_set(descriptor_index);
        self.desc_storage = descriptor_distributor.into_repository();

        Ok(())
    }

    fn pipelines(&mut self, kit: PipelineKit, swapchain: &HaSwapchain) -> Result<(), ProcedureError> {
        // shaders
        let vertex_shader = HaShaderInfo::from_source(
            ShaderStageFlag::VertexStage,
            Path::new(VERTEX_SHADER_SOURCE_PATH),
            None,
            "[Vertex Shader]");
        let fragment_shader = HaShaderInfo::from_source(
            ShaderStageFlag::FragmentStage,
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
            .add_descriptor_set(&self.ubo_set)
            .finish();

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
                .bind_descriptor_sets(&self.graphics_pipeline, 0, &[&self.ubo_set])
                .draw(self.vertex_data.len() as uint32_t, 1, 0, 0)
                .end_render_pass();

            let command_recorded = recorder.end_record()?;
            self.command_buffers.push(command_recorded);
        }

        Ok(())
    }

    fn draw(&mut self, device: &HaDevice, device_available: &HaFence, image_available: &HaSemaphore, image_index: usize, _: f32) -> Result<&HaSemaphore, ProcedureError> {

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

        for semaphore in self.present_availables.iter() {
            semaphore.cleanup();
        }
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
        self.desc_storage.cleanup();
        self.buffer_storage.cleanup();
    }

    fn react_input(&mut self, inputer: &ActionNerve, _: f32) -> SceneAction {

        if inputer.is_key_pressed(HaKeycode::Escape) {
            return SceneAction::Terminal
        }

        SceneAction::Rendering
    }
}

fn main() {

    let procecure = UniformBufferProcedure::new();

    let manifest = PathBuf::from(MANIFEST_PATH);
    let mut program = ProgramEnv::new(Some(manifest), procecure).unwrap();

    match program.launch() {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        }
    }
}
