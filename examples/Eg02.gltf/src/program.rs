
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

use nalgebra::{ Matrix4, Point3 };

use std::path::Path;
use std::marker::PhantomData;
use crate::{ UboObject, FilePathConstants, ShaderInputDefinition };

pub struct GltfModelViewer<T: ShaderInputDefinition> {

    phantom_type: PhantomData<T>,

    dst_model: GsglTFEntity,
    #[allow(dead_code)]
    model_repository: GsBufferRepository<Device>,

    ubo_data: Vec<UboObject>,
    ubo_storage: GsBufferRepository<Host>,
    ubo_buffer : GsUniformBuffer,

    #[allow(dead_code)]
    desc_storage: GsDescriptorRepository,
    ubo_set: DescriptorSet,

    depth_attachment: GsDSAttachment,
    #[allow(dead_code)]
    image_storage: GsImageRepository<Device>,

    pipeline: GsPipeline<Graphics>,

    command_pool   : GsCommandPool,
    command_buffers: Vec<GsCommandBuffer>,

    paths: FilePathConstants,
    camera: GsFlightCamera,

    present_availables: Vec<GsSemaphore>,
}

impl<T: ShaderInputDefinition> GltfModelViewer<T> {

    pub fn new(initializer: AssetInitializer, paths: FilePathConstants) -> GsResult<GltfModelViewer<T>> {

        let screen_dimension = initializer.screen_dimension();
        let camera = GsCameraFactory::config()
            .place_at(Point3::new(0.0, 0.0, 2.5))
            .screen_aspect_ratio(screen_dimension.width as f32 / screen_dimension.height as f32)
            .into_flight_camera();

        let (ubo_buffer, ubo_storage, dst_model, model_repository) = Self::load_model(&initializer, &paths)?;

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
        let (ubo_set, desc_storage) = {
            Self::ubo(&initializer, &ubo_buffer, &dst_model)
        }?;

        let (depth_attachment, image_storage) = {
            Self::image(&initializer, screen_dimension)
        }?;

        let pipeline = {
            Self::pipelines(&initializer, &paths, &ubo_set, &dst_model, &depth_attachment)
        }?;

        let present_availables = {
            Self::sync_resources(&initializer, &pipeline)
        }?;

        let (command_pool, command_buffers) = {
            Self::commands(&initializer, &pipeline, &ubo_set, &dst_model)
        }?;

        let procedure = GltfModelViewer {
            phantom_type: PhantomData,
            dst_model, model_repository,
            ubo_data, ubo_storage, ubo_buffer, ubo_set, desc_storage,
            depth_attachment, image_storage,
            pipeline,
            command_pool, command_buffers,
            paths, camera,
            present_availables,
        };

        Ok(procedure)
    }

    fn update_uniforms(&mut self) -> GsResult<()> {

        //self.ubo_data[0].model = self.camera.object_model_transformation();
        self.ubo_data[0].view  = self.camera.view_matrix();

        self.ubo_storage.data_updater()?
            .update(&self.ubo_buffer, &self.ubo_data)?
            .finish()?;

        Ok(())
    }

    fn load_model(initializer: &AssetInitializer, paths: &FilePathConstants) -> GsResult<(GsUniformBuffer, GsBufferRepository<Host>, GsglTFEntity, GsBufferRepository<Device>)> {

        // generate buffer allocator.
        let mut ubo_allocator = GsBufferAllocator::new(initializer, BufferStorageType::HOST);
        let mut model_allocator = GsBufferAllocator::new(initializer, BufferStorageType::DEVICE);

        // allocate uniform data buffer.
        let ubo_info = GsUniformBuffer::new(0, data_size!(UboObject));
        let ubo_index = ubo_allocator.assign(ubo_info)?;

        // load and allocate model data.
        let gltf_importer = GsglTFImporter::new(initializer);
        let (mut model_entity, model_data) = gltf_importer.load(Path::new(paths.model_path))?;

        let model_vertex_index = model_allocator.assign_v2(&model_data.vertex_allot_delegate())?;
        let model_uniform_index = ubo_allocator.assign_v2(&model_data.uniform_allot_delegate(1))?;

        // allocate memory.
        let ubo_distributor = ubo_allocator.allocate()?;
        let model_distributor = model_allocator.allocate()?;

        // get vertex and uniform buffer.
        model_entity.acquire_vertex(model_vertex_index, &model_distributor);
        model_entity.acquire_uniform(model_uniform_index, &ubo_distributor);

        let ubo_buffer = ubo_distributor.acquire(ubo_index);
        let mut ubo_repository = ubo_distributor.into_repository();
        let mut model_repository = model_distributor.into_repository();

        // upload actual model data to memory.
        model_repository.data_uploader()?
            .upload_v2(&model_entity.vertex_upload_delegate().unwrap(), &model_data)?
            .finish()?;
        ubo_repository.data_uploader()?
            .upload_v2(&model_entity.uniform_upload_delegate().unwrap(), &model_data)?
            .finish()?;

        Ok((ubo_buffer, ubo_repository, model_entity, model_repository))
    }

    fn ubo(initializer: &AssetInitializer, ubo_buffer: &GsUniformBuffer, model: &GsglTFEntity) -> GsResult<(DescriptorSet, GsDescriptorRepository)> {

        // allocate uniform descriptor.
        let mut descriptor_set_config = DescriptorSetConfig::new();
        descriptor_set_config.add_buffer_binding(ubo_buffer, GsPipelineStage::VERTEX);
        descriptor_set_config.add_buffer_binding(model, GsPipelineStage::VERTEX);

        let mut descriptor_allocator = GsDescriptorAllocator::new(initializer);
        let desc_index = descriptor_allocator.assign(descriptor_set_config);

        let descriptor_distributor = descriptor_allocator.allocate()?;
        let ubo_set = descriptor_distributor.acquire(desc_index);

        let desc_storage = descriptor_distributor.into_repository();

        Ok((ubo_set, desc_storage))
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

    fn pipelines(initializer: &AssetInitializer, paths: &FilePathConstants, ubo_set: &DescriptorSet, model: &GsglTFEntity, depth_image: &GsDSAttachment) -> GsResult<GsPipeline<Graphics>> {

        // shaders
        let vertex_shader = GsShaderCI::from_source(GsPipelineStage::VERTEX, Path::new(paths.vertex_shader), None, "[Vertex Shader]");
        let fragment_shader = GsShaderCI::from_source(GsPipelineStage::FRAGMENT, Path::new(paths.fragment_shader), None, "[Fragment Shader]");
        let shader_infos = vec![vertex_shader, fragment_shader];
        let vertex_input_desc = T::desc();

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
            .with_push_constants(vec![model.pushconst_description(GsPipelineStage::FRAGMENT)])
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

    fn commands(initializer: &AssetInitializer, pipeline: &GsPipeline<Graphics>, ubo_set: &DescriptorSet, model: &GsglTFEntity) -> GsResult<(GsCommandPool, Vec<GsCommandBuffer>)> {

        let command_pool = GsCommandPool::new(initializer, DeviceQueueIdentifier::Graphics)?;
        let mut command_buffers = vec![];

        let command_buffer_count = pipeline.frame_count();
        let raw_commands = command_pool.allocate(CmdBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command) in raw_commands.into_iter().enumerate() {
            let mut recorder = GsCmdRecorder::<Graphics>::new(initializer, pipeline, command);

            recorder.begin_record(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE)?
                .begin_render_pass(pipeline, frame_index)
                .bind_pipeline();

            model.record_command(&recorder, ubo_set, &[], None)?;

            recorder.end_render_pass();

            let command_recorded = recorder.end_record()?;
            command_buffers.push(command_recorded);
        }

        Ok((command_pool, command_buffers))
    }
}

impl<T: ShaderInputDefinition> GraphicsRoutine for GltfModelViewer<T> {

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

        self.pipeline = Self::pipelines(&initializer, &self.paths, &self.ubo_set, &self.dst_model, &self.depth_attachment)?;

        self.present_availables = Self::sync_resources(&initializer, &self.pipeline)?;

        let (command_pool, command_buffers) = Self::commands(&initializer, &self.pipeline, &self.ubo_set, &self.dst_model)?;
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
