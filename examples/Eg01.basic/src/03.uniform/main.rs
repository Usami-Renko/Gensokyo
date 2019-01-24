
// TODO: Remove all #[allow(dead_code)]

use ash::vk;
use gs::prelude::*;
use gsvk::prelude::common::*;
use gsvk::prelude::buffer::*;
use gsvk::prelude::descriptor::*;
use gsvk::prelude::pipeline::*;
use gsvk::prelude::command::*;
use gsvk::prelude::sync::*;
use gsvk::prelude::api::*;

use gsma::{ define_input, offset_of, vk_format, vertex_rate, data_size };

use nalgebra::{ Matrix4, Vector3 };

use std::path::{ Path, PathBuf };

const MANIFEST_PATH: &str = "src/03.uniform/gensokyo.toml";
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

const VERTEX_DATA: [Vertex; 3] = [
    Vertex { pos: [ 0.0, -0.5], color: [1.0, 0.0, 0.0, 1.0], },
    Vertex { pos: [ 0.5,  0.5], color: [0.0, 1.0, 0.0, 1.0], },
    Vertex { pos: [-0.5,  0.5], color: [0.0, 0.0, 1.0, 1.0], },
];

#[derive(Debug, Clone, Copy)]
struct UboObject {
    rotate: Matrix4<f32>,
}

struct UniformBufferProcedure {

    vertex_data   : Vec<Vertex>,
    #[allow(dead_code)]
    buffer_storage: GsBufferRepository<Host>,
    vertex_buffer : GsVertexBuffer,

    #[allow(dead_code)]
    desc_storage: GsDescriptorRepository,
    ubo_set: DescriptorSet,

    graphics_pipeline: GsPipeline<Graphics>,

    command_pool   : GsCommandPool,
    command_buffers: Vec<GsCommandBuffer>,

    present_availables: Vec<GsSemaphore>,
}

impl UniformBufferProcedure {

    fn new(loader: AssetsLoader) -> GsResult<UniformBufferProcedure> {

        let vertex_data = VERTEX_DATA.to_vec();
        let ubo_data = vec![
            UboObject {
                rotate: Matrix4::from_axis_angle(&Vector3::z_axis(), std::f32::consts::FRAC_PI_2) // rotate 90.0 degree.
            },
        ];

        let (vertex_buffer, ubo_buffer, buffer_storage) = loader.assets(|kit| {
            UniformBufferProcedure::buffers(kit, &vertex_data, &ubo_data)
        })?;

        let (ubo_set, desc_storage) = loader.assets(|kit| {
            UniformBufferProcedure::descriptor(kit, &ubo_buffer)
        })?;

        let graphics_pipeline = loader.pipelines(|kit| {
            UniformBufferProcedure::pipelines(kit, &ubo_set)
        })?;

        let present_availables = loader.syncs(|kit| {
            UniformBufferProcedure::sync_resources(kit, &graphics_pipeline)
        })?;

        let (command_pool, command_buffers) = loader.commands(|kit| {
            UniformBufferProcedure::commands(kit, &graphics_pipeline, &vertex_buffer, &ubo_set, &vertex_data)
        })?;

        let procedure = UniformBufferProcedure {
            vertex_data, buffer_storage, vertex_buffer,
            ubo_set, desc_storage,
            graphics_pipeline,
            command_pool, command_buffers,
            present_availables,
        };

        Ok(procedure)
    }

    fn buffers(kit: AllocatorKit, vertex_data: &Vec<Vertex>, uniform_data: &Vec<UboObject>) -> GsResult<(GsVertexBuffer, GsUniformBuffer, GsBufferRepository<Host>)> {

        // vertex and uniform buffer
        let mut buffer_allocator = kit.buffer(BufferStorageType::HOST);

        let vertex_info = GsBufVertexInfo::new(data_size!(Vertex), vertex_data.len());
        let vertex_index = buffer_allocator.assign(vertex_info)?;

        let uniform_info = GsBufUniformInfo::new(0, 1, data_size!(UboObject));
        let uniform_index = buffer_allocator.assign(uniform_info)?;

        let buffer_distributor = buffer_allocator.allocate()?;

        let vertex_buffer = buffer_distributor.acquire(vertex_index);
        let uniform_buffer = buffer_distributor.acquire(uniform_index);

        let mut buffer_storage = buffer_distributor.into_repository();

        buffer_storage.data_uploader()?
            .upload(&vertex_buffer, vertex_data)?
            .upload(&uniform_buffer, uniform_data)?
            .finish()?;

        Ok((vertex_buffer, uniform_buffer, buffer_storage))
    }

    fn descriptor(kit: AllocatorKit, ubo_buffer: &GsUniformBuffer) -> GsResult<(DescriptorSet, GsDescriptorRepository)> {

        // descriptor
        let mut descriptor_set_config = DescriptorSetConfig::init();
        descriptor_set_config.add_buffer_binding(ubo_buffer, GsPipelineStage::VERTEX);

        let mut descriptor_allocator = kit.descriptor(vk::DescriptorPoolCreateFlags::empty());
        let descriptor_index = descriptor_allocator.assign(descriptor_set_config);

        let descriptor_distributor = descriptor_allocator.allocate()?;
        let uniform_descriptor_set = descriptor_distributor.acquire(descriptor_index);
        let descriptor_repository = descriptor_distributor.into_repository();

        Ok((uniform_descriptor_set, descriptor_repository))
    }

    fn pipelines(kit: PipelineKit, descriptor_set: &DescriptorSet) -> GsResult<GsPipeline<Graphics>> {

        // shaders
        let vertex_shader = GsShaderInfo::from_source(
            GsPipelineStage::VERTEX,
            Path::new(VERTEX_SHADER_SOURCE_PATH),
            None,
            "[Vertex Shader]");
        let fragment_shader = GsShaderInfo::from_source(
            GsPipelineStage::FRAGMENT,
            Path::new(FRAGMENT_SHADER_SOURCE_PATH),
            None,
            "[Fragment Shader]");
        let shader_infos = vec![vertex_shader, fragment_shader];
        let vertex_input_desc = Vertex::desc();

        // pipeline
        let mut render_pass_builder = kit.pass_builder();
        let first_subpass = render_pass_builder.new_subpass();

        let color_attachment = kit.present_attachment();
        let _attachment_index = render_pass_builder.add_attachment(color_attachment, first_subpass);

        let dependency0 = kit.subpass_dependency(SubpassStage::BeginExternal, SubpassStage::AtIndex(first_subpass))
            .stage(vk::PipelineStageFlags::BOTTOM_OF_PIPE, vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .access(vk::AccessFlags::MEMORY_READ, vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
            .with_flags(vk::DependencyFlags::BY_REGION);
        render_pass_builder.add_dependency(dependency0);

        let dependency1 = kit.subpass_dependency(SubpassStage::AtIndex(first_subpass), SubpassStage::EndExternal)
            .stage(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT, vk::PipelineStageFlags::BOTTOM_OF_PIPE)
            .access(vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE, vk::AccessFlags::MEMORY_READ)
            .with_flags(vk::DependencyFlags::BY_REGION);
        render_pass_builder.add_dependency(dependency1);

        let render_pass = render_pass_builder.build()?;

        let pipeline_config = kit.pipeline_config(shader_infos, vertex_input_desc, render_pass)
            .add_descriptor_sets(&[descriptor_set])
            .finish();

        let mut pipeline_builder = kit.graphics_pipeline_builder()?;
        pipeline_builder.add_config(&pipeline_config)?;

        let mut pipelines = pipeline_builder.build(PipelineDeriveState::Independence)?;
        let graphics_pipeline = pipelines.pop().unwrap();

        Ok(graphics_pipeline)
    }

    fn sync_resources(kit: SyncKit, graphics_pipeline: &GsPipeline<Graphics>) -> GsResult<Vec<GsSemaphore>> {

        // sync
        let mut present_availables = vec![];
        for _ in 0..graphics_pipeline.frame_count() {
            let present_available = kit.semaphore()?;
            present_availables.push(present_available);
        }

        Ok(present_availables)
    }

    fn commands(kit: CommandKit, graphics_pipeline: &GsPipeline<Graphics>, vertex_buffer: &GsVertexBuffer, desc_set: &DescriptorSet, vertex_data: &Vec<Vertex>) -> GsResult<(GsCommandPool, Vec<GsCommandBuffer>)> {

        let command_pool = kit.pool(DeviceQueueIdentifier::Graphics)?;
        let mut command_buffers = vec![];

        let command_buffer_count = graphics_pipeline.frame_count();
        let raw_commands = command_pool.allocate(CmdBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command) in raw_commands.into_iter().enumerate() {
            let mut recorder = kit.pipeline_recorder(graphics_pipeline, command);

            recorder.begin_record(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE)?
                .begin_render_pass(graphics_pipeline.render_pass_ref(), frame_index)
                .bind_pipeline()
                .bind_vertex_buffers(0, &[vertex_buffer])
                .bind_descriptor_sets(0, &[desc_set])
                .draw(vertex_data.len() as vkuint, 1, 0, 0)
                .end_render_pass();

            let command_recorded = recorder.end_record()?;
            command_buffers.push(command_recorded);
        }

        Ok((command_pool, command_buffers))
    }
}


impl GraphicsRoutine for UniformBufferProcedure {

    fn draw(&mut self, device: &GsDevice, device_available: &GsFence, image_available: &GsSemaphore, image_index: usize, _: f32) -> GsResult<&GsSemaphore> {

        let submit_info = QueueSubmitBundle {
            wait_semaphores: &[image_available],
            sign_semaphores: &[&self.present_availables[image_index]],
            wait_stages    : &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT],
            commands       : &[&self.command_buffers[image_index]],
        };

        device.submit_single(&submit_info, Some(device_available), DeviceQueueIdentifier::Graphics)?;

        return Ok(&self.present_availables[image_index])
    }

    fn clean_resources(&mut self, _: &GsDevice) -> GsResult<()> {

        self.present_availables.iter()
            .for_each(|semaphore| semaphore.destroy());
        self.present_availables.clear();
        self.command_buffers.clear();
        self.command_pool.destroy();
        self.graphics_pipeline.destroy();

        Ok(())
    }

    fn reload_res(&mut self, loader: AssetsLoader) -> GsResult<()> {

        self.graphics_pipeline = loader.pipelines(|kit| {
            UniformBufferProcedure::pipelines(kit, &self.ubo_set)
        })?;

        self.present_availables = loader.syncs(|kit| {
            UniformBufferProcedure::sync_resources(kit, &self.graphics_pipeline)
        })?;

        let (command_pool, command_buffers) = loader.commands(|kit| {
            UniformBufferProcedure::commands(kit, &self.graphics_pipeline, &self.vertex_buffer, &self.ubo_set, &self.vertex_data)
        })?;
        self.command_pool = command_pool;
        self.command_buffers = command_buffers;

        Ok(())
    }

    fn clean_routine(&mut self, _device: &GsDevice) {

        self.present_availables.iter()
            .for_each(|semaphore| semaphore.destroy());
        self.graphics_pipeline.destroy();
        self.command_pool.destroy();
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

    let builder = program_env.routine().unwrap();

    let asset_loader = builder.assets_loader();
    let routine = UniformBufferProcedure::new(asset_loader).unwrap();
    let routine_flow = builder.build(routine);

    match routine_flow.launch(program_env) {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        },
    }
}
