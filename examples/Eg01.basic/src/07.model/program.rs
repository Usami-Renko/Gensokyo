
use ash::vk;
use gs::prelude::*;
use gsvk::prelude::common::*;
use gsvk::prelude::buffer::*;
use gsvk::prelude::image::*;
use gsvk::prelude::descriptor::*;
use gsvk::prelude::pipeline::*;
use gsvk::command::*;
use gsvk::sync::*;

use cgmath::{ Matrix4, SquareMatrix, Point3 };
use std::path::Path;

const VERTEX_SHADER_SOURCE_PATH  : &str = "src/07.model/model.vert";
const FRAGMENT_SHADER_SOURCE_PATH: &str = "src/07.model/model.frag";
const MODEL_GLTF_PATH: &str = "textures/triangle.gltf";

#[derive(Debug, Clone, Copy)]
pub struct UboObject {
    pub projection: Matrix4<f32>,
    pub view      : Matrix4<f32>,
    pub model     : Matrix4<f32>,
}

pub struct ModelProcedure {

    model: GltfEntity<Device>,
    buffer_storage: GsBufferRepository<Device>,

    graphics_pipeline: GsGraphicsPipeline,

    ubo_data   : Vec<UboObject>,
    ubo_storage: GsBufferRepository<Host>,
    ubo_buffer : GsUniformBlock,

    descriptor_storage: GsDescriptorRepository,
    descriptor_set    : DescriptorSet,

    depth_attachment: GsDepthStencilAttachment,
    image_storage: GsImageRepository,

    command_pool   : GsCommandPool,
    command_buffers: Vec<GsCommandBuffer>,

    camera: HaFlightCamera,

    present_availables: Vec<GsSemaphore>,
}

impl ModelProcedure {

    pub fn new(dimension: Dimension2D) -> ModelProcedure {

        let camera = CameraConfigurator::config()
            .place_at(Point3::new(0.0, 0.0, 3.0))
            .screen_aspect_ratio(dimension.width as f32 / dimension.height as f32)
            .for_flight_camera();

        ModelProcedure {

            model: GltfEntity::default(),
            buffer_storage: GsBufferRepository::empty(),

            graphics_pipeline: GsGraphicsPipeline::uninitialize(),

            ubo_data: vec![
                UboObject {
                    projection: camera.proj_matrix(),
                    view      : camera.view_matrix(),
                    model     : Matrix4::identity(),
                },
            ],
            ubo_storage: GsBufferRepository::empty(),
            ubo_buffer : HaUniformBlock::uninitialize(),
            descriptor_storage: GsDescriptorRepository::empty(),
            descriptor_set: DescriptorSet::unset(),

            depth_attachment: HaDepthStencilImage::uninitialize(),
            image_storage: GsImageRepository::empty(),

            command_pool: GsCommandPool::uninitialize(),
            command_buffers: vec![],

            camera,

            present_availables: vec![],
        }
    }

    fn update_uniforms(&mut self) -> Result<(), ProcedureError> {

        self.ubo_data[0].view  = self.camera.view_matrix();

        self.ubo_storage.data_updater()?
            .update(&self.ubo_buffer, &self.ubo_data)?
            .done()?;

        Ok(())
    }
}

impl ProgramProc for ModelProcedure {

    fn assets(&mut self, kit: AllocatorKit) -> Result<(), ProcedureError> {

        // load gltf model.
        let model_loader = kit.gltf_loader();
        self.model = model_loader.load_model(Path::new(MODEL_GLTF_PATH))?;

        self.model.config_buffer(&kit, BufferStorageType::Device)?;

        // uniform buffer
        let mut host_buffer_allocator = kit.buffer(BufferStorageType::Host);

        let ubo_info = UniformBlockInfo::new(0, 1, data_size!(self.ubo_data, UboObject));
        self.ubo_buffer = host_buffer_allocator.append_uniform(ubo_info)?;
        self.ubo_storage = host_buffer_allocator.allocate()?;

        self.ubo_storage.data_uploader()?
            .upload(&self.ubo_buffer, &self.ubo_data)?
            .done()?;

        // depth attachment image and model texture image
        let mut image_allocator = kit.image(ImageStorageType::Device);

        let mut depth_attachment_info = DepthStencilImageInfo::new_attachment(kit.swapchain_dimension());
        image_allocator.append_depth_stencil_image(&mut depth_attachment_info)?;

        let image_distributor = image_allocator.allocate()?;
        self.depth_attachment = image_distributor.acquire_depth_stencil_image(depth_attachment_info)?;

        self.image_storage = image_distributor.into_repository();

        // descriptor
        let mut descriptor_set_config = DescriptorSetConfig::init(&[]);
        descriptor_set_config.add_buffer_binding(
            &self.ubo_buffer,
            &[ShaderStageFlag::VertexStage]
        );

        let mut desc_allocator = kit.descriptor(&[]);
        let desc_index = desc_allocator.append_set(descriptor_set_config);

        let mut desc_distributor = desc_allocator.allocate()?;

        self.descriptor_set = desc_distributor.acquire_set(desc_index);
        self.descriptor_storage = desc_distributor.into_repository();

        Ok(())
    }

    fn pipelines(&mut self, kit: PipelineKit, swapchain: &HaSwapchain) -> Result<(), ProcedureError> {

        // shaders
        let vertex_shader = GsShaderInfo::from_source(
            ShaderStageFlag::VertexStage,
            Path::new(VERTEX_SHADER_SOURCE_PATH),
            None,
            "[Vertex Shader]");
        let fragment_shader = GsShaderInfo::from_source(
            ShaderStageFlag::FragmentStage,
            Path::new(FRAGMENT_SHADER_SOURCE_PATH),
            None,
            "[Fragment Shader]");
        let shader_infos = vec![
            vertex_shader,
            fragment_shader,
        ];

        // pipeline
        let mut render_pass_builder = kit.pass_builder();
        let first_subpass = render_pass_builder.new_subpass(PipelineType::Graphics);

        let color_attachment = RenderAttachement::setup(RenderAttachementPrefab::BackColorAttachment, swapchain.format);
        let _ = render_pass_builder.add_attachemnt(color_attachment, first_subpass, AttachmentType::Color);

        // TODO: Resign the API about Attachment.
        let depth_attachment = RenderAttachement::setup(RenderAttachementPrefab::DepthAttachment, self.depth_attachment.get_format());
        let _ = render_pass_builder.add_attachemnt(depth_attachment, first_subpass, AttachmentType::DepthStencil);
        render_pass_builder.set_depth_attachment(&self.depth_attachment);

        let mut dependency = RenderDependency::setup(RenderDependencyPrefab::Common, SUBPASS_EXTERAL, first_subpass);
        dependency.set_stage(PipelineStageFlag::ColorAttachmentOutputBit, PipelineStageFlag::ColorAttachmentOutputBit);
        dependency.set_access(&[], &[
            AccessFlag::ColorAttachmentReadBit,
            AccessFlag::ColorAttachmentWriteBit,
        ]);
        render_pass_builder.add_dependenty(dependency);

        let render_pass = render_pass_builder.build(swapchain)?;
        let viewport = GsViewportState::single(ViewportStateInfo::new(swapchain.extent));
        let depth_stencil = GsDepthStencilState::setup(GsDepthStencilPrefab::EnableDepth);

        let pipeline_config = GraphicsPipelineConfig::new(shader_infos, self.model.vertex_desc(), render_pass)
            .setup_viewport(ViewportStateType::Fixed { state: viewport })
            .setup_depth_stencil(depth_stencil)
            .add_descriptor_set(&self.descriptor_set)
            .finish();

        let mut pipeline_builder = kit.pipeline_builder(PipelineType::Graphics)?;
        let pipeline_index = pipeline_builder.add_config(pipeline_config);

        let mut pipelines = pipeline_builder.build()?;
        self.graphics_pipeline = pipelines.take_at(pipeline_index)?;
        
        Ok(())
    }

    fn subresources(&mut self, device: &GsDevice) -> Result<(), ProcedureError> {

        // sync
        for _ in 0..self.graphics_pipeline.frame_count() {
            let present_available = GsSemaphore::setup(device)?;
            self.present_availables.push(present_available);
        }

        Ok(())
    }

    fn commands(&mut self, kit: CommandKit) -> Result<(), ProcedureError> {

        self.command_pool = kit.pool(DeviceQueueIdentifier::Graphics)?;

        let command_buffer_count = self.graphics_pipeline.frame_count();
        let raw_commands = self.command_pool
            .allocate(CommandBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command) in raw_commands.into_iter().enumerate() {
            let mut recorder = kit.recorder(command);

            recorder.begin_record(&[CommandBufferUsageFlag::SimultaneousUseBit])?
                .begin_render_pass(&self.graphics_pipeline, frame_index)
                .bind_descriptor_sets(&self.graphics_pipeline, 0, &[&self.descriptor_set])
                .bind_pipeline(&self.graphics_pipeline);

            self.model.record_command(&recorder);

            recorder.end_render_pass();

            let command_recorded = recorder.end_record()?;
            self.command_buffers.push(command_recorded);
        }

        Ok(())
    }

    fn draw(&mut self, device: &GsDevice, device_available: &GsFence, image_available: &GsSemaphore, image_index: usize, _: f32) -> Result<&GsSemaphore, ProcedureError> {

        self.update_uniforms()?;

        let submit_infos = [
            QueueSubmitBundle {
                wait_semaphores: &[image_available],
                sign_semaphores: &[&self.present_availables[image_index]],
                wait_stages    : &[PipelineStageFlag::ColorAttachmentOutputBit],
                commands       : &[&self.command_buffers[image_index]],
            },
        ];

        device.submit(&submit_infos, Some(device_available), DeviceQueueIdentifier::Graphics)?;

        return Ok(&self.present_availables[image_index])
    }

    fn clean_resources(&mut self, _: &GsDevice) -> Result<(), ProcedureError> {

        self.present_availables.iter()
            .for_each(|semaphore| semaphore.cleanup());

        self.present_availables.clear();
        self.command_buffers.clear();

        self.graphics_pipeline.cleanup();
        self.command_pool.cleanup();

        Ok(())
    }

    fn cleanup(&mut self, _device: &GsDevice) {

        self.present_availables.iter()
            .for_each(|semaphore| semaphore.cleanup());

        self.graphics_pipeline.cleanup();
        self.command_pool.cleanup();
        self.image_storage.cleanup();
        self.descriptor_storage.cleanup();
        self.model.cleanup();
        self.ubo_storage.cleanup();
        self.buffer_storage.cleanup();
    }

    fn react_input(&mut self, inputer: &ActionNerve, delta_time: f32) -> SceneAction {

        if inputer.is_key_pressed(GsKeycode::Escape) {
            return SceneAction::Terminal
        }

        self.camera.react_input(inputer, delta_time);

        SceneAction::Rendering
    }
}
