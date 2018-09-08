
#[macro_use]
extern crate hakurei_macros;
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
use hakurei::resources::descriptor::*;
use hakurei::resources::image::*;
use hakurei::sync::prelude::*;

use std::path::Path;

const WINDOW_TITLE: &'static str = "Texture Mapping Example";
const WINDOW_WIDTH:  u32 = 800;
const WINDOW_HEIGHT: u32 = 800;
const VERTEX_SHADER_PATH  : &'static str = "shaders/texture.vert.spv";
const FRAGMENT_SHADER_PATH: &'static str = "shaders/texture.frag.spv";
const TEXTURE_PATH: &'static str = "texture/texture.jpg";

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

    fn assets(&mut self, device: &HaLogicalDevice, generator: &ResourceGenerator) -> Result<(), ProcedureError> {

        // vertex and uniform buffer
        let mut vertex_buffer_config = BufferConfig::init(
            &[BufferUsageFlag::VertexBufferBit],
            &[
                MemoryPropertyFlag::HostVisibleBit,
                MemoryPropertyFlag::HostCoherentBit,
            ],
            &[]
        );
        let _ = vertex_buffer_config.add_item(data_size!(self.vertex_data, Vertex));

        let mut vertex_allocator = generator.buffer();
        self.vertex_item = vertex_allocator.attach_buffer(vertex_buffer_config)?.pop().unwrap();

        self.vertex_storage = vertex_allocator.allocate()?;
        self.vertex_storage.tranfer_data(device, &self.vertex_data, &self.vertex_item)?;

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

        let mut image_allocator = generator.image();
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

        let mut descriptor_allocator = generator.descriptor(&[]);
        let (set_item, mut descriptor_binding_items) = descriptor_allocator.attach_descriptor_set(descriptor_set_config);
        let sampler_item = descriptor_binding_items.pop().unwrap();

        self.sampler_repository = descriptor_allocator.allocate()?;
        self.sampler_repository.update_descriptors(device, &[sampler_item]);
        self.sampler_set = set_item;

        Ok(())
    }

    fn pipelines(&mut self, device: &HaLogicalDevice, swapchain: &HaSwapchain) -> Result<(), ProcedureError> {
        // shaders
        let vertex_shader = HaShaderInfo::setup(
            ShaderStageFlag::VertexStage,
            Path::new(VERTEX_SHADER_PATH),
            None);
        let fragment_shader = HaShaderInfo::setup(
            ShaderStageFlag::FragmentStage,
            Path::new(FRAGMENT_SHADER_PATH),
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
            .add_descriptor_set(self.sampler_repository.set_layout_at(&self.sampler_set))
            .finish_config();

        let mut pipeline_builder = GraphicsPipelineBuilder::init();
        pipeline_builder.add_config(pipeline_config);

        let mut graphics_pipelines = pipeline_builder.build(device)?;
        self.graphics_pipeline = graphics_pipelines.pop().unwrap();

        Ok(())
    }

    fn subresources(&mut self, device: &HaLogicalDevice) -> Result<(), ProcedureError> {
        // sync
        for _ in 0..self.graphics_pipeline.frame_count() {
            let present_available = HaSemaphore::setup(device)?;
            self.present_availables.push(present_available);
        }
        Ok(())
    }

    fn commands(&mut self, device: &HaLogicalDevice) -> Result<(), ProcedureError> {

        // command buffer
        let command_pool = HaCommandPool::setup(&device, DeviceQueueIdentifier::Graphics, &[])?;

        let command_buffer_count = self.graphics_pipeline.frame_count();
        let mut command_buffers = command_pool
            .allocate(device, CommandBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command_buffer) in command_buffers.iter_mut().enumerate() {
            let recorder = command_buffer.setup_record(device);

            recorder.begin_record(&[CommandBufferUsageFlag::SimultaneousUseBit])?
                .begin_render_pass(&self.graphics_pipeline, frame_index)
                .bind_pipeline(&self.graphics_pipeline)
                .bind_vertex_buffers(0, &self.vertex_storage.vertex_binding_infos(&[&self.vertex_item]))
                .bind_descriptor_sets(&self.graphics_pipeline, 0, &self.sampler_repository.descriptor_binding_infos(&[&self.sampler_set]))
                .draw(self.vertex_data.len() as uint32_t, 1, 0, 0)
                .end_render_pass()
                .finish()?;
        }
        self.command_pool    = command_pool;
        self.command_buffers = command_buffers;

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

    fn clean_resources(&mut self, device: &HaLogicalDevice) -> Result<(), ProcedureError> {

        for semaphore in self.present_availables.iter() {
            semaphore.cleanup(device);
        }
        self.present_availables.clear();
        self.command_buffers.clear();

        self.graphics_pipeline.cleanup(device);
        self.command_pool.cleanup(device);

        Ok(())
    }

    fn cleanup(&mut self, device: &HaLogicalDevice) {

        for semaphore in self.present_availables.iter() {
            semaphore.cleanup(device);
        }

        self.graphics_pipeline.cleanup(device);
        self.command_pool.cleanup(device);

        self.sampler_repository.cleanup(device);
        self.image_repository.cleanup(device);
        self.vertex_storage.cleanup(device);
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
