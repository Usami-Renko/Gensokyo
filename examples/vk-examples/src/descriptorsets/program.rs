
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

use vk_examples::{ Y_CORRECTION, DEFAULT_CLEAR_COLOR };
use super::data::{ Vertex, UBOMatrices, CubeResources };

use nalgebra::{ Matrix4, Point3, Vector3 };
use std::path::Path;

const VERTEX_SHADER_SOURCE_PATH  : &'static str = "src/descriptorsets/cube.vert.glsl";
const FRAGMENT_SHADER_SOURCE_PATH: &'static str = "src/descriptorsets/cube.frag.glsl";
const MODEL_PATH: &'static str = "models/cube.gltf";
const TEXTURE1_PATH: &'static str = "textures/crate01_color_height_rgba.png";
const TEXTURE2_PATH: &'static str = "textures/crate02_color_height_rgba.png";

pub struct VulkanExample {

    model_entity: GsglTFEntity,
    cubes: CubeResources,

    #[allow(dead_code)]
    model_repository: GsBufferRepository<Device>,
    ubo_storage : GsBufferRepository<Host>,
    #[allow(dead_code)]
    desc_storage: GsDescriptorRepository,

    pipeline: GsPipeline<Graphics>,

    depth_attachment: GsDSAttachment,
    #[allow(dead_code)]
    image_storage   : GsImageRepository<Device>,

    command_pool   : GsCommandPool,
    command_buffers: Vec<GsCommandBuffer>,

    view_port: CmdViewportInfo,
    scissor  : CmdScissorInfo,

    camera: GsFlightCamera,
    present_availables: Vec<GsSemaphore>,

    is_toggle_event: bool,
}

impl VulkanExample {

    pub fn new(initializer: AssetInitializer) -> GsResult<VulkanExample> {

        let screen_dimension = initializer.screen_dimension();

        let mut camera = GsCameraFactory::config()
            .place_at(Point3::new(0.0, 0.0, 2.5))
            .screen_aspect_ratio(screen_dimension.width as f32 / screen_dimension.height as f32)
            .into_flight_camera();
        camera.set_move_speed(5.0);

        let view_port = CmdViewportInfo::from(screen_dimension);
        let scissor = CmdScissorInfo::from(screen_dimension);

        let ubo_data = [
            vec![
                UBOMatrices {
                    projection: camera.proj_matrix(),
                    model     : Matrix4::new_translation(&Vector3::new(-2.0, 0.0, 0.0)),
                    view      : camera.view_matrix(),
                    y_correction: Y_CORRECTION.clone(),
                },
            ],
            vec![
                UBOMatrices {
                    projection: camera.proj_matrix(),
                    model     : Matrix4::new_translation(&Vector3::new(1.5, 0.5, 0.0)),
                    view      : camera.view_matrix(),
                    y_correction: Y_CORRECTION.clone(),
                },
            ],
        ];

        let (model_entity, model_repository, ubo_buffers, ubo_storage) = {
            VulkanExample::load_model(&initializer, &ubo_data)
        }?;

        let (sample_images, depth_attachment, image_storage) = {
            VulkanExample::image(&initializer, screen_dimension)
        }?;

        let (ubo_sets, desc_storage) = {
            VulkanExample::ubo(&initializer, &model_entity, &sample_images, &ubo_buffers)
        }?;

        let push_consts = model_entity.pushconst_description(GsPipelineStage::VERTEX);
        let pipeline = {
            VulkanExample::pipelines(&initializer, push_consts, &[&ubo_sets[0], &ubo_sets[1]], &depth_attachment)
        }?;

        let present_availables = {
            VulkanExample::sync_resources(&initializer, &pipeline)
        }?;

        let cubes = CubeResources {
            matrices  : ubo_data,
            texture   : sample_images,
            ubo_set   : ubo_sets,
            ubo_buffer: ubo_buffers,
        };

        let (command_pool, command_buffers) = {
            VulkanExample::commands(&initializer, &pipeline, &model_entity, &cubes, &view_port, &scissor)
        }?;

        let procedure = VulkanExample {
            model_entity, cubes,
            model_repository, ubo_storage, desc_storage,
            pipeline,
            depth_attachment, image_storage,
            command_pool, command_buffers,
            camera, view_port, scissor,
            present_availables,
            is_toggle_event: false,
        };

        Ok(procedure)
    }

    fn update_uniforms(&mut self) -> GsResult<()> {

        // Update UBOMatrices uniform block.
        self.cubes.matrices[0][0].view = self.camera.view_matrix();
        self.cubes.matrices[1][0].view = self.camera.view_matrix();

        // Update data in memory.
        self.ubo_storage.data_updater()?
            .update(&self.cubes.ubo_buffer[0], &self.cubes.matrices[0])?
            .update(&self.cubes.ubo_buffer[1], &self.cubes.matrices[1])?
            .finish()?;

        Ok(())
    }

    fn load_model(initializer: &AssetInitializer, ubo_data: &[Vec<UBOMatrices>; 2]) -> GsResult<(GsglTFEntity, GsBufferRepository<Device>, [GsUniformBuffer; 2], GsBufferRepository<Host>)> {

        let mut model_allocator = GsBufferAllocator::new(initializer, BufferStorageType::DEVICE);
        let mut ubo_allocator = GsBufferAllocator::new(initializer, BufferStorageType::HOST);

        // allocate uniform data buffer.
        // refer to `layout (set = 0, binding = 0) uniform UBO` in cube.vert.
        let ubo_matrix_info1 = GsUniformBuffer::new(0, 1, data_size!(UBOMatrices));
        let ubo_matrix_info2 = ubo_matrix_info1.clone();
        let ubo_matrix_index1 = ubo_allocator.assign(ubo_matrix_info1)?; // ubo buffer for cube 0
        let ubo_matrix_index2 = ubo_allocator.assign(ubo_matrix_info2)?; // ubo buffer for cube 1

        // allocate model data buffer.
        let gltf_importer = GsglTFImporter::new(initializer);
        let (mut model_entity, model_data) = gltf_importer.load(Path::new(MODEL_PATH))?;

        let model_vertex_index = model_allocator.assign_v2(&model_data.vertex_allot_delegate())?;
        let model_uniform_index = ubo_allocator.assign_v2(&model_data.uniform_allot_delegate(1))?;

        let model_distributor = model_allocator.allocate()?;
        let ubo_distributor = ubo_allocator.allocate()?;

        model_entity.acquire_vertex(model_vertex_index, &model_distributor);
        model_entity.acquire_uniform(model_uniform_index, &ubo_distributor);
        
        let mut model_repository = model_distributor.into_repository();
        model_repository.data_uploader()?
            .upload_v2(&model_entity.vertex_upload_delegate().unwrap(), &model_data)?
            .finish()?;

        let cube0_ubo = ubo_distributor.acquire(ubo_matrix_index1);
        let cube1_ubo = ubo_distributor.acquire(ubo_matrix_index2);

        let mut ubo_repository = ubo_distributor.into_repository();
        ubo_repository.data_uploader()?
            .upload_v2(&model_entity.uniform_upload_delegate().unwrap(), &model_data)?
            .upload(&cube0_ubo, &ubo_data[0])?
            .upload(&cube1_ubo, &ubo_data[1])?
            .finish()?;

        Ok((model_entity, model_repository, [cube0_ubo, cube1_ubo], ubo_repository))
    }

    fn ubo(initializer: &AssetInitializer, model: &GsglTFEntity, textures: &[GsSampleImage; 2], ubo_buffers: &[GsUniformBuffer; 2]) -> GsResult<([DescriptorSet; 2], GsDescriptorRepository)> {

        let mut descriptor_allocator = GsDescriptorAllocator::new(initializer);

        // descriptor set for first cube.
        let mut descriptor_set_config = DescriptorSetConfig::new();
        descriptor_set_config.add_buffer_binding(&ubo_buffers[0], GsPipelineStage::VERTEX); // binding 0
        descriptor_set_config.add_buffer_binding(model, GsPipelineStage::VERTEX); // binding 1
        descriptor_set_config.add_image_binding(&textures[0], GsPipelineStage::FRAGMENT); // binding 2
        let cube0_desc_index = descriptor_allocator.assign(descriptor_set_config);

        // descriptor set for second cube.
        let mut descriptor_set_config = DescriptorSetConfig::new();
        descriptor_set_config.add_buffer_binding(&ubo_buffers[1], GsPipelineStage::VERTEX); // binding 0
        descriptor_set_config.add_buffer_binding(model, GsPipelineStage::VERTEX); // binding 1
        descriptor_set_config.add_image_binding(&textures[1], GsPipelineStage::FRAGMENT); // binding 2
        let cube1_desc_index = descriptor_allocator.assign(descriptor_set_config);

        // allocate descriptor set.
        let descriptor_distributor = descriptor_allocator.allocate()?;
        let cube0_ubo_set = descriptor_distributor.acquire(cube0_desc_index);
        let cube1_ubo_set = descriptor_distributor.acquire(cube1_desc_index);

        let desc_storage = descriptor_distributor.into_repository();

        Ok(([cube0_ubo_set, cube1_ubo_set], desc_storage))
    }

    fn image(initializer: &AssetInitializer, dimension: vkDim2D) -> GsResult<([GsSampleImage; 2], GsDSAttachment, GsImageRepository<Device>)> {

        let mut image_allocator = GsImageAllocator::new(initializer, ImageStorageType::DEVICE);

        // Depth Attachment
        let depth_attachment_info = GsDSAttachment::new(dimension, DepthStencilImageFormat::Depth32Bit);
        let depth_image_index = image_allocator.assign(depth_attachment_info)?;

        // Combined Sample Image
        let image_loader = ImageLoader::new(initializer);
        let image_storage1 = image_loader.load_2d(Path::new(TEXTURE1_PATH))?; // texture 1 for cube 1
        let image_storage2 = image_loader.load_2d(Path::new(TEXTURE2_PATH))?; // texture 2 for cube 2
        // refer to `layout (set = 0, binding = 2) sampler2D samplerColorMap` in cube.frag. Accessible from the fragment shader only.
        let image_info1 = GsSampleImage::new(2, 1, image_storage1, ImagePipelineStage::FragmentStage);
        let image_info2 = GsSampleImage::new(2, 1, image_storage2, ImagePipelineStage::FragmentStage);
        let sample_image_index1 = image_allocator.assign(image_info1)?;
        let sample_image_index2 = image_allocator.assign(image_info2)?;

        let image_distributor = image_allocator.allocate()?;

        let depth_attachment = image_distributor.acquire(depth_image_index);
        let sample_image1 = image_distributor.acquire(sample_image_index1);
        let sample_image2 = image_distributor.acquire(sample_image_index2);

        let image_storage = image_distributor.into_repository();

        Ok(([sample_image1, sample_image2], depth_attachment, image_storage))
    }

    fn pipelines(initializer: &AssetInitializer, push_consts: GsPushConstantRange, ubo_sets: &[&DescriptorSet; 2], depth_image: &GsDSAttachment) -> GsResult<GsPipeline<Graphics>> {

        // shaders
        let vertex_shader = GsShaderCI::from_source(GsPipelineStage::VERTEX, Path::new(VERTEX_SHADER_SOURCE_PATH), None, "[Vertex Shader]");
        let fragment_shader = GsShaderCI::from_source(GsPipelineStage::FRAGMENT, Path::new(FRAGMENT_SHADER_SOURCE_PATH), None, "[Fragment Shader]");
        let shader_infos = vec![vertex_shader, fragment_shader];
        let vertex_input_desc = Vertex::input_description();

        // pipeline
        let mut render_pass_builder = GsRenderPass::new(initializer);
        let first_subpass = render_pass_builder.new_subpass();

        let color_attachment = RenderAttachmentCI::<Present>::new(initializer)
            .op(vk::AttachmentLoadOp::CLEAR, vk::AttachmentStoreOp::STORE)
            .clear_value(DEFAULT_CLEAR_COLOR.clone());
        let depth_attachment = depth_image.attachment()
            .op(vk::AttachmentLoadOp::CLEAR, vk::AttachmentStoreOp::DONT_CARE);

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
            .with_viewport(ViewportStateType::Dynamic { count: 1 })
            .with_descriptor_sets(ubo_sets)
            .with_push_constants(vec![push_consts])
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

    fn commands(initializer: &AssetInitializer, pipeline: &GsPipeline<Graphics>, model_entity: &GsglTFEntity, cubes: &CubeResources, view_port: &CmdViewportInfo, scissor: &CmdScissorInfo) -> GsResult<(GsCommandPool, Vec<GsCommandBuffer>)> {

        let command_pool = GsCommandPool::new(initializer, DeviceQueueIdentifier::Graphics)?;
        let mut command_buffers = vec![];

        let command_buffer_count = pipeline.frame_count();
        let raw_commands = command_pool.allocate(CmdBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command) in raw_commands.into_iter().enumerate() {
            let mut recorder = GsCmdRecorder::<Graphics>::new(initializer, pipeline, command);

            recorder.begin_record(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE)?
                .begin_render_pass(pipeline, frame_index)
                .set_viewport(0, &[view_port.clone()])
                .set_scissor(0, &[scissor.clone()])
                .bind_pipeline();

            VulkanExample::record_commands(&recorder, model_entity, cubes)?;

            recorder.end_render_pass();

            let command_recorded = recorder.end_record()?;
            command_buffers.push(command_recorded);
        }

        Ok((command_pool, command_buffers))
    }

    fn record_commands(recorder: &GsCmdRecorder<Graphics>, model: &GsglTFEntity, cubes: &CubeResources) -> GsResult<()> {

        let model_render_params = GsglTFRenderParams {
            is_use_vertex        : true,
            is_use_node_transform: true,
            is_push_materials    : true,
            material_stage: GsPipelineStage::VERTEX,
        };

        // draw the model.
        model.record_command(recorder, &cubes.ubo_set[0], &[], Some(model_render_params.clone()))?;
        model.record_command(recorder, &cubes.ubo_set[1], &[], Some(model_render_params.clone()))?;

        Ok(())
    }
}

impl GraphicsRoutine for VulkanExample {

    fn draw(&mut self, device: &GsDevice, device_available: &GsFence, image_available: &GsSemaphore, image_index: usize, _: f32) -> GsResult<&GsSemaphore> {

        if self.is_toggle_event {
            self.update_uniforms()?;
        }

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

        let ubo_sets = &[&self.cubes.ubo_set[0], &self.cubes.ubo_set[1]];
        let push_consts = self.model_entity.pushconst_description(GsPipelineStage::VERTEX);

        self.pipeline = VulkanExample::pipelines(&initializer, push_consts, ubo_sets, &self.depth_attachment)?;

        self.present_availables = VulkanExample::sync_resources(&initializer, &self.pipeline)?;

        let (command_pool, command_buffers) = VulkanExample::commands(&initializer, &self.pipeline, &self.model_entity, &self.cubes, &self.view_port, &self.scissor)?;
        self.command_pool = command_pool;
        self.command_buffers = command_buffers;

        Ok(())
    }

    fn clean_routine(&mut self, device: &GsDevice) {

        // Remember to destroy sample image manually.
        for texture in self.cubes.texture.iter() {
            texture.destroy(device);
        }
    }

    fn react_input(&mut self, inputer: &ActionNerve, delta_time: f32) -> SceneAction {

        if inputer.is_key_active() || inputer.is_mouse_active() {

            if inputer.is_key_pressed(GsKeycode::ESCAPE) {
                return SceneAction::Terminal
            }

            self.is_toggle_event = true;
            self.camera.react_input(inputer, delta_time);
        } else {
            self.is_toggle_event = false;
        }

        SceneAction::Rendering
    }
}
