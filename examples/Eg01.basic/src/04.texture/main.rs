
extern crate ash;
#[macro_use]
extern crate gensokyo_macros;
extern crate gensokyo_vulkan as gsvk;
extern crate gensokyo as gs;
extern crate cgmath;

use ash::vk;
use gs::prelude::*;
use gsvk::prelude::common::*;
use gsvk::prelude::buffer::*;
use gsvk::prelude::image::*;
use gsvk::prelude::descriptor::*;
use gsvk::prelude::pipeline::*;
use gsvk::command::*;
use gsvk::sync::*;

use std::path::{ Path, PathBuf };

const MANIFEST_PATH: &str = "src/04.texture/gensokyo.toml";
const VERTEX_SHADER_SOURCE_PATH  : &str = "src/04.texture/texture.vert";
const FRAGMENT_SHADER_SOURCE_PATH: &str = "src/04.texture/texture.frag";
const TEXTURE_PATH: &str = "textures/texture.jpg";

define_input! {
    #[binding = 0, rate = vertex]
    struct Vertex {
        #[location = 0, format = vec2]
        pos: [f32; 2],
        #[location = 1, format = vec2]
        tex_coord: [f32; 2],
    }
}

const VERTEX_DATA: [Vertex; 6] = [
    Vertex { pos: [-0.75, -0.75], tex_coord: [1.0, 0.0], },
    Vertex { pos: [ 0.75, -0.75], tex_coord: [0.0, 0.0], },
    Vertex { pos: [ 0.75,  0.75], tex_coord: [0.0, 1.0], },
    Vertex { pos: [ 0.75,  0.75], tex_coord: [0.0, 1.0], },
    Vertex { pos: [-0.75,  0.75], tex_coord: [1.0, 1.0], },
    Vertex { pos: [-0.75, -0.75], tex_coord: [1.0, 0.0], },
];

struct TextureMappingProcedure {

    vertex_data   : Vec<Vertex>,
    buffer_storage: GsBufferRepository<Cached>,
    vertex_buffer : GsVertexBlock,

    desc_storage  : GsDescriptorRepository,
    sampler_set   : DescriptorSet,
    image_storage : GsImageRepository<Device>,
    sample_image  : GsSampleImage,

    graphics_pipeline: GsGraphicsPipeline,

    command_pool   : GsCommandPool,
    command_buffers: Vec<GsCommandBuffer>,

    present_availables: Vec<GsSemaphore>,
}

impl TextureMappingProcedure {

    fn new(loader: AssetsLoader) -> Result<TextureMappingProcedure, ProcedureError> {

        let vertex_data = VERTEX_DATA.to_vec();

        let (vertex_buffer, buffer_storage) = loader.assets(|kit| {
            TextureMappingProcedure::vertex_buffer(kit, &vertex_data)
        })?;

        let (sample_image, image_storage) = loader.assets(|kit| {
            TextureMappingProcedure::image_sampler(kit)
        })?;

        let (sampler_set, desc_storage) = loader.assets(|kit| {
            TextureMappingProcedure::descriptor(kit, &sample_image)
        })?;

        let graphics_pipeline = loader.pipelines(|kit| {
            TextureMappingProcedure::pipelines(kit, &sampler_set)
        })?;

        let present_availables = loader.syncs(|kit| {
            TextureMappingProcedure::sync_resources(kit, &graphics_pipeline)
        })?;

        let (command_pool, command_buffers) = loader.commands(|kit| {
            TextureMappingProcedure::commands(kit, &graphics_pipeline, &vertex_buffer, &sampler_set, &vertex_data)
        })?;

        let procecure = TextureMappingProcedure {
            vertex_data, buffer_storage, vertex_buffer,
            desc_storage, sampler_set, image_storage, sample_image,
            graphics_pipeline,
            command_pool, command_buffers,
            present_availables,
        };

        Ok(procecure)
    }

    fn vertex_buffer(kit: AllocatorKit, vertex_data: &Vec<Vertex>) -> Result<(GsVertexBlock, GsBufferRepository<Cached>), ProcedureError> {

        let mut buffer_allocator = kit.buffer(BufferStorageType::CACHED);

        let vertex_info = VertexBlockInfo::new(data_size!(vertex_data, Vertex));
        let vertex_index = buffer_allocator.append_buffer(vertex_info)?;

        let vertex_distributor = buffer_allocator.allocate()?;

        let vertex_buffer = vertex_distributor.acquire_vertex(vertex_index);
        let mut vertex_storage = vertex_distributor.into_repository();

        vertex_storage.data_uploader()?
            .upload(&vertex_buffer, vertex_data)?
            .finish()?;

        Ok((vertex_buffer, vertex_storage))
    }

    fn image_sampler(kit: AllocatorKit) -> Result<(GsSampleImage, GsImageRepository<Device>), ProcedureError> {

        let image_loader = kit.image_loader();
        let image_storage_info = image_loader.load_2d(Path::new(TEXTURE_PATH))?;

        let mut image_info = SampleImageInfo::new(0, 1, image_storage_info, ImagePipelineStage::FragmentStage);

        let mut image_allocator = kit.image(ImageStorageType::DEVICE);
        image_allocator.append_sample_image(&mut image_info)?;

        let image_distributor = image_allocator.allocate()?;

        let sample_image = image_distributor.acquire_sample_image(image_info)?;
        let image_storage = image_distributor.into_repository();

        Ok((sample_image, image_storage))
    }

    fn descriptor(kit: AllocatorKit, sample_image: &GsSampleImage) -> Result<(DescriptorSet, GsDescriptorRepository), ProcedureError> {

        let mut descriptor_set_config = DescriptorSetConfig::init(vk::DescriptorSetLayoutCreateFlags::empty());
        descriptor_set_config.add_image_binding(sample_image, GsDescBindingStage::FRAGMENT);

        let mut descriptor_allocator = kit.descriptor(vk::DescriptorPoolCreateFlags::empty());
        let descriptor_index = descriptor_allocator.append_set(descriptor_set_config);

        let mut descriptor_distributor = descriptor_allocator.allocate()?;
        let sampler_set = descriptor_distributor.acquire_set(descriptor_index);

        let descriptor_storage = descriptor_distributor.into_repository();

        Ok((sampler_set, descriptor_storage))
    }

    fn pipelines(kit: PipelineKit, sampler_set: &DescriptorSet) -> Result<GsGraphicsPipeline, ProcedureError> {

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

        let color_attachment = kit.present_attachment();
        let _attachment_index = render_pass_builder.add_attachemnt(color_attachment, first_subpass);

        let dependency = kit.subpass_dependency(SubpassStage::External, SubpassStage::AtIndex(first_subpass))
            .stage(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT, vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .access(vk::AccessFlags::empty(), vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE);
        render_pass_builder.add_dependenty(dependency);

        let render_pass = render_pass_builder.build()?;

        let pipeline_config = kit.pipeline_config(shader_infos, vertex_input_desc, render_pass)
            .add_descriptor_set(sampler_set)
            .finish();

        let mut pipeline_builder = kit.graphics_pipeline_builder()?;
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

    fn commands(kit: CommandKit, graphics_pipeline: &GsGraphicsPipeline, vertex_buffer: &GsVertexBlock, sampler_set: &DescriptorSet, vertex_data: &Vec<Vertex>) -> Result<(GsCommandPool, Vec<GsCommandBuffer>), ProcedureError> {

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
                .bind_descriptor_sets(graphics_pipeline, 0, &[sampler_set])
                .draw(vertex_data.len() as vkuint, 1, 0, 0)
                .end_render_pass();

            let command_recorded = recorder.end_record()?;
            command_buffers.push(command_recorded);
        }

        Ok((command_pool, command_buffers))
    }
}

impl GraphicsRoutine for TextureMappingProcedure {

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
            TextureMappingProcedure::pipelines(kit, &self.sampler_set)
        })?;

        self.present_availables = loader.syncs(|kit| {
            TextureMappingProcedure::sync_resources(kit, &self.graphics_pipeline)
        })?;

        let (command_pool, command_buffers) = loader.commands(|kit| {
            TextureMappingProcedure::commands(kit, &self.graphics_pipeline, &self.vertex_buffer, &self.sampler_set, &self.vertex_data)
        })?;
        self.command_pool = command_pool;
        self.command_buffers = command_buffers;

        Ok(())
    }

    fn clean_routine(&mut self, device: &GsDevice) {

        self.present_availables.iter()
            .for_each(|semaphore| semaphore.cleanup());
        self.graphics_pipeline.cleanup();
        self.command_pool.cleanup();
        
        self.desc_storage.cleanup();
        self.sample_image.cleanup(device);
        self.image_storage.cleanup();
        self.buffer_storage.cleanup();
    }

    fn react_input(&mut self, inputer: &ActionNerve, _: f32) -> SceneAction {

        if inputer.is_key_pressed(GsKeycode::ESCAPE) {
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
        let routine = TextureMappingProcedure::new(asset_loader).unwrap();
        builder.build(routine)
    };

    match routine_flow.launch(program_env) {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        },
    }
}
