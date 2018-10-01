
#[macro_use]
extern crate hakurei_macros;
extern crate hakurei;

use hakurei::prelude::*;
use hakurei::prelude::config::*;
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
    vertex_item   : BufferSubItem,

    sampler_repository: HaDescriptorRepository,
    sampler_set       : DescriptorSetItem,
    image_repository  : HaImageRepository,

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
            vertex_item: BufferSubItem::unset(),

            sampler_repository: HaDescriptorRepository::empty(),
            sampler_set       : DescriptorSetItem::unset(),
            image_repository  : HaImageRepository::empty(),


            graphics_pipeline: HaGraphicsPipeline::uninitialize(),

            command_pool: HaCommandPool::uninitialize(),
            command_buffers: vec![],

            present_availables: vec![],
        }
    }
}

impl ProgramProc for TextureMappingProcedure {

    fn assets(&mut self, device: &HaDevice, kit: AllocatorKit) -> Result<(), ProcedureError> {

        // vertex buffer
        let mut vertex_allocator = kit.buffer(BufferStorageType::Cached);

        let mut vertex_buffer_config = CachedBufferConfig::new(CachedBufferUsage::VertexBuffer);
        vertex_buffer_config.add_item(data_size!(self.vertex_data, Vertex));

        self.vertex_item = vertex_allocator.attach_cached_buffer(vertex_buffer_config)?.pop().unwrap();
        self.vertex_storage = vertex_allocator.allocate()?;

        self.vertex_storage.data_uploader()?
            .upload(&self.vertex_item, &self.vertex_data)?
            .done()?;

        // image
        let image_desc = ImageDescInfo::init(
            ImageType::Type2d,
            ImageTiling::Optimal,
            &[
                ImageUsageFlag::TransferDstBit,
                ImageUsageFlag::SampledBit,
            ],
            ImageLayout::Undefined
        );
        let view_desc = ImageViewDescInfo::init(ImageViewType::Type2d);

        let mut image_allocator = kit.image();
        let image_view_index = image_allocator.attach_image(Path::new(TEXTURE_PATH), image_desc, view_desc)?;
        self.image_repository = image_allocator.allocate()?;
        let image_view_item = self.image_repository.view_item(image_view_index);

        // descriptor
        let sampler_info = SamplerDescInfo::init();
        let sampler = HaSampler::init(device, sampler_info).unwrap();

        let sampler_descriptor_info = DescriptorImageBindingInfo {
            binding: 0,
            type_  : ImageDescriptorType::CombinedImageSampler,
            count  : 1,
            sampler,
            layout: ImageLayout::ShaderReadOnlyOptimal,
            view_item: image_view_item,
        };

        let mut descriptor_set_config = DescriptorSetConfig::init(&[]);
        let _ = descriptor_set_config.add_image_binding(sampler_descriptor_info, &[
            ShaderStageFlag::FragmentStage,
        ]);

        let mut descriptor_allocator = kit.descriptor(&[]);
        let (set_item, mut descriptor_binding_items) = descriptor_allocator.attach_descriptor_set(descriptor_set_config);
        let sampler_item = descriptor_binding_items.pop().unwrap();

        self.sampler_repository = descriptor_allocator.allocate()?;
        self.sampler_repository.update_descriptors(&[sampler_item]);
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
        let first_subpass = render_pass_builder.new_subpass(SubpassType::Graphics);

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
        let viewport = HaViewport::setup(swapchain.extent);

        let pipeline_config = GraphicsPipelineConfig::new(shader_infos, vertex_input_desc, render_pass)
            .setup_viewport(viewport)
            .add_descriptor_set(self.sampler_repository.set_layout_at(&self.sampler_set))
            .finish_config();

        let mut pipeline_builder = kit.graphics_pipeline_builder()?;
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

        // command buffer
        let command_pool = kit.pool(DeviceQueueIdentifier::Graphics)?;

        let command_buffer_count = self.graphics_pipeline.frame_count();
        let command_buffers = command_pool
            .allocate(CommandBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command_buffer) in command_buffers.iter().enumerate() {
            let recorder = command_buffer.setup_record();

            recorder.begin_record(&[CommandBufferUsageFlag::SimultaneousUseBit])?
                .begin_render_pass(&self.graphics_pipeline, frame_index)
                .bind_pipeline(&self.graphics_pipeline)
                .bind_vertex_buffers(0, &self.vertex_storage.vertex_binding_infos(&[&self.vertex_item]))
                .bind_descriptor_sets(&self.graphics_pipeline, 0, &self.sampler_repository.descriptor_binding_infos(&[&self.sampler_set]))
                .draw(self.vertex_data.len() as uint32_t, 1, 0, 0)
                .end_render_pass()
                .end_record()?;
        }
        self.command_pool    = command_pool;
        self.command_buffers = command_buffers;

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

    fn clean_resources(&mut self) -> Result<(), ProcedureError> {

        for semaphore in self.present_availables.iter() {
            semaphore.cleanup();
        }
        self.present_availables.clear();
        self.command_buffers.clear();

        self.graphics_pipeline.cleanup();
        self.command_pool.cleanup();

        Ok(())
    }

    fn cleanup(&mut self) {

        for semaphore in self.present_availables.iter() {
            semaphore.cleanup();
        }

        self.graphics_pipeline.cleanup();
        self.command_pool.cleanup();

        self.sampler_repository.cleanup();
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
