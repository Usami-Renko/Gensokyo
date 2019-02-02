
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
use super::data::{ Vertex, UboView, UboDataDynamic, RotationData };
use super::data::{ VERTEX_DATA, INDEX_DATA };

use nalgebra::{ Matrix4, Point3 };
use std::path::Path;
type Matrix4F = Matrix4<f32>;

const VERTEX_SHADER_SOURCE_PATH  : &'static str = "src/dynamicuniformbuffer/base.vert.glsl";
const FRAGMENT_SHADER_SOURCE_PATH: &'static str = "src/dynamicuniformbuffer/base.frag.glsl";
pub const OBJECT_INSTANCE: usize = 125;

pub struct VulkanExample {

    vertex_buffer : GsVertexBuffer,
    index_buffer  : GsIndexBuffer,
    #[allow(dead_code)]
    vertex_storage: GsBufferRepository<Device>,

    rotations: RotationData,
    ubo_data_dynamic: UboDataDynamic,
    ubo_instance_buffer: GsUniformBuffer,
    dynamic_alignment: vkbytes,

    ubo_view_data  : Vec<UboView>,
    ubo_view_buffer: GsUniformBuffer,
    ubo_storage: GsBufferRepository<Host>,

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
            .place_at(Point3::new(0.0, 0.0, 34.0))
            .screen_aspect_ratio(screen_dimension.width as f32 / screen_dimension.height as f32)
            .into_flight_camera();
        camera.set_move_speed(50.0);

        let ubo_view_data = vec![
            UboView {
                projection: camera.proj_matrix(),
                view      : camera.view_matrix(),
                y_correction: Y_CORRECTION.clone(),
            },
        ];

        let rotations = RotationData::new_by_rng();
        let ubo_data_dynamic = UboDataDynamic::identity();

        let (vertex_buffer, index_buffer, vertex_storage, ubo_view_buffer, ubo_instance_buffer, ubo_storage) = {
            VulkanExample::buffers(&initializer, &ubo_data_dynamic)
        }?;

        let dynamic_alignment = ubo_instance_buffer.aligned_size();

        println!("minUniformBufferOffsetAlignment: {}", ubo_instance_buffer.require_dynamic_alignment());
        println!("dynamicAlignment: {}", dynamic_alignment);

        let (depth_attachment, image_storage) = {
            VulkanExample::image(&initializer, screen_dimension)
        }?;

        let (ubo_set, desc_storage) = {
            VulkanExample::ubo(&initializer, &ubo_view_buffer, &ubo_instance_buffer)
        }?;

        let pipeline = {
            VulkanExample::pipelines(&initializer, &ubo_set, &depth_attachment)
        }?;

        let present_availables = {
            VulkanExample::sync_resources(&initializer, &pipeline)
        }?;

        let view_port = CmdViewportInfo::from(screen_dimension);
        let scissor = CmdScissorInfo::from(screen_dimension);

        let (command_pool, command_buffers) = {
            VulkanExample::commands(&initializer, &pipeline, &vertex_buffer, &index_buffer, &ubo_set, dynamic_alignment as _, &view_port, &scissor)
        }?;

        let procedure = VulkanExample {
            ubo_view_data,
            vertex_storage, vertex_buffer, index_buffer,
            ubo_data_dynamic, ubo_instance_buffer, rotations, dynamic_alignment,
            ubo_view_buffer, ubo_storage,
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

    fn update_uniforms(&mut self, delta_time: f32) -> GsResult<()> {

        const FPS_60: f32 = 1.0 / 60.0;

        // if self.is_toggle_event && delta_time > 1.0 / 60.0 { // update at max. 60 fps.
        // if self.is_toggle_event {
        if delta_time > FPS_60 {

            // update camera.
            self.ubo_view_data[0].view = self.camera.view_matrix();

            // update models.
            self.ubo_data_dynamic.update(&mut self.rotations, delta_time);

            self.ubo_storage.data_updater()?
                .update(&self.ubo_view_buffer, &self.ubo_view_data)?
                // call update align on dynamic uniform buffer.
                .update_align(&self.ubo_instance_buffer, &self.ubo_data_dynamic.model, self.dynamic_alignment)?
                .finish()?;
        }

        Ok(())
    }

    fn buffers(initializer: &AssetInitializer, ubo_instance_data: &UboDataDynamic) -> GsResult<(GsVertexBuffer, GsIndexBuffer, GsBufferRepository<Device>, GsUniformBuffer, GsUniformBuffer, GsBufferRepository<Host>)> {

        // vertex, index and uniform buffer
        let mut vertex_allocator = GsBufferAllocator::new(initializer, BufferStorageType::DEVICE);
        let mut ubo_allocator = GsBufferAllocator::new(initializer, BufferStorageType::HOST);

        let vertex_info = GsVertexBuffer::new(data_size!(Vertex), VERTEX_DATA.len());
        let vertex_index = vertex_allocator.assign(vertex_info)?;

        let index_info = GsIndexBuffer::new(INDEX_DATA.len());
        let index_index = vertex_allocator.assign(index_info)?;

        // refer to `layout (set = 0, binding = 0) uniform UboView` in base.vert.gls,.
        let ubo_view_info = GsUniformBuffer::new(0, data_size!(UboView));
        let ubo_view_index = ubo_allocator.assign(ubo_view_info)?;

        // refer to `layout (set = 0, binding = 1) uniform UboInstance` in base.vert.glsl.
        let ubo_instance_info = GsUniformBuffer::new_dyn(1, data_size!(Matrix4F), OBJECT_INSTANCE);
        let ubo_instance_index = ubo_allocator.assign(ubo_instance_info)?;

        let vertex_distributor = vertex_allocator.allocate()?;
        let ubo_distributor = ubo_allocator.allocate()?;

        let vertex_buffer = vertex_distributor.acquire(vertex_index);
        let index_buffer = vertex_distributor.acquire(index_index);
        let ubo_view_buffer = ubo_distributor.acquire(ubo_view_index);
        let ubo_instance_buffer = ubo_distributor.acquire(ubo_instance_index);

        let mut vertex_storage = vertex_distributor.into_repository();
        let mut ubo_storage = ubo_distributor.into_repository();

        vertex_storage.data_uploader()?
            .upload(&vertex_buffer, VERTEX_DATA.as_ref())?
            .upload(&index_buffer, INDEX_DATA.as_ref())?
            .finish()?;
        ubo_storage.data_uploader()?
            // call upload align on dynamic uniform buffer.
            .upload_align(&ubo_instance_buffer, &ubo_instance_data.model, ubo_instance_buffer.aligned_size())?
            .finish()?;

        Ok((vertex_buffer, index_buffer, vertex_storage, ubo_view_buffer, ubo_instance_buffer, ubo_storage))
    }

    fn ubo(initializer: &AssetInitializer, ubo_view_buffer: &GsUniformBuffer, ubo_instance_buffer: &GsUniformBuffer) -> GsResult<(DescriptorSet, GsDescriptorRepository)> {

        // descriptor
        let mut descriptor_set_config = DescriptorSetConfig::new();
        descriptor_set_config.add_buffer_binding(ubo_view_buffer, GsPipelineStage::VERTEX);
        descriptor_set_config.add_buffer_binding(ubo_instance_buffer, GsPipelineStage::VERTEX);

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

    fn pipelines(initializer: &AssetInitializer, ubo_set: &DescriptorSet, depth_image: &GsDSAttachment) -> GsResult<GsPipeline<Graphics>> {

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

    fn commands(initializer: &AssetInitializer, pipeline: &GsPipeline<Graphics>, vertex_buffer: &GsVertexBuffer, index_buffer: &GsIndexBuffer, ubo_set: &DescriptorSet, dynamic_alignment: vkuint, view_port: &CmdViewportInfo, scissor: &CmdScissorInfo) -> GsResult<(GsCommandPool, Vec<GsCommandBuffer>)> {

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
                .bind_pipeline()
                .bind_vertex_buffers(0, &[vertex_buffer])
                .bind_index_buffer(index_buffer, 0);

            // Render multiple objects using different model matrices by dynamically offsetting into one uniform buffer.
            for i in 0..(OBJECT_INSTANCE as vkuint) {
                // One dynamic offset per dynamic descriptor to offset into the ubo containing all model matrices.
                let dynamic_offset = i * dynamic_alignment;
                recorder
                    .bind_descriptor_sets_dynamic(0, &[ubo_set], &[dynamic_offset])
                    .draw_indexed(index_buffer.total_count(), 1, 0, 0, 0);
            }

            recorder.end_render_pass();

            let command_recorded = recorder.end_record()?;
            command_buffers.push(command_recorded);
        }

        Ok((command_pool, command_buffers))
    }
}

impl GraphicsRoutine for VulkanExample {

    fn draw(&mut self, device: &GsDevice, device_available: &GsFence, image_available: &GsSemaphore, image_index: usize, delta_time: f32) -> GsResult<&GsSemaphore> {

        self.update_uniforms(delta_time)?;

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

        self.pipeline = VulkanExample::pipelines(&initializer, &self.ubo_set, &self.depth_attachment)?;

        self.present_availables = VulkanExample::sync_resources(&initializer, &self.pipeline)?;

        let (command_pool, command_buffers) = VulkanExample::commands(&initializer, &self.pipeline, &self.vertex_buffer, &self.index_buffer, &self.ubo_set,  self.dynamic_alignment as _, &self.view_port, &self.scissor)?;
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
