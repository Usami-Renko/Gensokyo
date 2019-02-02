
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
use super::data::{ Vertex, UBOVS };
use super::data::{ VERTEX_DATA, INDEX_DATA };

use nalgebra::{ Matrix4, Point3, Point4 };
use std::path::Path;

const VERTEX_SHADER_SOURCE_PATH  : &'static str = "src/texture/texture.vert.glsl";
const FRAGMENT_SHADER_SOURCE_PATH: &'static str = "src/texture/texture.frag.glsl";
const TEXTURE_PATH: &'static str = "textures/metalplate01_rgba.png";

pub struct VulkanExample {

    vertex_buffer : GsVertexBuffer,
    index_buffer  : GsIndexBuffer,
    #[allow(dead_code)]
    vertex_storage: GsBufferRepository<Device>,

    ubo_data   : Vec<UBOVS>,
    ubo_buffer : GsUniformBuffer,
    ubo_storage: GsBufferRepository<Host>,

    pipeline: GsPipeline<Graphics>,

    ubo_set     : DescriptorSet,
    #[allow(dead_code)]
    desc_storage: GsDescriptorRepository,

    #[allow(dead_code)]
    sample_image: GsCombinedImgSampler,
    depth_attachment: GsDSAttachment,
    #[allow(dead_code)]
    image_storage   : GsImageRepository<Device>,

    command_pool   : GsCommandPool,
    command_buffers: Vec<GsCommandBuffer>,

    view_port: CmdViewportInfo,
    scissor  : CmdScissorInfo,

    camera: GsFlightCamera,
    present_availables: Vec<GsSemaphore>,

    lod_bias: f32,
    is_toggle_event: bool,
}

impl VulkanExample {

    pub fn new(initializer: AssetInitializer) -> GsResult<VulkanExample> {

        let screen_dimension = initializer.screen_dimension();

        let camera = GsCameraFactory::config()
            .place_at(Point3::new(0.0, 0.0, 2.5))
            .screen_aspect_ratio(screen_dimension.width as f32 / screen_dimension.height as f32)
            .into_flight_camera();

        let ubo_data = vec![
            UBOVS {
                projection  : camera.proj_matrix(),
                view        : camera.view_matrix(),
                model       : Matrix4::identity(),
                y_correction: Y_CORRECTION.clone(),
                view_pos    : Point4::from([0.0, 0.0, 2.5, 0.0]),
                lod_bias    : 0.0,
            },
        ];

        let view_port = CmdViewportInfo::from(screen_dimension);
        let scissor = CmdScissorInfo::from(screen_dimension);

        let (vertex_buffer, index_buffer, vertex_storage, ubo_buffer, ubo_storage) = {
            VulkanExample::buffers(&initializer, &ubo_data)
        }?;

        let (depth_attachment, sample_image, image_storage) = {
            VulkanExample::image(&initializer, screen_dimension)
        }?;

        let (ubo_set, desc_storage) = {
            VulkanExample::ubo(&initializer, &ubo_buffer, &sample_image)
        }?;

        let pipeline = {
            VulkanExample::pipelines(&initializer, &ubo_set, &depth_attachment)
        }?;

        let present_availables = {
            VulkanExample::sync_resources(&initializer, &pipeline)
        }?;

        let (command_pool, command_buffers) = {
            VulkanExample::commands(&initializer, &pipeline, &vertex_buffer, &index_buffer, &ubo_set, &view_port, &scissor)
        }?;

        let procedure = VulkanExample {
            ubo_data,
            vertex_storage, vertex_buffer, index_buffer,
            ubo_buffer, ubo_storage,
            desc_storage, ubo_set,
            pipeline,
            sample_image, depth_attachment, image_storage,
            command_pool, command_buffers,
            camera, view_port, scissor,
            present_availables,

            lod_bias: 0.0,
            is_toggle_event: false,
        };

        Ok(procedure)
    }

    fn update_uniforms(&mut self) -> GsResult<()> {

        if self.is_toggle_event {

            let camera_pos = self.camera.current_position();

            self.ubo_data[0].view = self.camera.view_matrix();
            self.ubo_data[0].lod_bias = self.lod_bias;
            self.ubo_data[0].view_pos = Point4::new(camera_pos.x, camera_pos.y, camera_pos.z, 0.0);

            self.ubo_storage.data_updater()?
                .update(&self.ubo_buffer, &self.ubo_data)?
                .finish()?;
        }

        Ok(())
    }

    fn buffers(initializer: &AssetInitializer, ubo_data: &[UBOVS]) -> GsResult<(GsVertexBuffer, GsIndexBuffer, GsBufferRepository<Device>, GsUniformBuffer, GsBufferRepository<Host>)> {

        // vertex, index and uniform buffer
        let mut vertex_allocator = GsBufferAllocator::new(initializer, BufferStorageType::DEVICE);
        let mut ubo_allocator = GsBufferAllocator::new(initializer, BufferStorageType::HOST);

        let vertex_info = GsVertexBuffer::new(data_size!(Vertex), VERTEX_DATA.len());
        let vertex_index = vertex_allocator.assign(vertex_info)?;

        let index_info = GsIndexBuffer::new(INDEX_DATA.len());
        let index_index = vertex_allocator.assign(index_info)?;

        // refer to `layout (binding = 0) uniform UBO` in texture.vert.glsl.
        let ubo_info = GsUniformBuffer::new(0, data_size!(UBOVS));
        let ubo_index = ubo_allocator.assign(ubo_info)?;

        let vertex_distributor = vertex_allocator.allocate()?;
        let ubo_distributor = ubo_allocator.allocate()?;

        let vertex_buffer = vertex_distributor.acquire(vertex_index);
        let index_buffer = vertex_distributor.acquire(index_index);
        let ubo_buffer = ubo_distributor.acquire(ubo_index);

        let mut vertex_storage = vertex_distributor.into_repository();
        let mut ubo_storage = ubo_distributor.into_repository();

        vertex_storage.data_uploader()?
            .upload(&vertex_buffer, VERTEX_DATA.as_ref())?
            .upload(&index_buffer, INDEX_DATA.as_ref())?
            .finish()?;

        ubo_storage.data_uploader()?
            .upload(&ubo_buffer, ubo_data)?
            .finish()?;

        Ok((vertex_buffer, index_buffer, vertex_storage, ubo_buffer, ubo_storage))
    }

    fn image(initializer: &AssetInitializer, dimension: vkDim2D) -> GsResult<(GsDSAttachment, GsCombinedImgSampler, GsImageRepository<Device>)> {

        // depth attachment image.
        let mut image_allocator = GsImageAllocator::new(initializer, ImageStorageType::DEVICE);

        let depth_attachment_info = GsDSAttachment::new(dimension, DepthStencilImageFormat::Depth32Bit);
        let depth_image_index = image_allocator.assign(depth_attachment_info)?;

        // combined image sampler.
        let image_storage = ImageLoader::new(initializer).load_2d(Path::new(TEXTURE_PATH))?;

        let sampler = GsSampler::new()
            .filter(vk::Filter::LINEAR, vk::Filter::LINEAR)
            .mipmap(vk::SamplerMipmapMode::LINEAR, vk::SamplerAddressMode::REPEAT, vk::SamplerAddressMode::REPEAT, vk::SamplerAddressMode::REPEAT)
            .anisotropy(Some(16.0))
            .lod(0.0, 0.0, 1.0)
            .compare_op(None)
            .border_color(vk::BorderColor::FLOAT_OPAQUE_WHITE);
        // refer to `layout (binding = 1) uniform sampler2D samplerColor` in texture.frag.glsl.
        let mut sample_image_info = GsCombinedImgSampler::new(1, image_storage, ImagePipelineStage::FragmentStage);
        sample_image_info.reset_sampler(sampler);
        sample_image_info.set_mipmap(MipmapMethod::StepBlit); // tell engine to generate mipmap automatically in runtime.

        let sample_image_index = image_allocator.assign(sample_image_info)?;

        let image_distributor = image_allocator.allocate()?;

        let depth_attachment = image_distributor.acquire(depth_image_index);
        let sample_image = image_distributor.acquire(sample_image_index);

        let image_storage = image_distributor.into_repository();

        Ok((depth_attachment, sample_image, image_storage))
    }

    fn ubo(initializer: &AssetInitializer, ubo_buffer: &GsUniformBuffer, sample_image: &GsCombinedImgSampler) -> GsResult<(DescriptorSet, GsDescriptorRepository)> {

        // descriptor
        let mut descriptor_set_config = DescriptorSetConfig::new();
        descriptor_set_config.add_buffer_binding(ubo_buffer, GsPipelineStage::VERTEX);
        descriptor_set_config.add_image_binding(sample_image, GsPipelineStage::FRAGMENT);

        let mut descriptor_allocator = GsDescriptorAllocator::new(initializer);
        let desc_index = descriptor_allocator.assign(descriptor_set_config);

        let descriptor_distributor = descriptor_allocator.allocate()?;
        let ubo_set = descriptor_distributor.acquire(desc_index);

        let desc_storage = descriptor_distributor.into_repository();

        Ok((ubo_set, desc_storage))
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
            .clear_value(DEFAULT_CLEAR_COLOR);
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

    fn commands(initializer: &AssetInitializer, pipeline: &GsPipeline<Graphics>, vertex_buffer: &GsVertexBuffer, index_buffer: &GsIndexBuffer, ubo_set: &DescriptorSet, view_port: &CmdViewportInfo, scissor: &CmdScissorInfo) -> GsResult<(GsCommandPool, Vec<GsCommandBuffer>)> {

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
                .bind_descriptor_sets(0, &[ubo_set])
                .bind_pipeline()
                .bind_vertex_buffers(0, &[vertex_buffer])
                .bind_index_buffer(index_buffer, 0)
                .draw_indexed(index_buffer.total_count(), 1, 0, 0, 0)
                .end_render_pass();

            let command_recorded = recorder.end_record()?;
            command_buffers.push(command_recorded);
        }

        Ok((command_pool, command_buffers))
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

        self.pipeline = VulkanExample::pipelines(&initializer, &self.ubo_set, &self.depth_attachment)?;

        self.present_availables = VulkanExample::sync_resources(&initializer, &self.pipeline)?;

        let (command_pool, command_buffers) = VulkanExample::commands(&initializer, &self.pipeline, &self.vertex_buffer, &self.index_buffer, &self.ubo_set, &self.view_port, &self.scissor)?;
        self.command_pool = command_pool;
        self.command_buffers = command_buffers;

        Ok(())
    }

    fn react_input(&mut self, inputer: &ActionNerve, delta_time: f32) -> SceneAction {

        if inputer.is_key_active() || inputer.is_mouse_active() {

            if inputer.is_key_pressed(GsKeycode::ESCAPE) {
                return SceneAction::Terminal
            }

            if inputer.is_key_pressed(GsKeycode::EQUALS) { // press '='
                self.lod_bias += 0.1;
            } else if inputer.is_key_pressed(GsKeycode::MINUS) { // press '-'
                self.lod_bias -= 0.1;
            }

            self.is_toggle_event = true;
            self.camera.react_input(inputer, delta_time);
        } else {
            self.is_toggle_event = false;
        }

        SceneAction::Rendering
    }
}
