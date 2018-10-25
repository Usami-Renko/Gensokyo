
#[macro_use]
extern crate hakurei_macros;
extern crate hakurei;

use hakurei::prelude::*;
use hakurei::prelude::queue::*;
use hakurei::prelude::pipeline::*;
use hakurei::prelude::resources::*;
use hakurei::prelude::sync::*;
use hakurei::prelude::input::*;

use std::path::Path;

const WINDOW_TITLE: &'static str = "04.Texture";
const WINDOW_WIDTH:  u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const VERTEX_SHADER_SOURCE_PATH  : &'static str = "src/04.texture/texture.vert";
const FRAGMENT_SHADER_SOURCE_PATH: &'static str = "src/04.texture/texture.frag";
const TEXTURE_PATH: &'static str = "textures/texture.jpg";

define_input! {
    #[binding = 0, rate = vertex]
    struct Vertex {
        #[location = 0, format = vec2]
        pos: [f32; 2],
        #[location = 1, format = vec2]
        tex_coord: [f32; 2],
    }
}

struct TextureMappingProcedure {

    vertex_data   : Vec<Vertex>,
    vertex_storage: HaBufferRepository,
    vertex_buffer : HaVertexBlock,

    descriptor_repository: HaDescriptorRepository,
    sampler_set          : DescriptorSetItem,
    image_repository     : HaImageRepository,
    sample_image         : HaSampleImage,

    graphics_pipeline: HaGraphicsPipeline,

    command_pool   : HaCommandPool,
    command_buffers: Vec<HaCommandBuffer>,

    present_availables: Vec<HaSemaphore>,
}

impl TextureMappingProcedure {

    fn new() -> TextureMappingProcedure {
        TextureMappingProcedure {
            vertex_data: vec![
                Vertex { pos: [-0.75, -0.75], tex_coord: [1.0, 0.0], },
                Vertex { pos: [ 0.75, -0.75], tex_coord: [0.0, 0.0], },
                Vertex { pos: [ 0.75,  0.75], tex_coord: [0.0, 1.0], },
                Vertex { pos: [ 0.75,  0.75], tex_coord: [0.0, 1.0], },
                Vertex { pos: [-0.75,  0.75], tex_coord: [1.0, 1.0], },
                Vertex { pos: [-0.75, -0.75], tex_coord: [1.0, 0.0], },
            ],
            vertex_storage: HaBufferRepository::empty(),
            vertex_buffer : HaVertexBlock::uninitialize(),

            descriptor_repository: HaDescriptorRepository::empty(),
            sampler_set       : DescriptorSetItem::unset(),
            image_repository  : HaImageRepository::empty(),
            sample_image      : HaSampleImage::uninitialize(),

            graphics_pipeline: HaGraphicsPipeline::uninitialize(),

            command_pool: HaCommandPool::uninitialize(),
            command_buffers: vec![],

            present_availables: vec![],
        }
    }
}

impl ProgramProc for TextureMappingProcedure {

    fn assets(&mut self, kit: AllocatorKit) -> Result<(), ProcedureError> {

        // vertex buffer
        let mut buffer_allocator = kit.buffer(BufferStorageType::Cached);

        let vertex_info = VertexBlockInfo::new(data_size!(self.vertex_data, Vertex));

        self.vertex_buffer = buffer_allocator.append_vertex(vertex_info)?;
        self.vertex_storage = buffer_allocator.allocate()?;

        self.vertex_storage.data_uploader()?
            .upload(&self.vertex_buffer, &self.vertex_data)?
            .done()?;

        // image
        let image_info = SampleImageInfo::new(0, 1, ImagePipelineStage::FragmentStage);

        let mut image_allocator = kit.image(ImageStorageType::Device);
        self.sample_image = image_allocator.attach_sample_image(Path::new(TEXTURE_PATH), image_info)?;
        self.image_repository = image_allocator.allocate()?;
        self.image_repository.get_allocated_infos(&mut self.sample_image);

        // descriptor
        let mut descriptor_set_config = DescriptorSetConfig::init(&[]);
        let _ = descriptor_set_config.add_image_binding(&self.sample_image, &[
            ShaderStageFlag::FragmentStage,
        ]);

        let mut descriptor_allocator = kit.descriptor(&[]);
        let (set_item, mut descriptor_binding_items) = descriptor_allocator.attach_descriptor_set(descriptor_set_config);
        let sampler_item = descriptor_binding_items.pop().unwrap();

        self.descriptor_repository = descriptor_allocator.allocate()?;
        self.descriptor_repository.update_descriptors(&[sampler_item])?;
        self.sampler_set = set_item;

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
            .add_descriptor_set(self.descriptor_repository.set_layout_at(&self.sampler_set))
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
        self.command_buffers = self.command_pool
            .allocate(CommandBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command_buffer) in self.command_buffers.iter().enumerate() {
            let recorder = command_buffer.setup_record();

            recorder.begin_record(&[CommandBufferUsageFlag::SimultaneousUseBit])?
                .begin_render_pass(&self.graphics_pipeline, frame_index)
                .bind_pipeline(&self.graphics_pipeline)
                .bind_vertex_buffers(0, &[CmdVertexBindingInfo { block: &self.vertex_buffer, sub_block_index: None }])
                .bind_descriptor_sets(&self.graphics_pipeline, 0, self.descriptor_repository.descriptor_binding_infos(&[&self.sampler_set]))
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

    fn cleanup(&mut self, device: &HaDevice) {

        self.present_availables.iter()
            .for_each(|semaphore| semaphore.cleanup());

        self.graphics_pipeline.cleanup();
        self.command_pool.cleanup();

        self.sample_image.cleanup(device);
        self.descriptor_repository.cleanup();
        self.image_repository.cleanup();
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

    let procecure = TextureMappingProcedure::new();

    let mut program = ProgramEnv::new(procecure).unwrap();

    match program.launch() {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        }
    }
}
