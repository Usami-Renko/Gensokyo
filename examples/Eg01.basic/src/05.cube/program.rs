
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

use gsma::data_size;

use super::data::{ Vertex, UboObject };
use super::data::{ VERTEX_DATA, INDEX_DATA };

use nalgebra::{ Matrix4, Point3 };
use std::path::Path;

const VERTEX_SHADER_SOURCE_PATH  : &str = "src/05.cube/cube.vert";
const FRAGMENT_SHADER_SOURCE_PATH: &str = "src/05.cube/cube.frag";

pub struct CubeProcedure {

    index_data : Vec<vkuint>,
    ubo_data   : Vec<UboObject>,

    buffer_storage: GsBufferRepository<Host>,
    vertex_buffer : GsVertexBuffer,
    index_buffer  : GsIndexBuffer,
    ubo_buffer    : GsUniformBuffer,

    pipeline: GsPipeline<Graphics>,

    ubo_set     : DescriptorSet,
    #[allow(dead_code)]
    desc_storage: GsDescriptorRepository,

    command_pool   : GsCommandPool,
    command_buffers: Vec<GsCommandBuffer>,

    camera: GsStageCamera,

    present_availables: Vec<GsSemaphore>,
}

impl CubeProcedure {

    pub fn new(loader: AssetsLoader) -> GsResult<CubeProcedure> {

        let screen_dimension = loader.screen_dimension();

        let camera = GsCameraFactory::config()
            .place_at(Point3::new(0.0, 0.0, 2.5))
            .screen_aspect_ratio(screen_dimension.width as f32 / screen_dimension.height as f32)
            .into_stage_camera();

        let vertex_data = VERTEX_DATA.to_vec();
        let index_data = INDEX_DATA.to_vec();
        let ubo_data = vec![
            UboObject {
                projection: camera.proj_matrix(),
                view      : camera.view_matrix(),
                model     : Matrix4::identity(),
            },
        ];

        let (vertex_buffer, index_buffer, ubo_buffer, buffer_storage) = loader.assets(|kit| {
            CubeProcedure::buffers(kit, &vertex_data, &index_data, &ubo_data)
        })?;

        let (ubo_set, desc_storage) = loader.assets(|kit| {
            CubeProcedure::ubo(kit, &ubo_buffer)
        })?;

        let pipeline = loader.pipelines(|kit| {
            CubeProcedure::pipelines(kit, &ubo_set)
        })?;

        let present_availables = loader.syncs(|kit| {
            CubeProcedure::sync_resources(kit, &pipeline)
        })?;

        let (command_pool, command_buffers) = loader.commands(|kit| {
            CubeProcedure::commands(kit, &pipeline, &vertex_buffer, &index_buffer, &ubo_set, index_data.len())
        })?;

        let procecure = CubeProcedure {
            index_data, ubo_data,
            buffer_storage, vertex_buffer, index_buffer, ubo_buffer,
            desc_storage, ubo_set,
            pipeline,
            command_pool, command_buffers,
            camera,
            present_availables,
        };

        Ok(procecure)
    }

    fn update_uniforms(&mut self) -> GsResult<()> {

        self.ubo_data[0].model = self.camera.object_model_transformation();
        self.ubo_data[0].view  = self.camera.view_matrix();

        self.buffer_storage.data_updater()?
            .update(&self.ubo_buffer, &self.ubo_data)?
            .finish()?;

        Ok(())
    }

    fn buffers(kit: AllocatorKit, vertex_data: &Vec<Vertex>, index_data: &Vec<vkuint>, ubo_data: &Vec<UboObject>) -> GsResult<(GsVertexBuffer, GsIndexBuffer, GsUniformBuffer, GsBufferRepository<Host>)> {

        // vertex, index and uniform buffers.
        let mut buffer_allocator = kit.buffer(BufferStorageType::HOST);
        
        let vertex_info = GsBufVertexInfo::new(data_size!(Vertex), vertex_data.len());
        let vertex_index = buffer_allocator.assign(vertex_info)?;
        
        let index_info = GsBufIndicesInfo::new(index_data.len());
        let index_index = buffer_allocator.assign(index_info)?;
        
        let ubo_info = GsBufUniformInfo::new(0, 1, data_size!(UboObject));
        let ubo_index = buffer_allocator.assign(ubo_info)?;

        let buffer_distributor = buffer_allocator.allocate()?;

        let vertex_buffer = buffer_distributor.acquire(vertex_index);
        let index_buffer = buffer_distributor.acquire(index_index);
        let ubo_buffer = buffer_distributor.acquire(ubo_index);
        
        let mut buffer_storage = buffer_distributor.into_repository();
        
        buffer_storage.data_uploader()?
            .upload(&vertex_buffer, vertex_data)?
            .upload(&index_buffer, index_data)?
            .upload(&ubo_buffer, ubo_data)?
            .finish()?;
        
        Ok((vertex_buffer, index_buffer, ubo_buffer, buffer_storage))
    }
    
    fn ubo(kit: AllocatorKit, ubo_buffer: &GsUniformBuffer) -> GsResult<(DescriptorSet, GsDescriptorRepository)> {

        // descriptor
        let mut descriptor_set_config = DescriptorSetConfig::init();
        descriptor_set_config.add_buffer_binding(ubo_buffer, GsPipelineStage::VERTEX);

        let mut descriptor_allocator = kit.descriptor(vk::DescriptorPoolCreateFlags::empty());
        let desc_index = descriptor_allocator.assign(descriptor_set_config);

        let descriptor_distributor = descriptor_allocator.allocate()?;
        let ubo_set = descriptor_distributor.acquire(desc_index);
        
        let desc_storage = descriptor_distributor.into_repository();

        Ok((ubo_set, desc_storage))
    }

    fn pipelines(kit: PipelineKit, ubo_set: &DescriptorSet) -> GsResult<GsPipeline<Graphics>> {

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
        let shader_infos = vec![
            vertex_shader,
            fragment_shader,
        ];
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
            .add_descriptor_sets(&[ubo_set])
            .finish();

        let mut pipeline_builder = kit.graphics_pipeline_builder()?;
        pipeline_builder.add_config(pipeline_config);

        let mut pipelines = pipeline_builder.build()?;
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

    fn commands(kit: CommandKit, graphics_pipeline: &GsPipeline<Graphics>, vertex_buffer: &GsVertexBuffer, index_buffer: &GsIndexBuffer, ubo_set: &DescriptorSet, index_count: usize) -> GsResult<(GsCommandPool, Vec<GsCommandBuffer>)> {

        let command_pool = kit.pool(DeviceQueueIdentifier::Graphics)?;
        let mut command_buffers = vec![];

        let command_buffer_count = graphics_pipeline.frame_count();
        let raw_commands = command_pool.allocate(CmdBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command) in raw_commands.into_iter().enumerate() {
            let mut recorder = kit.pipeline_recorder(graphics_pipeline, command);

            recorder.begin_record(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE)?
                .begin_render_pass(graphics_pipeline, frame_index)
                .bind_pipeline()
                .bind_vertex_buffers(0, &[vertex_buffer])
                .bind_index_buffer(index_buffer, 0)
                .bind_descriptor_sets(0, &[ubo_set])
                .draw_indexed(index_count as vkuint, 1, 0, 0, 0)
                .end_render_pass();

            let command_recorded = recorder.end_record()?;
            command_buffers.push(command_recorded);
        }

        Ok((command_pool, command_buffers))
    }
}

impl GraphicsRoutine for CubeProcedure {

    fn draw(&mut self, device: &GsDevice, device_available: &GsFence, image_available: &GsSemaphore, image_index: usize, _: f32) -> GsResult<&GsSemaphore> {

        self.update_uniforms()?;

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
        self.pipeline.destroy();

        Ok(())
    }

    fn reload_res(&mut self, loader: AssetsLoader) -> GsResult<()> {

        self.pipeline = loader.pipelines(|kit| {
            CubeProcedure::pipelines(kit, &self.ubo_set)
        })?;

        self.present_availables = loader.syncs(|kit| {
            CubeProcedure::sync_resources(kit, &self.pipeline)
        })?;

        let (command_pool, command_buffers) = loader.commands(|kit| {
            CubeProcedure::commands(kit, &self.pipeline, &self.vertex_buffer, &self.index_buffer, &self.ubo_set, self.index_data.len())
        })?;
        self.command_pool = command_pool;
        self.command_buffers = command_buffers;

        Ok(())
    }

    fn clean_routine(&mut self, _: &GsDevice) {

        self.present_availables.iter()
            .for_each(|semaphore| semaphore.destroy());
        self.pipeline.destroy();
        self.command_pool.destroy();
    }

    fn react_input(&mut self, inputer: &ActionNerve, delta_time: f32) -> SceneAction {

        if inputer.is_key_pressed(GsKeycode::ESCAPE) {
            return SceneAction::Terminal
        }

        self.camera.react_input(inputer, delta_time);

        SceneAction::Rendering
    }
}
