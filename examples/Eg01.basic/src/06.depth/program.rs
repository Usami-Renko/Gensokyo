
// TODO: Remove all #[allow(dead_code)]

use ash::vk;
use gs::prelude::*;
use gsvk::prelude::common::*;
use gsvk::prelude::buffer::*;
use gsvk::prelude::image::*;
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

const VERTEX_SHADER_SOURCE_PATH  : &str = "src/06.depth/depth.vert.glsl";
const FRAGMENT_SHADER_SOURCE_PATH: &str = "src/06.depth/depth.frag.glsl";

pub struct DepthProcedure {

    index_data : Vec<vkuint>,
    ubo_data   : Vec<UboObject>,

    buffer_storage: GsBufferRepository<Host>,
    vertex_buffer : GsVertexBuffer,
    index_buffer  : GsIndexBuffer,
    ubo_buffer    : GsUniformBuffer,

    pipeline: GsPipeline<Graphics>,

    #[allow(dead_code)]
    desc_storage: GsDescriptorRepository,
    ubo_set     : DescriptorSet,

    depth_attachment: GsDSAttachment,
    #[allow(dead_code)]
    image_storage   : GsImageRepository<Device>,

    command_pool   : GsCommandPool,
    command_buffers: Vec<GsCommandBuffer>,

    camera: GsFlightCamera,

    present_availables: Vec<GsSemaphore>,
}

impl DepthProcedure {

    pub fn new(initializer: AssetInitializer) -> GsResult<DepthProcedure> {

        let screen_dimension = initializer.screen_dimension();

        let camera = GsCameraFactory::config()
            .place_at(Point3::new(0.0, 0.0, 2.5))
            .screen_aspect_ratio(screen_dimension.width as f32 / screen_dimension.height as f32)
            .into_flight_camera();

        let vertex_data = VERTEX_DATA.to_vec();
        let index_data = INDEX_DATA.to_vec();

        let y_correction: Matrix4<f32> = Matrix4::new(
            1.0,  0.0, 0.0, 0.0,
            0.0, -1.0, 0.0, 0.0,
            0.0,  0.0, 0.5, 0.5,
            0.0,  0.0, 0.0, 1.0,
        );
        let ubo_data = vec![
            UboObject {
                projection: camera.proj_matrix(),
                view      : camera.view_matrix(),
                model     : Matrix4::identity(),
                y_correction,
            },
        ];

        let (vertex_buffer, index_buffer, ubo_buffer, buffer_storage) = {
            DepthProcedure::buffers(&initializer, &vertex_data, &index_data, &ubo_data)
        }?;

        let (depth_attachment, image_storage) = {
            DepthProcedure::image(&initializer, screen_dimension)
        }?;

        let (ubo_set, desc_storage) = {
            DepthProcedure::ubo(&initializer, &ubo_buffer)
        }?;

        let pipeline = {
            DepthProcedure::pipelines(&initializer, &ubo_set, &depth_attachment)
        }?;

        let present_availables = {
            DepthProcedure::sync_resources(&initializer, &pipeline)
        }?;

        let (command_pool, command_buffers) = {
            DepthProcedure::commands(&initializer, &pipeline, &vertex_buffer, &index_buffer, &ubo_set, index_data.len())
        }?;

        let procedure = DepthProcedure {
            index_data, ubo_data,
            buffer_storage, vertex_buffer, index_buffer, ubo_buffer,
            desc_storage, ubo_set,
            pipeline,
            depth_attachment, image_storage,
            command_pool, command_buffers,
            camera,
            present_availables,
        };

        Ok(procedure)
    }

    fn update_uniforms(&mut self) -> GsResult<()> {

        self.ubo_data[0].view = self.camera.view_matrix();

        self.buffer_storage.data_updater()?
            .update(&self.ubo_buffer, &self.ubo_data)?
            .finish()?;

        Ok(())
    }

    fn buffers(initializer: &AssetInitializer, vertex_data: &Vec<Vertex>, index_data: &Vec<vkuint>, ubo_data: &Vec<UboObject>) -> GsResult<(GsVertexBuffer, GsIndexBuffer, GsUniformBuffer, GsBufferRepository<Host>)> {

        // vertex, index and uniform buffer
        let mut buffer_allocator = GsBufferAllocator::new(initializer, BufferStorageType::HOST);

        let vertex_info = GsVertexBuffer::new(data_size!(Vertex), vertex_data.len());
        let vertex_index = buffer_allocator.assign(vertex_info)?;

        let index_info = GsIndexBuffer::new(index_data.len());
        let index_index = buffer_allocator.assign(index_info)?;

        let ubo_info = GsUniformBuffer::new(0, 1, data_size!(UboObject));
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

    fn image(initializer: &AssetInitializer, dimension: vkDim2D) -> GsResult<(GsDSAttachment, GsImageRepository<Device>)> {

        // depth attachment image
        let mut image_allocator = GsImageAllocator::new(initializer, ImageStorageType::DEVICE);

        let depth_attachment_info = GsDSAttachment::new(dimension, DepthStencilImageFormat::Depth32Bit);
        let image_index = image_allocator.assign(depth_attachment_info)?;

        let image_distributor = image_allocator.allocate()?;
        let depth_attachment = image_distributor.acquire(image_index);
        let image_storage = image_distributor.into_repository();

        Ok((depth_attachment, image_storage))
    }

    fn ubo(initializer: &AssetInitializer, ubo_buffer: &GsUniformBuffer) -> GsResult<(DescriptorSet, GsDescriptorRepository)> {

        // descriptor
        let mut descriptor_set_config = DescriptorSetConfig::new();
        descriptor_set_config.add_buffer_binding(ubo_buffer, GsPipelineStage::VERTEX);

        let mut descriptor_allocator = GsDescriptorAllocator::new(initializer);
        let desc_index = descriptor_allocator.assign(descriptor_set_config);

        let descriptor_distributor = descriptor_allocator.allocate()?;
        let ubo_set = descriptor_distributor.acquire(desc_index);

        let desc_storage = descriptor_distributor.into_repository();

        Ok((ubo_set, desc_storage))
    }

    fn pipelines(initializer: &AssetInitializer, ubo_set: &DescriptorSet, depth_image: &GsDSAttachment) -> GsResult<GsPipeline<Graphics>> {

        // shaders
        let vertex_shader = GsShaderCI::from_source(
            GsPipelineStage::VERTEX,
            Path::new(VERTEX_SHADER_SOURCE_PATH),
            None,
            "[Vertex Shader]");
        let fragment_shader = GsShaderCI::from_source(
            GsPipelineStage::FRAGMENT,
            Path::new(FRAGMENT_SHADER_SOURCE_PATH),
            None,
            "[Fragment Shader]");
        let shader_infos = vec![vertex_shader, fragment_shader];
        let vertex_input_desc = Vertex::desc();

        // pipeline
        let mut render_pass_builder = GsRenderPass::new(initializer);
        let first_subpass = render_pass_builder.new_subpass();

        let color_attachment = RenderAttachmentCI::<Present>::new(initializer);
        let depth_attachment = depth_image.attachment();

        render_pass_builder.add_attachment(color_attachment, first_subpass);
        render_pass_builder.add_attachment(depth_attachment, first_subpass);

        let dependency0 = RenderDependencyCI::new(SubpassStage::BeginExternal, SubpassStage::AtIndex(first_subpass))
            .stage(vk::PipelineStageFlags::BOTTOM_OF_PIPE, vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .access(vk::AccessFlags::MEMORY_READ, vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE)
            .with_flags(vk::DependencyFlags::BY_REGION);
        render_pass_builder.add_dependency(dependency0);

        let dependency1 = RenderDependencyCI::new(SubpassStage::AtIndex(first_subpass), SubpassStage::EndExternal)
            .stage(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT, vk::PipelineStageFlags::BOTTOM_OF_PIPE)
            .access(vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE, vk::AccessFlags::MEMORY_READ)
            .with_flags(vk::DependencyFlags::BY_REGION);
        render_pass_builder.add_dependency(dependency1);

        let render_pass = render_pass_builder.build()?;
        let depth_stencil = GsDepthStencilState::setup(GsDepthStencilPrefab::EnableDepth);

        let pipeline_config = GfxPipelineConfig::new(shader_infos, vertex_input_desc, render_pass, initializer.screen_dimension())
            .with_depth_stencil(depth_stencil)
            .with_descriptor_sets(&[ubo_set])
            .finish();

        let mut pipeline_builder = GfxPipelineBuilder::new(initializer)?;
        let graphics_pipeline = pipeline_builder.build(pipeline_config)?;

        Ok(graphics_pipeline)
    }

    fn sync_resources(initializer: &AssetInitializer, pipeline: &GsPipeline<Graphics>) -> GsResult<Vec<GsSemaphore>> {

        // sync
        let mut present_availables = Vec::with_capacity(pipeline.frame_count());
        for _ in 0..pipeline.frame_count() {
            let semaphore = GsSemaphore::new(initializer)?;
            present_availables.push(semaphore);
        }
        Ok(present_availables)
    }

    fn commands(initializer: &AssetInitializer, pipeline: &GsPipeline<Graphics>, vertex_buffer: &GsVertexBuffer, index_buffer: &GsIndexBuffer, ubo_set: &DescriptorSet, index_count: usize) -> GsResult<(GsCommandPool, Vec<GsCommandBuffer>)> {

        let command_pool = GsCommandPool::new(initializer, DeviceQueueIdentifier::Graphics)?;
        let mut command_buffers = vec![];

        let command_buffer_count = pipeline.frame_count();
        let raw_commands = command_pool.allocate(CmdBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command) in raw_commands.into_iter().enumerate() {
            let mut recorder = GsCmdRecorder::<Graphics>::new(initializer, pipeline, command);

            recorder.begin_record(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE)?
                .begin_render_pass(pipeline, frame_index)
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

impl GraphicsRoutine for DepthProcedure {

    fn draw(&mut self, device: &GsDevice, device_available: &GsFence, image_available: &GsSemaphore, image_index: usize, _: f32) -> GsResult<&GsSemaphore> {

        self.update_uniforms()?;

        let submit_info = QueueSubmitBundle {
            wait_semaphores: &[image_available],
            sign_semaphores: &[&self.present_availables[image_index]],
            wait_stages    : &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT],
            commands       : &[&self.command_buffers[image_index]],
        };

        device.logic.submit_single(&submit_info, Some(device_available), DeviceQueueIdentifier::Graphics)?;

        return Ok(&self.present_availables[image_index])
    }

    fn reload_res(&mut self, initializer: AssetInitializer) -> GsResult<()> {

        self.pipeline = DepthProcedure::pipelines(&initializer, &self.ubo_set, &self.depth_attachment)?;

        self.present_availables = DepthProcedure::sync_resources(&initializer, &self.pipeline)?;

        let (command_pool, command_buffers) = DepthProcedure::commands(&initializer, &self.pipeline, &self.vertex_buffer, &self.index_buffer, &self.ubo_set, self.index_data.len())?;
        self.command_pool = command_pool;
        self.command_buffers = command_buffers;

        Ok(())
    }

    fn react_input(&mut self, inputer: &ActionNerve, delta_time: f32) -> SceneAction {

        if inputer.is_key_pressed(GsKeycode::ESCAPE) {
            return SceneAction::Terminal
        }

        self.camera.react_input(inputer, delta_time);

        SceneAction::Rendering
    }
}
