
#[macro_use]
extern crate hakurei_macros;
extern crate hakurei;

extern crate cgmath;
use cgmath::{ Matrix4, Vector3, Deg };

use hakurei::prelude::*;
use hakurei::prelude::config::*;
use hakurei::prelude::queue::*;
use hakurei::prelude::pipeline::*;
use hakurei::prelude::resources::*;
use hakurei::prelude::sync::*;
use hakurei::prelude::input::*;

use std::path::Path;

const WINDOW_TITLE: &'static str = "03.Unifrom";
const WINDOW_WIDTH:  u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const VERTEX_SHADER_SOURCE_PATH  : &'static str = "src/03.uniform/uniform.vert";
const FRAGMENT_SHADER_SOURCE_PATH: &'static str = "src/03.uniform/uniform.frag";

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
    ubo_set: DescriptorSetItem,

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
            ubo_set: DescriptorSetItem::unset(),

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
        let ubo_binding_index = descriptor_set_config.add_buffer_binding(
            &self.ubo_buffer,
            &[
            ShaderStageFlag::VertexStage,
        ])?;

        let mut descriptor_allocator = kit.descriptor(&[]);
        let (descriptor_set_item, descriptor_binding_items) = descriptor_allocator.attach_descriptor_set(descriptor_set_config);
        let ubo_descriptor_item = descriptor_binding_items[ubo_binding_index].clone();

        self.desc_storage = descriptor_allocator.allocate()?;
        self.desc_storage.update_descriptors(&[ubo_descriptor_item])?;
        self.ubo_set = descriptor_set_item;

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
            .add_descriptor_set(self.desc_storage.set_layout_at(&self.ubo_set))
            .finish_config();

        let mut pipeline_builder = kit.pipeline_builder(PipelineType::Graphics)?;
        pipeline_builder.add_config(pipeline_config);

        let mut graphics_pipelines = pipeline_builder.build()?;
        self.graphics_pipeline = graphics_pipelines.pop().unwrap();

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
        self.command_buffers = self.command_pool
            .allocate(CommandBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command_buffer) in self.command_buffers.iter().enumerate() {
            let recorder = command_buffer.setup_record();

            recorder.begin_record(&[CommandBufferUsageFlag::SimultaneousUseBit])?
                .begin_render_pass(&self.graphics_pipeline, frame_index)
                .bind_pipeline(&self.graphics_pipeline)
                .bind_vertex_buffers(0, &[CmdVertexBindingInfo { block: &self.vertex_buffer, sub_block_index: None }])
                .bind_descriptor_sets(&self.graphics_pipeline, 0, self.desc_storage.descriptor_binding_infos(&[&self.ubo_set]))
                .draw(self.vertex_data.len() as uint32_t, 1, 0, 0)
                .end_render_pass()
                .end_record()?;
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
