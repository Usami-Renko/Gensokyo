
use ash::vk;
use gs::prelude::*;
use gsvk::prelude::common::*;
use gsvk::prelude::buffer::*;
use gsvk::prelude::image::*;
use gsvk::prelude::descriptor::*;
use gsvk::prelude::pipeline::*;
use gsvk::command::*;
use gsvk::sync::*;
use gsma::data_size;

use nalgebra::{ Matrix4, Point3 };

use std::path::Path;
use std::marker::PhantomData;
use crate::{ UboObject, FilePathConstants, ShaderInputDefination };

pub struct GltfModelViewer<T: ShaderInputDefination> {

    phantom_type: PhantomData<T>,

    model_entity: GsGltfEntity,
    #[allow(dead_code)]
    model_repository: GsGltfRepository<Device>,

    ubo_data: Vec<UboObject>,
    ubo_storage: GsBufferRepository<Host>,
    ubo_buffer : GsUniformBlock,

    #[allow(dead_code)]
    desc_storage: GsDescriptorRepository,
    ubo_set: DescriptorSet,

    depth_attachment: GsDepthStencilAttachment,
    #[allow(dead_code)]
    image_storage   : GsImageRepository<Device>,

    pipeline: GsGraphicsPipeline,

    command_pool   : GsCommandPool,
    command_buffers: Vec<GsCommandBuffer>,

    paths: FilePathConstants,
    camera: GsFlightCamera,

    present_availables: Vec<GsSemaphore>,
}

impl<T: ShaderInputDefination> GltfModelViewer<T> {

    pub fn new(loader: AssetsLoader, paths: FilePathConstants) -> Result<GltfModelViewer<T>, ProcedureError> {

        let screen_dimension = loader.screen_dimension();
        let camera = GsCameraFactory::config()
            .place_at(Point3::new(0.0, 0.0, 2.5))
            .screen_aspect_ratio(screen_dimension.width as f32 / screen_dimension.height as f32)
            .into_flight_camera();

        let (model_entity, model_repository) = loader.assets(|kit| {
            Self::load_model(kit, &paths)
        })?;

        let ubo_data = vec![
            UboObject {
                projection: camera.proj_matrix(),
                view      : camera.view_matrix(),
                model     : Matrix4::identity(),
            }
        ];
        let (ubo_buffer, ubo_storage, ubo_set, desc_storage) = loader.assets(|kit| {
            Self::ubo(kit, &ubo_data)
        })?;

        let (depth_attachment, image_storage) = loader.assets(|kit| {
            Self::image(kit)
        })?;

        let pipeline = loader.pipelines(|kit| {
            Self::pipelines(kit, &paths, &ubo_set, &depth_attachment)
        })?;

        let present_availables = loader.syncs(|kit| {
            Self::sync_resources(kit, &pipeline)
        })?;

        let (command_pool, command_buffers) = loader.commands(|kit| {
            Self::commands(kit, &pipeline, &ubo_set, &model_entity)
        })?;

        let procedure = GltfModelViewer {
            phantom_type: PhantomData,
            model_entity, model_repository,
            ubo_data, ubo_storage, ubo_buffer, ubo_set, desc_storage,
            depth_attachment, image_storage,
            pipeline,
            command_pool, command_buffers,
            paths, camera,
            present_availables,
        };

        Ok(procedure)
    }

    fn update_uniforms(&mut self) -> Result<(), ProcedureError> {

        self.ubo_data[0].view  = self.camera.view_matrix();

        self.ubo_storage.data_updater()?
            .upload(&self.ubo_buffer, &self.ubo_data)?
            .finish()?;

        Ok(())
    }

    fn load_model(kit: AllocatorKit, paths: &FilePathConstants) -> Result<(GsGltfEntity, GsGltfRepository<Device>), ProcedureError> {

        let model_data_source = GsGltfImporter::load(Path::new(paths.model_path))?;
        let mut model_allocator = kit.gltf_allocator(BufferStorageType::DEVICE);

        let model_index = model_allocator.append_model(&model_data_source)?;
        let model_distributor = model_allocator.allocate()?;

        let model_entity = model_distributor.acquire_model(model_index);
        let mut model_repository = model_distributor.into_repository();

        model_repository.data_uploader()?
            .upload(&model_entity, &model_data_source)?
            .finish()?;

        Ok((model_entity, model_repository))
    }

    fn ubo(kit: AllocatorKit, ubo_data: &Vec<UboObject>) -> Result<(GsUniformBlock, GsBufferRepository<Host>, DescriptorSet, GsDescriptorRepository), ProcedureError> {

        // allocate uniform data buffer.
        let mut buffer_allocator = kit.buffer(BufferStorageType::HOST);
        let ubo_info = UniformBlockInfo::new(0, 1, data_size!(UboObject));
        let ubo_index = buffer_allocator.append_buffer(ubo_info)?;
        let buffer_distributor = buffer_allocator.allocate()?;
        let ubo_buffer = buffer_distributor.acquire_uniform(ubo_index)?;

        let mut ubo_storage = buffer_distributor.into_repository();
        ubo_storage.data_uploader()?
            .upload(&ubo_buffer, ubo_data)?
            .finish()?;

        // allocate uniform descriptor.
        let mut descriptor_set_config = DescriptorSetConfig::init(vk::DescriptorSetLayoutCreateFlags::empty());
        descriptor_set_config.add_buffer_binding(&ubo_buffer, GsDescBindingStage::VERTEX);

        let mut descriptor_allocator = kit.descriptor(vk::DescriptorPoolCreateFlags::empty());
        let desc_index = descriptor_allocator.append_set(descriptor_set_config);

        let mut descriptor_distributor = descriptor_allocator.allocate()?;
        let ubo_set = descriptor_distributor.acquire_set(desc_index);

        let desc_storage = descriptor_distributor.into_repository();

        Ok((ubo_buffer, ubo_storage, ubo_set, desc_storage))
    }

    fn image(kit: AllocatorKit) -> Result<(GsDepthStencilAttachment, GsImageRepository<Device>), ProcedureError> {

        // depth attachment image
        let mut image_allocator = kit.image(ImageStorageType::DEVICE);

        let mut depth_attachment_info = DepthStencilAttachmentInfo::new(kit.swapchain_dimension(), DepthStencilImageFormat::Depth32Bit);
        image_allocator.append_depth_stencil_image(&mut depth_attachment_info)?;

        let image_distributor = image_allocator.allocate()?;
        let depth_attachment = image_distributor.acquire_depth_stencil_image(depth_attachment_info)?;
        let image_storage = image_distributor.into_repository();

        Ok((depth_attachment, image_storage))
    }

    fn pipelines(kit: PipelineKit, paths: &FilePathConstants, ubo_set: &DescriptorSet, depth_image: &GsDepthStencilAttachment) -> Result<GsGraphicsPipeline, ProcedureError> {

        // shaders
        let vertex_shader = GsShaderInfo::from_source(
            vk::ShaderStageFlags::VERTEX,
            Path::new(paths.vertex_shader),
            None,
            "[Vertex Shader]");
        let fragment_shader = GsShaderInfo::from_source(
            vk::ShaderStageFlags::FRAGMENT,
            Path::new(paths.framment_shader),
            None,
            "[Fragment Shader]");
        let shader_infos = vec![
            vertex_shader,
            fragment_shader,
        ];
        let vertex_input_desc = T::desc();

        // pipeline
        let mut render_pass_builder = kit.pass_builder();
        let first_subpass = render_pass_builder.new_subpass();

        let color_attachment = kit.present_attachment();
        let depth_attachment = depth_image.to_subpass_attachment();

        let _ = render_pass_builder.add_attachemnt(color_attachment, first_subpass);
        let _ = render_pass_builder.add_attachemnt(depth_attachment, first_subpass);

        render_pass_builder.set_depth_attachment(depth_image);

        let dependency = kit.subpass_dependency(SubpassStage::External, SubpassStage::AtIndex(first_subpass))
            .stage(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT, vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .access(vk::AccessFlags::empty(), vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE);
        render_pass_builder.add_dependenty(dependency);

        let render_pass = render_pass_builder.build()?;
        let depth_stencil = GsDepthStencilState::setup(GsDepthStencilPrefab::EnableDepth);

        let pipeline_config = kit.pipeline_config(shader_infos, vertex_input_desc, render_pass)
            .with_depth_stencil(depth_stencil)
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

    fn commands(kit: CommandKit, graphics_pipeline: &GsGraphicsPipeline, ubo_set: &DescriptorSet, model: &GsGltfEntity) -> Result<(GsCommandPool, Vec<GsCommandBuffer>), ProcedureError> {

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
                .bind_descriptor_sets(graphics_pipeline, 0, &[ubo_set]);

            model.record_command(&recorder);

            recorder.end_render_pass();

            let command_recorded = recorder.end_record()?;
            command_buffers.push(command_recorded);
        }

        Ok((command_pool, command_buffers))
    }
}

impl<T: ShaderInputDefination> GraphicsRoutine for GltfModelViewer<T> {

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
            Self::pipelines(kit, &self.paths, &self.ubo_set, &self.depth_attachment)
        })?;

        self.present_availables = loader.syncs(|kit| {
            Self::sync_resources(kit, &self.pipeline)
        })?;

        let (command_pool, command_buffers) = loader.commands(|kit| {
            Self::commands(kit, &self.pipeline, &self.ubo_set, &self.model_entity)
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
