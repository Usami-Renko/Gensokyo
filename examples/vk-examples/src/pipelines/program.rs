
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

use vk_examples::Y_CORRECTION;
use super::data::{ Vertex, UBOVS, PipelineContent };

use nalgebra::{ Matrix4, Point3, Vector4 };
use std::path::Path;

const PHONG_VERTEX_SHADER_SOURCE_PATH      : &'static str = "src/pipelines/phong.vert";
const PHONG_FRAGMENT_SHADER_SOURCE_PATH    : &'static str = "src/pipelines/phong.frag";
const TOON_VERTEX_SHADER_SOURCE_PATH       : &'static str = "src/pipelines/toon.vert";
const TOON_FRAGMENT_SHADER_SOURCE_PATH     : &'static str = "src/pipelines/toon.frag";
const WIREFRAME_VERTEX_SHADER_SOURCE_PATH  : &'static str = "src/pipelines/wireframe.vert";
const WIREFRAME_FRAGMENT_SHADER_SOURCE_PATH: &'static str = "src/pipelines/wireframe.frag";
const MODEL_PATH: &'static str = "models/treasure_smooth.gltf";

pub struct VulkanExample {

    model_entity: GsglTFEntity,
    #[allow(dead_code)]
    model_storage: GsBufferRepository<Device>,

    ubo_data   : Vec<UBOVS>,
    ubo_buffer : GsUniformBuffer,
    ubo_storage: GsBufferRepository<Host>,

    phone_pipeline: PipelineContent,
    toon_pipeline: PipelineContent,
    wireframe_pipeline: PipelineContent,

    ubo_set     : DescriptorSet,
    #[allow(dead_code)]
    desc_storage: GsDescriptorRepository,

    depth_attachment: GsDSAttachment,
    #[allow(dead_code)]
    image_storage   : GsImageRepository<Device>,

    command_pool   : GsCommandPool,
    command_buffers: Vec<GsCommandBuffer>,

    camera: GsFlightCamera,
    present_availables: Vec<GsSemaphore>,

    is_toggle_event: bool,
}

impl VulkanExample {

    pub fn new(loader: AssetsLoader) -> GsResult<VulkanExample> {

        let screen_dimension = loader.screen_dimension();

        let mut camera = GsCameraFactory::config()
            .place_at(Point3::new(0.0, 0.0, 2.5))
            .screen_aspect_ratio(screen_dimension.width as f32 / screen_dimension.height as f32)
            .into_flight_camera();
        camera.set_move_speed(50.0);

        let ubo_data = vec![
            UBOVS {
                projection: camera.proj_matrix(),
                view      : camera.view_matrix(),
                model     : Matrix4::identity(),
                y_correction: Y_CORRECTION.clone(),
                light_pos : Vector4::new(0.0, 2.0, 1.0, 0.0),
            },
        ];

        let (model_entity, model_storage, ubo_buffer, ubo_storage) = loader.assets(|kit| {
            VulkanExample::load_model(kit, &ubo_data)
        })?;

        let (ubo_set, desc_storage) = loader.assets(|kit| {
            VulkanExample::ubo(kit, &model_entity, &ubo_buffer)
        })?;

        let (depth_attachment, image_storage) = loader.assets(|kit| {
            VulkanExample::image(kit, screen_dimension)
        })?;

        let (phone_pipeline, toon_pipeline, wireframe_pipeline) = loader.pipelines(|kit| {
            VulkanExample::pipelines(kit, &model_entity, &ubo_set, &depth_attachment, screen_dimension)
        })?;

        let present_availables = loader.syncs(|kit| {
            VulkanExample::sync_resources(kit, &phone_pipeline.pipeline)
        })?;

        let (command_pool, command_buffers) = loader.commands(|kit| {
            VulkanExample::commands(kit, &phone_pipeline, &toon_pipeline, &wireframe_pipeline, &model_entity, &ubo_set)
        })?;

        let procedure = VulkanExample {
            ubo_data,
            model_entity, model_storage,
            ubo_buffer, ubo_storage,
            desc_storage, ubo_set,
            phone_pipeline, toon_pipeline, wireframe_pipeline,
            depth_attachment, image_storage,
            command_pool, command_buffers,
            camera,
            present_availables,
            is_toggle_event: false,
        };

        Ok(procedure)
    }

    fn update_uniforms(&mut self) -> GsResult<()> {

        self.ubo_data[0].view = self.camera.view_matrix();

        self.ubo_storage.data_updater()?
            .update(&self.ubo_buffer, &self.ubo_data)?
            .finish()?;

        Ok(())
    }
    fn load_model(kit: AllocatorKit, ubo_data: &Vec<UBOVS>) -> GsResult<(GsglTFEntity, GsBufferRepository<Device>, GsUniformBuffer, GsBufferRepository<Host>)> {

        let mut model_allocator = kit.buffer(BufferStorageType::DEVICE);
        let mut ubo_allocator = kit.buffer(BufferStorageType::HOST);

        // allocate uniform data buffer.
        // refer to `layout (binding = 0) uniform UBO` in phong.vert, toon.vert or wireframe.vert.
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

    fn pipelines(kit: PipelineKit, model_entity: &GsglTFEntity, ubo_set: &DescriptorSet, depth_image: &GsDSAttachment, dimension: vkDim2D) -> GsResult<(PipelineContent, PipelineContent, PipelineContent)> {

        // shaders ------------------------------------------------------------------------
        let vertex_input_desc = Vertex::input_description();

        // shaders of Phone Pipeline.
        let phong_vertex_shader = GsShaderInfo::from_source(GsPipelineStage::VERTEX, Path::new(PHONG_VERTEX_SHADER_SOURCE_PATH), None, "[Phone Vertex Shader]");
        let phone_fragment_shader = GsShaderInfo::from_source(GsPipelineStage::FRAGMENT, Path::new(PHONG_FRAGMENT_SHADER_SOURCE_PATH), None, "[Phone Fragment Shader]");
        let phong_shader_infos = vec![phong_vertex_shader, phone_fragment_shader];

        // shaders of Toon Pipeline.
        let tone_vertex_shader = GsShaderInfo::from_source(GsPipelineStage::VERTEX, Path::new(TOON_VERTEX_SHADER_SOURCE_PATH), None, "[Toon Vertex Shader]");
        let tone_fragment_shader = GsShaderInfo::from_source(GsPipelineStage::FRAGMENT, Path::new(TOON_FRAGMENT_SHADER_SOURCE_PATH), None, "[Toon Fragment Shader]");
        let tone_shader_infos = vec![tone_vertex_shader, tone_fragment_shader];

        // shaders of Wireframe Pipeline.
        let wireframe_vertex_shader = GsShaderInfo::from_source(GsPipelineStage::VERTEX, Path::new(WIREFRAME_VERTEX_SHADER_SOURCE_PATH), None, "[Wireframe Vertex Shader]");
        let wireframe_fragment_shader = GsShaderInfo::from_source(GsPipelineStage::FRAGMENT, Path::new(WIREFRAME_FRAGMENT_SHADER_SOURCE_PATH), None, "[Wireframe Fragment Shader]");
        let wireframe_shader_infos = vec![wireframe_vertex_shader, wireframe_fragment_shader];
        // --------------------------------------------------------------------------------

        // create render pass -------------------------------------------------------------
        let render_pass = {
            let mut render_pass_builder = kit.pass_builder();
            let first_subpass = render_pass_builder.new_subpass();

            let color_attachment = kit.present_attachment()
                .op(vk::AttachmentLoadOp::CLEAR, vk::AttachmentStoreOp::STORE)
                .clear_value(vk::ClearValue { color: vk::ClearColorValue { float32: [0.025, 0.025, 0.025, 1.0] } });
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

            render_pass_builder.build()?
        };
        // --------------------------------------------------------------------------------

        // set pipeline states ----------------------------------------------------------------------------
        let depth_stencil = GsDepthStencilState::setup(GsDepthStencilPrefab::EnableDepth);
        let viewport_state = ViewportStateType::Dynamic { count: 1 }; // use dynamic viewport and scissor.
        // TODO: Wide line is not support yet.
        // rasterizer.set_line_width(DynamicableValue::Dynamic); // use dynamic line with.

        let phong_config = kit.pipeline_config(phong_shader_infos, vertex_input_desc, render_pass)
            .with_depth_stencil(depth_stencil)
            .with_viewport(viewport_state)
            .add_descriptor_sets(&[ubo_set])
            .add_push_constants(vec![model_entity.pushconst_description(GsPipelineStage::VERTEX)])
            .finish();
        // --------------------------------------------------------------------------------

        // create phone pipeline. ---------------------------------------------------------------
        let width_stride = (dimension.width as f32 / 3.0) as vkuint;

        let phong_pipeline = {
            let mut pipeline_builder = kit.graphics_pipeline_builder()?;
            pipeline_builder.add_config(&phong_config)?;

            let mut pipelines = pipeline_builder.build(PipelineDeriveState::AsParent)?;

            PipelineContent {
                pipeline: pipelines.pop().unwrap(),
                // Left: Solid colored.
                viewport: CmdViewportInfo::new(0, 0, width_stride, dimension.height),
                scissor : CmdScissorInfo::from(dimension),
            }
        };
        // --------------------------------------------------------------------------------

        // create toon pipeline -----------------------------------------------------------
        let toon_config = phong_config
            .with_shader(tone_shader_infos)
            .finish();

        let toon_pipeline = {
            let mut pipeline_builder = kit.graphics_pipeline_builder()?;
            pipeline_builder.add_config(&toon_config)?;

            let derive_state = PipelineDeriveState::AsChildren { parent: &phong_pipeline.pipeline };
            let mut pipelines = pipeline_builder.build(derive_state)?;

            PipelineContent {
                pipeline: pipelines.pop().unwrap(),
                // Center: Toon
                viewport: CmdViewportInfo::new(width_stride, 0, width_stride, dimension.height),
                scissor : CmdScissorInfo::from(dimension),
            }
        };
        // --------------------------------------------------------------------------------

        // create wireframe pipeline ------------------------------------------------------
        let mut rasterizer = GsRasterizerState::setup(RasterizerPrefab::Common);
        rasterizer.set_polygon_mode(vk::PolygonMode::LINE);
        let wireframe_config = toon_config
            .with_shader(wireframe_shader_infos)
            .with_rasterizer(rasterizer)
            .finish();

        let wireframe_pipeline = {
            let mut pipeline_builder = kit.graphics_pipeline_builder()?;
            pipeline_builder.add_config(&wireframe_config)?;

            let derive_state = PipelineDeriveState::AsChildren { parent: &phong_pipeline.pipeline };
            let mut pipelines = pipeline_builder.build(derive_state)?;

            PipelineContent {
                pipeline: pipelines.pop().unwrap(),
                /// Right: Wireframe
                viewport: CmdViewportInfo::new(width_stride * 2, 0, width_stride, dimension.height),
                scissor : CmdScissorInfo::from(dimension),
            }
        };
        // --------------------------------------------------------------------------------

        Ok((phong_pipeline, toon_pipeline, wireframe_pipeline))
    }

    fn sync_resources(kit: SyncKit, graphics_pipeline: &GsPipeline<Graphics>) -> GsResult<Vec<GsSemaphore>> {

        // sync
        kit.multi_semaphores(graphics_pipeline.frame_count())
    }

    fn commands(kit: CommandKit, phong: &PipelineContent, toon: &PipelineContent, wireframe: &PipelineContent, model_entity: &GsglTFEntity, ubo_set: &DescriptorSet) -> GsResult<(GsCommandPool, Vec<GsCommandBuffer>)> {

        let command_pool = kit.pool(DeviceQueueIdentifier::Graphics)?;
        let mut command_buffers = vec![];

        let command_buffer_count = phong.pipeline.frame_count();
        let raw_commands = command_pool.allocate(CmdBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command) in raw_commands.into_iter().enumerate() {

            let render_params = GsglTFRenderParams {
                is_use_vertex: true,
                is_use_node_transform: true,
                is_push_materials: true,
                material_stage: GsPipelineStage::VERTEX,
            };

            let mut recorder = kit.pipeline_recorder(&phong.pipeline, command);
            recorder.begin_record(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE)?
                // three pipeline shared the same render pass. So it's ok to set once here.
                .begin_render_pass(phong.pipeline.render_pass_ref(), frame_index);

            { // Draw with Phong Pipeline.
                recorder
                    .bind_pipeline()
                    .set_viewport(0, &[phong.viewport.clone()])
                    .set_scissor(0, &[phong.scissor.clone()]);

                model_entity.record_command(&recorder, ubo_set, &[], Some(render_params.clone()))?;
            }

            { // Draw with Toon Pipeline.
                recorder.switch_pipeline(&toon.pipeline);
                recorder
                    .bind_pipeline()
                    .set_viewport(0, &[toon.viewport.clone()]);

                model_entity.record_command(&recorder, ubo_set, &[], Some(render_params.clone()))?;
            }

            { // Draw with Wireframe Pipeline.
                recorder.switch_pipeline(&wireframe.pipeline);
                recorder
                    .bind_pipeline()
                    .set_viewport(0, &[wireframe.viewport.clone()]);

                model_entity.record_command(&recorder, ubo_set, &[], Some(render_params.clone()))?;
            }

            recorder.end_render_pass();

            let command_recorded = recorder.end_record()?;
            command_buffers.push(command_recorded);
        }

        Ok((command_pool, command_buffers))
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

    fn clean_resources(&mut self, _: &GsDevice) -> GsResult<()> {

        self.present_availables.iter()
            .for_each(|semaphore| semaphore.destroy());
        self.present_availables.clear();
        self.command_buffers.clear();
        self.command_pool.destroy();
        self.phone_pipeline.pipeline.destroy();
        self.toon_pipeline.pipeline.destroy();
        self.wireframe_pipeline.pipeline.destroy();

        Ok(())
    }

    fn reload_res(&mut self, loader: AssetsLoader) -> GsResult<()> {

        let (phone, toon, wireframe) = loader.pipelines(|kit| {
            VulkanExample::pipelines(kit, &self.model_entity, &self.ubo_set, &self.depth_attachment, loader.screen_dimension())
        })?;
        self.phone_pipeline = phone;
        self.toon_pipeline = toon;
        self.wireframe_pipeline = wireframe;

        self.present_availables = loader.syncs(|kit| {
            VulkanExample::sync_resources(kit, &self.phone_pipeline.pipeline)
        })?;

        let (command_pool, command_buffers) = loader.commands(|kit| {
            VulkanExample::commands(kit, &self.phone_pipeline, &self.toon_pipeline, &self.wireframe_pipeline, &self.model_entity, &self.ubo_set)
        })?;
        self.command_pool = command_pool;
        self.command_buffers = command_buffers;

        Ok(())
    }

    fn clean_routine(&mut self, _: &GsDevice) {

        self.present_availables.iter()
            .for_each(|semaphore| semaphore.destroy());
        self.phone_pipeline.pipeline.destroy();
        self.toon_pipeline.pipeline.destroy();
        self.wireframe_pipeline.pipeline.destroy();
        self.command_pool.destroy();
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
