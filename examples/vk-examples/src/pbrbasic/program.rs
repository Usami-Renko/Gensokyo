
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
use super::data::{ Vertex, UBOMatrices, UboParams, ObjPosPushBlock, MaterialPushBlock };
use super::data::MATERIAL_DATA;

use nalgebra::{ Matrix4, Point3, Vector3, Vector4 };
use std::path::Path;
type Vector3F = Vector3<f32>;

const VERTEX_SHADER_SOURCE_PATH  : &'static str = "src/pbrbasic/pbr.vert.glsl";
const FRAGMENT_SHADER_SOURCE_PATH: &'static str = "src/pbrbasic/pbr.frag.glsl";
const MODEL_PATH: &'static str = "modles/geosphere.gltf";
const GRID_DIM: usize = 7;
const MATERIAL_INDEX: usize = 0;

pub struct VulkanExample {

    lights  : [Vector4<f32>; 4],
    ubo_data: Vec<UBOMatrices>,

    model_entity: GsglTFEntity,
    #[allow(dead_code)]
    model_repository: GsBufferRepository<Device>,

    push_ranges : Vec<GsPushConstantRange>,

    ubo_matrices: GsUniformBuffer,
    ubo_params  : GsUniformBuffer,
    ubo_storage : GsBufferRepository<Host>,

    pipeline: GsPipeline<Graphics>,

    ubo_set     : DescriptorSet,
    #[allow(dead_code)]
    desc_storage: GsDescriptorRepository,

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
        camera.set_move_speed(20.0);

        let view_port = CmdViewportInfo::from(screen_dimension);
        let scissor = CmdScissorInfo::from(screen_dimension);

        let ubo_data = vec![
            UBOMatrices {
                projection: camera.proj_matrix(),
                view      : camera.view_matrix(),
                model     : Matrix4::identity(),
                camera_pos: camera.current_position(),
                y_correction: Y_CORRECTION.clone(),
            },
        ];
        const P: f32 = 15.0;
        let lights = [
            Vector4::new(-P, -P * 0.5, -P, 1.0),
            Vector4::new(-P, -P * 0.5,  P, 1.0),
            Vector4::new( P, -P * 0.5,  P, 1.0),
            Vector4::new( P, -P * 0.5, -P, 1.0),
        ];

        let (model_entity, model_repository) = {
            VulkanExample::load_model(&initializer)
        }?;

        let (ubo_matrices, ubo_params, ubo_storage) = {
            VulkanExample::uniform_buffers(&initializer)
        }?;

        let push_ranges = VulkanExample::push_constants()?;

        let (depth_attachment, image_storage) = {
            VulkanExample::image(&initializer, screen_dimension)
        }?;

        let (ubo_set, desc_storage) = {
            VulkanExample::ubo(&initializer, &ubo_matrices, &ubo_params)
        }?;

        let pipeline = {
            VulkanExample::pipelines(&initializer, &ubo_set, push_ranges.clone(), &depth_attachment)
        }?;

        let present_availables = {
            VulkanExample::sync_resources(&initializer, &pipeline)
        }?;

        let (command_pool, command_buffers) = {
            VulkanExample::commands(&initializer, &pipeline, &model_entity, &ubo_set, &view_port, &scissor)
        }?;

        let procedure = VulkanExample {
            ubo_data, lights,
            model_entity, model_repository,
            ubo_matrices, ubo_params, push_ranges, ubo_storage,
            desc_storage, ubo_set,
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

        if self.is_toggle_event {

            // Update UBOMatrices
            self.ubo_data[0].projection = self.camera.proj_matrix();
            self.ubo_data[0].view = self.camera.view_matrix();
            self.ubo_data[0].camera_pos = self.camera.current_position();

            // Update lights
            const P: f32 = 15.0;
            self.lights = [
                Vector4::new(-P, -P * 0.5, -P, 1.0),
                Vector4::new(-P, -P * 0.5,  P, 1.0),
                Vector4::new( P, -P * 0.5,  P, 1.0),
                Vector4::new( P, -P * 0.5, -P, 1.0),
            ];

            // Update data in memory.
            self.ubo_storage.data_updater()?
                .update(&self.ubo_matrices, &self.ubo_data)?
                .update(&self.ubo_params, &self.lights)?
                .finish()?;
        }

        Ok(())
    }

    fn load_model(initializer: &AssetInitializer) -> GsResult<(GsglTFEntity, GsBufferRepository<Device>)> {

        // allocate model data buffer.
        let gltf_importer = GsglTFImporter::new(initializer);
        let (mut model_entity, model_data) = gltf_importer.load(Path::new(MODEL_PATH))?;

        let mut model_allocator = GsBufferAllocator::new(initializer, BufferStorageType::DEVICE);
        let model_vertex_index = model_allocator.assign_v2(&model_data.vertex_allot_delegate())?;
        let model_distributor = model_allocator.allocate()?;
        model_entity.acquire_vertex(model_vertex_index, &model_distributor);
        
        let mut model_repository = model_distributor.into_repository();
        model_repository.data_uploader()?
            .upload_v2(&model_entity.vertex_upload_delegate().unwrap(), &model_data)?
            .finish()?;

        Ok((model_entity, model_repository))
    }
    
    fn uniform_buffers(initializer: &AssetInitializer) -> GsResult<(GsUniformBuffer, GsUniformBuffer, GsBufferRepository<Host>)> {

        let mut ubo_allocator = GsBufferAllocator::new(initializer, BufferStorageType::HOST);

        // allocate uniform data buffer.
        // refer to `layout (binding = 0) uniform UBO` in pbr.frag.glsl.
        let ubo_matrix_info = GsUniformBuffer::new(0, data_size!(UBOMatrices));
        let ubo_matrix_index = ubo_allocator.assign(ubo_matrix_info)?;
        // refer to `layout (binding = 1) uniform UBOShared` in pbr.frag.glsl.
        let ubo_params_info = GsUniformBuffer::new(1, data_size!(UboParams));
        let ubo_params_index = ubo_allocator.assign(ubo_params_info)?;

        let ubo_distributor = ubo_allocator.allocate()?;

        let matrix_buffer = ubo_distributor.acquire(ubo_matrix_index);
        let params_buffer = ubo_distributor.acquire(ubo_params_index);

        let ubo_repository = ubo_distributor.into_repository();

        Ok((matrix_buffer, params_buffer, ubo_repository))
    }

    fn push_constants() -> GsResult<Vec<GsPushConstantRange>> {

        let ranges = vec![
            // refer to `layout(push_constant) uniform PushConsts` in pbr.vert.glsl.
            GsPushConstantRange::new(GsPipelineStage::VERTEX, 0, data_size!(Vector3F)),
            // refer to `layout(push_constant) uniform PushConsts` in pbr.frag.glsl.
            GsPushConstantRange::new(GsPipelineStage::FRAGMENT, data_size!(Vector3F), data_size!(MaterialPushBlock)),
        ];

        Ok(ranges)
    }

    fn ubo(initializer: &AssetInitializer, ubo_matrices: &GsUniformBuffer, ubo_params: &GsUniformBuffer) -> GsResult<(DescriptorSet, GsDescriptorRepository)> {

        // descriptor
        let mut descriptor_set_config = DescriptorSetConfig::new();
        descriptor_set_config.add_buffer_binding(ubo_matrices, GsPipelineStage::VERTEX | GsPipelineStage::FRAGMENT);
        descriptor_set_config.add_buffer_binding(ubo_params, GsPipelineStage::FRAGMENT);

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

    fn pipelines(initializer: &AssetInitializer, ubo_set: &DescriptorSet, ranges: Vec<GsPushConstantRange>, depth_image: &GsDSAttachment) -> GsResult<GsPipeline<Graphics>> {

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
        let mut rasterization = GsRasterizerState::setup(RasterizerPrefab::Common);
        rasterization.set_front_face(vk::FrontFace::COUNTER_CLOCKWISE); // TODO: Fix Clockwise different to tutorial.

        let pipeline_config = GfxPipelineConfig::new(shader_infos, vertex_input_desc, render_pass, initializer.screen_dimension())
            .with_depth_stencil(depth_stencil)
            .with_viewport(ViewportStateType::Dynamic { count: 1 })
            .with_rasterizer(rasterization)
            .with_push_constants(ranges)
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

    fn commands(initializer: &AssetInitializer, pipeline: &GsPipeline<Graphics>, model_entity: &GsglTFEntity, ubo_set: &DescriptorSet, view_port: &CmdViewportInfo, scissor: &CmdScissorInfo) -> GsResult<(GsCommandPool, Vec<GsCommandBuffer>)> {

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

            VulkanExample::record_commands(&recorder, model_entity, ubo_set)?;

            recorder.end_render_pass();

            let command_recorded = recorder.end_record()?;
            command_buffers.push(command_recorded);
        }

        Ok((command_pool, command_buffers))
    }

    fn record_commands(recorder: &GsCmdRecorder<Graphics>, model: &GsglTFEntity, ubo_set: &DescriptorSet) -> GsResult<()> {

        let model_render_params = GsglTFRenderParams {
            is_use_vertex        : true,
            is_use_node_transform: false,
            is_push_materials    : false,
            material_stage: GsPipelineStage::VERTEX,
        };

        // select a material from candidate materials.
        let mut mat = MATERIAL_DATA[MATERIAL_INDEX].clone();
        mat.metallic = 1.0;

        for y in 0..GRID_DIM {
            for x in 0..GRID_DIM {

                // upload push constant in pbr.vert.glsl.
                let pos = ObjPosPushBlock {
                    pos: [((x as f32) - (GRID_DIM as f32 / 2.0)) * 2.5, 0.0, 0.0],
                };
                let pos_data = bincode::serialize(&pos).map_err(GsError::serialize)?; // serialize data to bytes.
                let pos_size = data_size!(pos_data, u8);
                recorder.push_constants(GsPipelineStage::VERTEX, 0, &pos_data);

                // upload push constant in pbr.frag.glsl.
                mat.metallic  = nalgebra::clamp(x as f32 / (GRID_DIM - 1) as f32,  0.1, 1.0);
                mat.roughness = nalgebra::clamp(y as f32 / (GRID_DIM - 1) as f32, 0.05, 1.0);

                let mat_data = bincode::serialize(&mat).map_err(GsError::serialize)?; // serialize data to bytes.
                recorder.push_constants(GsPipelineStage::FRAGMENT, pos_size, &mat_data);

                // draw the model.
                model.record_command(recorder, ubo_set, &[], Some(model_render_params.clone()))?;

                return Ok(())
            }
        }

        Ok(())
    }
}

impl GraphicsRoutine for VulkanExample {

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

        self.pipeline = VulkanExample::pipelines(&initializer, &self.ubo_set, self.push_ranges.clone(), &self.depth_attachment)?;

        self.present_availables = VulkanExample::sync_resources(&initializer, &self.pipeline)?;

        let (command_pool, command_buffers) = VulkanExample::commands(&initializer, &self.pipeline, &self.model_entity, &self.ubo_set, &self.view_port, &self.scissor)?;
        self.command_pool = command_pool;
        self.command_buffers = command_buffers;

        Ok(())
    }

    fn react_input(&mut self, inputer: &ActionNerve, delta_time: f32) -> SceneAction {

        if inputer.is_key_active() || inputer.is_mouse_active() {

            if inputer.is_key_pressed(GsKeycode::ESCAPE) {
                return SceneAction::Terminal
            }

            self.camera.react_input(inputer, delta_time);
            self.is_toggle_event = true;
        } else {
            self.is_toggle_event = false;
        }

        SceneAction::Rendering
    }
}
