
// TODO: Remove all #[allow(dead_code)]

use ash::vk;
use gs::prelude::*;
use gsvk::prelude::common::*;
use gsvk::prelude::buffer::*;
use gsvk::prelude::descriptor::*;
use gsvk::prelude::pipeline::*;
use gsvk::command::*;
use gsvk::sync::*;

use gsma::data_size;

use super::data::{ Vertex, UboObject };
use super::data::{ VERTEX_DATA, INDEX_DATA };

use cgmath::{ Matrix4, SquareMatrix, Point3 };
use std::path::Path;

const VERTEX_SHADER_SOURCE_PATH  : &str = "src/05.cube/cube.vert";
const FRAGMENT_SHADER_SOURCE_PATH: &str = "src/05.cube/cube.frag";

pub struct CubeProcedure {

    index_data : Vec<vkuint>,
    ubo_data   : Vec<UboObject>,

    buffer_storage: GsBufferRepository<Host>,
    vertex_buffer : GsVertexBlock,
    index_buffer  : GsIndexBlock,
    ubo_buffer    : GsUniformBlock,

    pipeline: GsGraphicsPipeline,

    ubo_set     : DescriptorSet,
    #[allow(dead_code)]
    desc_storage: GsDescriptorRepository,

    command_pool   : GsCommandPool,
    command_buffers: Vec<GsCommandBuffer>,

    camera: GsStageCamera,

    present_availables: Vec<GsSemaphore>,
}

impl CubeProcedure {

    pub fn new(loader: AssetsLoader) -> Result<CubeProcedure, ProcedureError> {

        let screen_dimension = loader.screen_dimension();

        let camera = CameraConfigurator::config()
            .place_at(Point3::new(0.0, 0.0, 3.0))
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

    fn update_uniforms(&mut self) -> Result<(), ProcedureError> {

        self.ubo_data[0].model = self.camera.object_model_transformation();
        self.ubo_data[0].view  = self.camera.view_matrix();

        self.buffer_storage.data_updater()?
            .upload(&self.ubo_buffer, &self.ubo_data)?
            .finish()?;

        Ok(())
    }

    fn buffers(kit: AllocatorKit, vertex_data: &Vec<Vertex>, index_data: &Vec<vkuint>, ubo_data: &Vec<UboObject>) -> Result<(GsVertexBlock, GsIndexBlock, GsUniformBlock, GsBufferRepository<Host>), ProcedureError> {

        // vertex, index and uniform buffers.
        let mut buffer_allocator = kit.buffer(BufferStorageType::HOST);
        
        let vertex_info = VertexBlockInfo::new(data_size!(vertex_data, Vertex));
        let vertex_index = buffer_allocator.append_buffer(vertex_info)?;
        
        let index_info = IndexBlockInfo::new(data_size!(index_data, vkuint));
        let index_index = buffer_allocator.append_buffer(index_info)?;
        
        let ubo_info = UniformBlockInfo::new(0, 1, data_size!(UboObject));
        let ubo_index = buffer_allocator.append_buffer(ubo_info)?;

        let buffer_distributor = buffer_allocator.allocate()?;
        let vertex_buffer = buffer_distributor.acquire_vertex(vertex_index);
        let index_buffer = buffer_distributor.acquire_index(index_index);
        let ubo_buffer = buffer_distributor.acquire_uniform(ubo_index)?;
        
        let mut buffer_storage = buffer_distributor.into_repository();
        
        buffer_storage.data_uploader()?
            .upload(&vertex_buffer, vertex_data)?
            .upload(&index_buffer, index_data)?
            .upload(&ubo_buffer, ubo_data)?
            .finish()?;
        
        Ok((vertex_buffer, index_buffer, ubo_buffer, buffer_storage))
    }
    
    fn ubo(kit: AllocatorKit, ubo_buffer: &GsUniformBlock) -> Result<(DescriptorSet, GsDescriptorRepository), ProcedureError> {

        // descriptor
        let mut descriptor_set_config = DescriptorSetConfig::init(vk::DescriptorSetLayoutCreateFlags::empty());
        descriptor_set_config.add_buffer_binding(ubo_buffer, GsDescBindingStage::VERTEX);

        let mut descriptor_allocator = kit.descriptor(vk::DescriptorPoolCreateFlags::empty());
        let desc_index = descriptor_allocator.append_set(descriptor_set_config);

        let mut descriptor_distributor = descriptor_allocator.allocate()?;
        let ubo_set = descriptor_distributor.acquire_set(desc_index);
        
        let desc_storage = descriptor_distributor.into_repository();

        Ok((ubo_set, desc_storage))
    }

    fn pipelines(kit: PipelineKit, ubo_set: &DescriptorSet) -> Result<GsGraphicsPipeline, ProcedureError> {

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
            .add_descriptor_set(ubo_set)
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

    fn commands(kit: CommandKit, graphics_pipeline: &GsGraphicsPipeline, vertex_buffer: &GsVertexBlock, index_buffer: &GsIndexBlock, ubo_set: &DescriptorSet, index_count: usize) -> Result<(GsCommandPool, Vec<GsCommandBuffer>), ProcedureError> {

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
                .bind_index_buffer(index_buffer, 0)
                .bind_descriptor_sets(graphics_pipeline, 0, &[ubo_set])
                .draw_indexed(index_count as vkuint, 1, 0, 0, 0)
                .end_render_pass();

            let command_recorded = recorder.end_record()?;
            command_buffers.push(command_recorded);
        }

        Ok((command_pool, command_buffers))
    }
}

impl GraphicsRoutine for CubeProcedure {

    fn draw(&mut self, device: &GsDevice, device_available: &GsFence, image_available: &GsSemaphore, image_index: usize, _: f32) -> Result<&GsSemaphore, ProcedureError> {

        self.update_uniforms()?;

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
            .for_each(|semaphore| semaphore.destroy());
        self.present_availables.clear();
        self.command_buffers.clear();
        self.command_pool.destroy();
        self.pipeline.destroy();

        Ok(())
    }

    fn reload_res(&mut self, loader: AssetsLoader) -> Result<(), ProcedureError> {

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
