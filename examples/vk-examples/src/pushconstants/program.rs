
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
use super::data::{ Vertex, UBOVS, PushConstants };

use nalgebra::{ Matrix4, Point3 };
use std::path::Path;

const VERTEX_SHADER_SOURCE_PATH  : &'static str = "src/pushconstants/lights.vert";
const FRAGMENT_SHADER_SOURCE_PATH: &'static str = "src/pushconstants/lights.frag";
const MODEL_PATH: &'static str = "models/samplescene.gltf";
const TIMER: f32 = 0.10;

pub struct VulkanExample {

    ubo_data: Vec<UBOVS>,

    model_entity: GsglTFEntity,
    #[allow(dead_code)]
    model_repository: GsBufferRepository<Device>,

    push_range : GsPushConstantRange,

    ubo_buffer  : GsUniformBuffer,
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

    pub fn new(loader: AssetsLoader) -> GsResult<VulkanExample> {

        let screen_dimension = loader.screen_dimension();

        let mut camera = GsCameraFactory::config()
            .place_at(Point3::new(20.0, 20.0, 2.5))
            .screen_aspect_ratio(screen_dimension.width as f32 / screen_dimension.height as f32)
            .into_flight_camera();
        camera.set_move_speed(50.0);

        let view_port = CmdViewportInfo::from(screen_dimension);
        let scissor = CmdScissorInfo::from(screen_dimension);

        let ubo_data = vec![
            UBOVS {
                projection: camera.proj_matrix(),
                model     : Matrix4::identity(),
                view      : camera.view_matrix(),
                y_correction: Y_CORRECTION.clone(),
            },
        ];

        let (model_entity, model_repository, ubo_buffer, ubo_storage) = loader.assets(|kit| {
            VulkanExample::load_model(kit, &ubo_data)
        })?;

        let push_range = VulkanExample::push_constants();

        let (depth_attachment, image_storage) = loader.assets(|kit| {
            VulkanExample::image(kit, screen_dimension)
        })?;

        let (ubo_set, desc_storage) = loader.assets(|kit| {
            VulkanExample::ubo(kit, &model_entity, &ubo_buffer)
        })?;

        let pipeline = loader.pipelines(|kit| {
            VulkanExample::pipelines(kit, &ubo_set, push_range.clone(), &depth_attachment)
        })?;

        let present_availables = loader.syncs(|kit| {
            VulkanExample::sync_resources(kit, &pipeline)
        })?;

        let (command_pool, command_buffers) = loader.commands(|kit| {
            VulkanExample::commands(kit, &pipeline, &model_entity, &ubo_set, &view_port, &scissor)
        })?;

        let procedure = VulkanExample {
            ubo_data,
            model_entity, model_repository,
            ubo_buffer, push_range, ubo_storage,
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

        // Update UBOVS uniform block.
        self.ubo_data[0].view = self.camera.view_matrix();

        // Update data in memory.
        self.ubo_storage.data_updater()?
            .update(&self.ubo_buffer, &self.ubo_data)?
            .finish()?;

        Ok(())
    }

    fn load_model(kit: AllocatorKit, ubo_data: &Vec<UBOVS>) -> GsResult<(GsglTFEntity, GsBufferRepository<Device>, GsUniformBuffer, GsBufferRepository<Host>)> {

        let mut model_allocator = kit.buffer(BufferStorageType::DEVICE);
        let mut ubo_allocator = kit.buffer(BufferStorageType::HOST);

        // allocate uniform data buffer.
        // refer to `layout (binding = 0) uniform UBO` in pbr.frag.
        let ubo_vertex_info = GsBufUniformInfo::new(0, 1, data_size!(UBOVS));
        let ubo_vertex_index = ubo_allocator.assign(ubo_vertex_info)?;

        // allocate model data buffer.
        let gltf_importer = kit.gltf_loader();
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

        let ubo_buffer = ubo_distributor.acquire(ubo_vertex_index);
        let mut ubo_repository = ubo_distributor.into_repository();
        ubo_repository.data_uploader()?
            .upload_v2(&model_entity.uniform_upload_delegate().unwrap(), &model_data)?
            .upload(&ubo_buffer, ubo_data)?
            .finish()?;

        Ok((model_entity, model_repository, ubo_buffer, ubo_repository))
    }

    fn push_constants() -> GsPushConstantRange {

        GsPushConstantRange::new(GsPipelineStage::VERTEX, 0, data_size!(PushConstants))
    }

    fn ubo(kit: AllocatorKit, model: &GsglTFEntity, ubo_buffer: &GsUniformBuffer) -> GsResult<(DescriptorSet, GsDescriptorRepository)> {

        // descriptor
        let mut descriptor_set_config = DescriptorSetConfig::init();
        descriptor_set_config.add_buffer_binding(ubo_buffer, GsPipelineStage::VERTEX);
        descriptor_set_config.add_buffer_binding(model, GsPipelineStage::VERTEX);

        let mut descriptor_allocator = kit.descriptor(vk::DescriptorPoolCreateFlags::empty());
        let desc_index = descriptor_allocator.assign(descriptor_set_config);

        let descriptor_distributor = descriptor_allocator.allocate()?;
        let ubo_set = descriptor_distributor.acquire(desc_index);

        let desc_storage = descriptor_distributor.into_repository();

        Ok((ubo_set, desc_storage))
    }

    fn image(kit: AllocatorKit, dimension: vkDim2D) -> GsResult<(GsDSAttachment, GsImageRepository<Device>)> {

        // depth attachment image
        let mut image_allocator = kit.image(ImageStorageType::DEVICE);

        let depth_attachment_info = GsDSAttachmentInfo::new(dimension, DepthStencilImageFormat::Depth32Bit);
        let image_index = image_allocator.assign(depth_attachment_info)?;

        let image_distributor = image_allocator.allocate()?;
        let depth_attachment = image_distributor.acquire(image_index);
        let image_storage = image_distributor.into_repository();

        Ok((depth_attachment, image_storage))
    }

    fn pipelines(kit: PipelineKit, ubo_set: &DescriptorSet, range: GsPushConstantRange, depth_image: &GsDSAttachment) -> GsResult<GsPipeline<Graphics>> {

        // shaders
        let vertex_shader = GsShaderInfo::from_source(GsPipelineStage::VERTEX, Path::new(VERTEX_SHADER_SOURCE_PATH), None, "[Vertex Shader]");
        let fragment_shader = GsShaderInfo::from_source(GsPipelineStage::FRAGMENT, Path::new(FRAGMENT_SHADER_SOURCE_PATH), None, "[Fragment Shader]");
        let shader_infos = vec![vertex_shader, fragment_shader];
        let vertex_input_desc = Vertex::input_description();

        // pipeline
        let mut render_pass_builder = kit.pass_builder();
        let first_subpass = render_pass_builder.new_subpass();

        let color_attachment = kit.present_attachment()
            .op(vk::AttachmentLoadOp::CLEAR, vk::AttachmentStoreOp::STORE)
            .clear_value(DEFAULT_CLEAR_COLOR.clone());
        let depth_attachment = depth_image.attachment()
            .op(vk::AttachmentLoadOp::CLEAR, vk::AttachmentStoreOp::DONT_CARE);

        render_pass_builder.add_attachment(color_attachment, first_subpass);
        render_pass_builder.add_attachment(depth_attachment, first_subpass);

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
        let depth_stencil = GsDepthStencilState::setup(GsDepthStencilPrefab::EnableDepth);
        let mut rasterization = GsRasterizerState::setup(RasterizerPrefab::Common);
        rasterization.set_front_face(vk::FrontFace::COUNTER_CLOCKWISE);

        let pipeline_config = kit.pipeline_config(shader_infos, vertex_input_desc, render_pass)
            .with_depth_stencil(depth_stencil)
            .with_viewport(ViewportStateType::Dynamic { count: 1 })
            .with_rasterizer(rasterization)
            .with_push_constants(vec![range])
            .with_descriptor_sets(&[ubo_set])
            .finish();

        let mut pipeline_builder = kit.gfx_builder()?;
        let graphics_pipeline = pipeline_builder.build(pipeline_config)?;

        Ok(graphics_pipeline)
    }

    fn sync_resources(kit: SyncKit, graphics_pipeline: &GsPipeline<Graphics>) -> GsResult<Vec<GsSemaphore>> {

        // sync
        kit.multi_semaphores(graphics_pipeline.frame_count())
    }

    fn commands(kit: CommandKit, graphics_pipeline: &GsPipeline<Graphics>, model_entity: &GsglTFEntity, ubo_set: &DescriptorSet, view_port: &CmdViewportInfo, scissor: &CmdScissorInfo) -> GsResult<(GsCommandPool, Vec<GsCommandBuffer>)> {

        let command_pool = kit.pool(DeviceQueueIdentifier::Graphics)?;
        let mut command_buffers = vec![];

        let command_buffer_count = graphics_pipeline.frame_count();
        let raw_commands = command_pool.allocate(CmdBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command) in raw_commands.into_iter().enumerate() {
            let mut recorder = kit.pipeline_recorder(graphics_pipeline, command);

            recorder.begin_record(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE)?
                .begin_render_pass(graphics_pipeline, frame_index)
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
            is_use_node_transform: true,
            is_push_materials    : false,
            material_stage: GsPipelineStage::VERTEX,
        };

        // Update light positions
        const R: f32 = 10.5;
        const Y1: f32 = -2.0;
        const Y2: f32 = 15.0;

        let sin_t = (TIMER * 360.0).to_radians().sin();
        let cos_t = (TIMER * 360.0).to_radians().cos();

        let push_data = PushConstants {
            // w component = light radius scale.
            lights: [
                [R * 1.1 * sin_t, Y1, R * 1.1 * cos_t, 2.0],
                [-R * sin_t, Y1, -R * cos_t, 2.0],
                [R * 0.85 * sin_t, Y1, -sin_t * 2.5, 3.0],
                [0.0, Y2, R * 1.25 * cos_t, 3.0],
                [R * 2.25 * cos_t, Y2, 0.0, 2.5],
                [R * 2.5 * cos_t, Y2, R * 2.5 * sin_t, 2.5],
            ],
        };
        let raw_data = bincode::serialize(&push_data)
            .map_err(GsError::serialize)?;

        recorder.push_constants(GsPipelineStage::VERTEX, 0, &raw_data);

        // draw the model.
        model.record_command(recorder, ubo_set, &[], Some(model_render_params))?;

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

        device.submit_single(&submit_info, Some(device_available), DeviceQueueIdentifier::Graphics)?;

        return Ok(&self.present_availables[image_index])
    }

    fn reload_res(&mut self, loader: AssetsLoader) -> GsResult<()> {

        self.pipeline = loader.pipelines(|kit| {
            VulkanExample::pipelines(kit, &self.ubo_set, self.push_range.clone(), &self.depth_attachment)
        })?;

        self.present_availables = loader.syncs(|kit| {
            VulkanExample::sync_resources(kit, &self.pipeline)
        })?;

        let (command_pool, command_buffers) = loader.commands(|kit| {
            VulkanExample::commands(kit, &self.pipeline, &self.model_entity, &self.ubo_set, &self.view_port, &self.scissor)
        })?;
        self.command_pool = command_pool;
        self.command_buffers = command_buffers;

        Ok(())
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
