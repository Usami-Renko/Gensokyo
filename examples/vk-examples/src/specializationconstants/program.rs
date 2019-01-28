
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
use super::data::{ Vertex, UBOVS, UniformResource, PipelineResource, ModelResource, SpecializationData };

use nalgebra::{ Matrix4, Point3, Vector4 };
use std::path::Path;
use std::mem;
use std::ffi::c_void;

const VERTEX_SHADER_SOURCE_PATH  : &'static str = "src/specializationconstants/uber.vert";
const FRAGMENT_SHADER_SOURCE_PATH: &'static str = "src/specializationconstants/uber.frag";
const MODEL_PATH  : &'static str = "models/cube.gltf";
const TEXTURE_PATH: &'static str = "textures/metalplate_nomips_rgba.png";

pub struct VulkanExample {

    model: ModelResource,

    ubo: UniformResource,

    pipelines: PipelineResource,

    command_pool   : GsCommandPool,
    command_buffers: Vec<GsCommandBuffer>,

    scissor  : CmdScissorInfo,

    camera: GsFlightCamera,
    present_availables: Vec<GsSemaphore>,

    is_toggle_event: bool,
}

impl VulkanExample {

    pub fn new(loader: AssetsLoader) -> GsResult<VulkanExample> {

        let screen_dimension = loader.screen_dimension();

        let mut camera = GsCameraFactory::config()
            .place_at(Point3::new(0.0, 0.0, 20.0))
            .screen_aspect_ratio(screen_dimension.width as f32 / 3.0 / screen_dimension.height as f32)
            .into_flight_camera();
        camera.set_move_speed(25.0);

        let scissor = CmdScissorInfo::from(screen_dimension);

        let ubo_data = vec![
            UBOVS {
                projection: camera.proj_matrix(),
                view      : camera.view_matrix(),
                model     : Matrix4::identity(),
                y_correction: Y_CORRECTION.clone(),
                light_pos : Vector4::new(0.0, -2.0, 1.0, 0.0),
            },
        ];

        let (model_entity, model_repository, ubo_buffer, ubo_storage) = loader.assets(|kit| {
            VulkanExample::load_model(kit, &ubo_data)
        })?;

        let (sample_image, depth_attachment, image_storage) = loader.assets(|kit| {
            VulkanExample::image(kit, screen_dimension)
        })?;

        let (ubo_set, desc_storage) = loader.assets(|kit| {
            VulkanExample::ubo(kit, &model_entity, &sample_image, &ubo_buffer)
        })?;

        let pipelines = loader.pipelines(|kit| {
            VulkanExample::pipelines(kit, &model_entity, &ubo_set, &depth_attachment, screen_dimension)
        })?;

        let present_availables = loader.syncs(|kit| {
            VulkanExample::sync_resources(kit, &pipelines.pipeline_set)
        })?;

        let (command_pool, command_buffers) = loader.commands(|kit| {
            VulkanExample::commands(kit, &pipelines, &model_entity, &ubo_set, &scissor)
        })?;

        let model = ModelResource {
            model: model_entity,
            repository: model_repository,
        };

        let ubo = UniformResource {
            texture: sample_image,
            ubo_vs : ubo_data,
            ubo_set, ubo_buffer, ubo_storage,
            depth_attachment, image_storage,
            desc_storage,
        };

        let procedure = VulkanExample {
            model, ubo,
            pipelines,
            command_pool, command_buffers,
            camera, scissor,
            present_availables,
            is_toggle_event: false,
        };

        Ok(procedure)
    }

    fn update_uniforms(&mut self) -> GsResult<()> {

        // Update UBOMatrices uniform block.
        self.ubo.ubo_vs[0].view = self.camera.view_matrix();

        // Update data in memory.
        self.ubo.ubo_storage.data_updater()?
            .update(&self.ubo.ubo_buffer, &self.ubo.ubo_vs)?
            .finish()?;

        Ok(())
    }

    fn load_model(kit: AllocatorKit, ubo_data: &Vec<UBOVS>) -> GsResult<(GsglTFEntity, GsBufferRepository<Device>, GsUniformBuffer, GsBufferRepository<Host>)> {

        let mut model_allocator = kit.buffer(BufferStorageType::DEVICE);
        let mut ubo_allocator = kit.buffer(BufferStorageType::HOST);

        // allocate uniform data buffer.
        // refer to `layout (set = 0, binding = 0) uniform UBO` in uber.vert.
        let ubo_matrix_info = GsBufUniformInfo::new(0, 1, data_size!(UBOVS));
        let ubo_matrix_index1 = ubo_allocator.assign(ubo_matrix_info)?;

        // allocate model data buffer.
        let gltf_importer = kit.gltf_loader();
        let (mut model_entity, model_data) = gltf_importer.load(Path::new(MODEL_PATH))?;

        let model_vertex_index = model_allocator.assign_v2(&model_data.vertex_allot_delegate())?;
        // refer to `layout (set = 0, binding = 1) uniform DynNode` in uber.vert.
        let model_uniform_index = ubo_allocator.assign_v2(&model_data.uniform_allot_delegate(1))?;

        let model_distributor = model_allocator.allocate()?;
        let ubo_distributor = ubo_allocator.allocate()?;

        model_entity.acquire_vertex(model_vertex_index, &model_distributor);
        model_entity.acquire_uniform(model_uniform_index, &ubo_distributor);
        
        let mut model_repository = model_distributor.into_repository();
        model_repository.data_uploader()?
            .upload_v2(&model_entity.vertex_upload_delegate().unwrap(), &model_data)?
            .finish()?;

        let ubo_buffer = ubo_distributor.acquire(ubo_matrix_index1);

        let mut ubo_repository = ubo_distributor.into_repository();
        ubo_repository.data_uploader()?
            .upload_v2(&model_entity.uniform_upload_delegate().unwrap(), &model_data)?
            .upload(&ubo_buffer, ubo_data)?
            .finish()?;

        Ok((model_entity, model_repository, ubo_buffer, ubo_repository))
    }

    fn ubo(kit: AllocatorKit, model: &GsglTFEntity, texture: &GsSampleImage, ubo_buffer: &GsUniformBuffer) -> GsResult<(DescriptorSet, GsDescriptorRepository)> {

        let mut descriptor_allocator = kit.descriptor(vk::DescriptorPoolCreateFlags::empty());

        let mut descriptor_set_config = DescriptorSetConfig::init();
        descriptor_set_config.add_buffer_binding(ubo_buffer, GsPipelineStage::VERTEX); // binding 0
        descriptor_set_config.add_buffer_binding(model, GsPipelineStage::VERTEX); // binding 1
        descriptor_set_config.add_image_binding(texture, GsPipelineStage::FRAGMENT); // binding 2
        let desc_index = descriptor_allocator.assign(descriptor_set_config);

        // allocate descriptor set.
        let descriptor_distributor = descriptor_allocator.allocate()?;
        let ubo_set = descriptor_distributor.acquire(desc_index);

        let desc_storage = descriptor_distributor.into_repository();

        Ok((ubo_set, desc_storage))
    }

    fn image(kit: AllocatorKit, dimension: vkDim2D) -> GsResult<(GsSampleImage, GsDSAttachment, GsImageRepository<Device>)> {

        let mut image_allocator = kit.image(ImageStorageType::DEVICE);

        // Depth Attachment
        let depth_attachment_info = GsDSAttachmentInfo::new(dimension, DepthStencilImageFormat::Depth32Bit);
        let depth_image_index = image_allocator.assign(depth_attachment_info)?;

        // Combined Sample Image
        let image_loader = kit.image_loader();
        let image_storage = image_loader.load_2d(Path::new(TEXTURE_PATH))?; // texture.
        // refer to `layout (set = 0, binding = 2) sampler2D samplerColorMap` in uber.frag. Accessible from the fragment shader only.
        let image_info = GsSampleImgInfo::new(2, 1, image_storage, ImagePipelineStage::FragmentStage);
        let sample_image_index = image_allocator.assign(image_info)?;

        let image_distributor = image_allocator.allocate()?;

        let depth_attachment = image_distributor.acquire(depth_image_index);
        let sample_image = image_distributor.acquire(sample_image_index);

        let image_storage = image_distributor.into_repository();

        Ok((sample_image, depth_attachment, image_storage))
    }

    fn pipelines(kit: PipelineKit, model: &GsglTFEntity, ubo_set: &DescriptorSet, depth_image: &GsDSAttachment, dimension: vkDim2D) -> GsResult<PipelineResource> {

        // render pass
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

        // set pipeline states.
        let pipeline_template = kit.pipeline_config(vec![], Vertex::input_description(), render_pass)
            .with_depth_stencil(depth_stencil)
            .with_viewport(ViewportStateType::Dynamic { count: 1 })
            .with_descriptor_sets(&[ubo_set])
            .with_push_constants(vec![model.pushconst_description(GsPipelineStage::VERTEX)])
            .finish();

        // generate pipelines.
        let width_stride = (dimension.width as f32 / 3.0) as vkuint;
        let mut pipeline_builder = kit.gfx_set_builder(pipeline_template)?;
        pipeline_builder.set_base_pipeline_use(false);

        // Solid phong shading ---------------------------------------------------------------------------
        let (phong_pipeline, phong_viewport) = {

            let specialization_data = SpecializationData {
                light_model: 0,
                toon_desaturation_factor: 0.5,
            };

            let map_entries = SpecializationData::specialization_map_entries();
            let specialization_info = vk::SpecializationInfo {
                map_entry_count: map_entries.len() as _,
                p_map_entries  : map_entries.as_ptr(),
                data_size      : mem::size_of::<SpecializationData>(),
                p_data         : &specialization_data as *const SpecializationData as *const c_void,
            };

            let vertex_shader = GsShaderInfo::from_source(GsPipelineStage::VERTEX, Path::new(VERTEX_SHADER_SOURCE_PATH), None, "[Vertex Shader]");
            let mut fragment_shader = GsShaderInfo::from_source(GsPipelineStage::FRAGMENT, Path::new(FRAGMENT_SHADER_SOURCE_PATH), None, "[Fragment Shader]");
            fragment_shader.set_specialization(specialization_info);
            let shader_infos = vec![vertex_shader, fragment_shader];

            let pipeline_template = pipeline_builder.template_mut();
            pipeline_template.reset_shader(shader_infos);
            let pipeline = pipeline_builder.build_template()?;
            // Left: Solid colored.
            let viewport = CmdViewportInfo::new(0, 0, width_stride, dimension.height);
            (pipeline, viewport)
        };
        // -----------------------------------------------------------------------------------------------

        // Phong and textured ----------------------------------------------------------------------------
        let (toon_pipeline, toon_viewport) = {

            let specialization_data = SpecializationData {
                light_model: 1,
                toon_desaturation_factor: 0.5,
            };
            let map_entries = SpecializationData::specialization_map_entries();
            let specialization_info = vk::SpecializationInfo {
                map_entry_count: map_entries.len() as _,
                p_map_entries  : map_entries.as_ptr(),
                data_size      : mem::size_of::<SpecializationData>(),
                p_data         : &specialization_data as *const SpecializationData as *const c_void,
            };

            let vertex_shader = GsShaderInfo::from_source(GsPipelineStage::VERTEX, Path::new(VERTEX_SHADER_SOURCE_PATH), None, "[Vertex Shader]");
            let mut fragment_shader = GsShaderInfo::from_source(GsPipelineStage::FRAGMENT, Path::new(FRAGMENT_SHADER_SOURCE_PATH), None, "[Fragment Shader]");
            fragment_shader.set_specialization(specialization_info);
            let shader_infos = vec![vertex_shader, fragment_shader];

            let pipeline_template = pipeline_builder.template_mut();
            pipeline_template.reset_shader(shader_infos);
            let pipeline = pipeline_builder.build_template()?;
            // Center: Toon
            let viewport = CmdViewportInfo::new(width_stride, 0, width_stride, dimension.height);
            (pipeline, viewport)
        };
        // ----------------------------------------------------------------------------------------------

        // Textured discard -----------------------------------------------------------------------------
        let (textured_pipeline, textured_viewport) = {

            let specialization_data = SpecializationData {
                light_model: 2,
                toon_desaturation_factor: 0.5,
            };
            let map_entries = SpecializationData::specialization_map_entries();
            let specialization_info = vk::SpecializationInfo {
                map_entry_count: map_entries.len() as _,
                p_map_entries  : map_entries.as_ptr(),
                data_size      : mem::size_of::<SpecializationData>(),
                p_data         : &specialization_data as *const SpecializationData as *const c_void,
            };

            let vertex_shader = GsShaderInfo::from_source(GsPipelineStage::VERTEX, Path::new(VERTEX_SHADER_SOURCE_PATH), None, "[Vertex Shader]");
            let mut fragment_shader = GsShaderInfo::from_source(GsPipelineStage::FRAGMENT, Path::new(FRAGMENT_SHADER_SOURCE_PATH), None, "[Fragment Shader]");
            fragment_shader.set_specialization(specialization_info);
            let shader_infos = vec![vertex_shader, fragment_shader];

            let pipeline_template = pipeline_builder.template_mut();
            pipeline_template.reset_shader(shader_infos);
            let pipeline = pipeline_builder.build_template()?;
            // Right: Textured
            let viewport = CmdViewportInfo::new(width_stride * 2, 0, width_stride, dimension.height);
            (pipeline, viewport)
        };
        // ---------------------------------------------------------------------------------------------

        let pipeline_set = pipeline_builder.collect_into_set();

        let pipelines = PipelineResource {
            pipeline_set,
            phong: phong_pipeline, phong_viewport,
            toon : toon_pipeline, toon_viewport,
            textured: textured_pipeline, textured_viewport,
        };

        Ok(pipelines)
    }

    fn sync_resources(kit: SyncKit, pipelines: &GsPipelineSet<Graphics>) -> GsResult<Vec<GsSemaphore>> {

        // sync
        kit.multi_semaphores(pipelines.frame_count())
    }

    fn commands(kit: CommandKit, pipelines: &PipelineResource, model: &GsglTFEntity, ubo_set: &DescriptorSet, scissor: &CmdScissorInfo) -> GsResult<(GsCommandPool, Vec<GsCommandBuffer>)> {

        let command_pool = kit.pool(DeviceQueueIdentifier::Graphics)?;
        let mut command_buffers = vec![];

        let command_buffer_count = pipelines.pipeline_set.frame_count();
        let raw_commands = command_pool.allocate(CmdBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command) in raw_commands.into_iter().enumerate() {

            let model_render_params = GsglTFRenderParams {
                is_use_vertex        : true,
                is_use_node_transform: true,
                is_push_materials    : true,
                material_stage: GsPipelineStage::VERTEX,
            };

            let phong_pipeline = pipelines.pipeline_set.element(&pipelines.phong);
            let mut recorder = kit.pipeline_recorder(&phong_pipeline, command);

            recorder.begin_record(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE)?
                .begin_render_pass(&phong_pipeline, frame_index);

            { // Draw with Phong Pipeline.
                recorder
                    .bind_pipeline()
                    .set_viewport(0, &[pipelines.phong_viewport.clone()])
                    .set_scissor(0, &[scissor.clone()]);

                model.record_command(&recorder, ubo_set, &[], Some(model_render_params.clone()))?;
            }

            { // Draw with Toon Pipeline.
                let toon_pipeline = pipelines.pipeline_set.element(&pipelines.toon);
                recorder.switch_pipeline(&toon_pipeline);
                recorder
                    .bind_pipeline()
                    .set_viewport(0, &[pipelines.toon_viewport.clone()]);

                model.record_command(&recorder, ubo_set, &[], Some(model_render_params.clone()))?;
            }

            { // Draw with Textured Pipeline.
                let textured_pipeline = pipelines.pipeline_set.element(&pipelines.textured);
                recorder.switch_pipeline(&textured_pipeline);
                recorder
                    .bind_pipeline()
                    .set_viewport(0, &[pipelines.textured_viewport.clone()]);

                model.record_command(&recorder, ubo_set, &[], Some(model_render_params.clone()))?;
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

    fn reload_res(&mut self, loader: AssetsLoader) -> GsResult<()> {

        self.pipelines = loader.pipelines(|kit| {
            VulkanExample::pipelines(kit, &self.model.model, &self.ubo.ubo_set, &self.ubo.depth_attachment, loader.screen_dimension())
        })?;

        self.present_availables = loader.syncs(|kit| {
            VulkanExample::sync_resources(kit, &self.pipelines.pipeline_set)
        })?;

        let (command_pool, command_buffers) = loader.commands(|kit| {
            VulkanExample::commands(kit, &self.pipelines, &self.model.model, &self.ubo.ubo_set, &self.scissor)
        })?;
        self.command_pool = command_pool;
        self.command_buffers = command_buffers;

        Ok(())
    }

    fn clean_routine(&mut self, device: &GsDevice) {

        // Remember to destroy sample image manually.
        self.ubo.texture.destroy(device);
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
