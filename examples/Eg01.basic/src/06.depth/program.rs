
use hakurei::prelude::*;
use hakurei::prelude::queue::*;
use hakurei::prelude::pipeline::*;
use hakurei::prelude::resources::*;
use hakurei::prelude::sync::*;
use hakurei::prelude::input::*;
use hakurei::prelude::utility::*;

use super::data::{ Vertex, UboObject };
use super::data::{ VERTEX_DATA, INDEX_DATA };

use cgmath::{ Matrix4, SquareMatrix, Point3 };
use std::path::Path;

const VERTEX_SHADER_SOURCE_PATH  : &'static str = "src/06.depth/depth.vert";
const FRAGMENT_SHADER_SOURCE_PATH: &'static str = "src/06.depth/depth.frag";

pub struct DepthProcedure {

    vertex_data: Vec<Vertex>,
    index_data : Vec<uint32_t>,

    buffer_storage: HaBufferRepository,
    vertex_buffer : HaVertexBlock,
    index_buffer  : HaIndexBlock,

    graphics_pipeline: HaGraphicsPipeline,

    ubo_data   : Vec<UboObject>,
    ubo_storage: HaBufferRepository,
    ubo_buffer : HaUniformBlock,

    desc_storage: HaDescriptorRepository,
    ubo_set    : DescriptorSetItem,

    depth_attachment: HaDepthStencilImage,
    image_storage: HaImageRepository,

    command_pool   : HaCommandPool,
    command_buffers: Vec<HaCommandBuffer>,

    camera: HaFlightCamera,

    present_availables: Vec<HaSemaphore>,
}

impl DepthProcedure {

    pub fn new() -> DepthProcedure {
        let camera = CameraConfigurator::config()
            .place_at(Point3::new(0.0, 0.0, 3.0))
            .screen_dimension(super::WINDOW_WIDTH, super::WINDOW_HEIGHT)
            .for_flight_camera();

        DepthProcedure {

            vertex_data: VERTEX_DATA.to_vec(),
            index_data : INDEX_DATA.to_vec(),

            buffer_storage: HaBufferRepository::empty(),
            vertex_buffer : HaVertexBlock::uninitialize(),
            index_buffer  : HaIndexBlock::uninitialize(),

            graphics_pipeline: HaGraphicsPipeline::uninitialize(),

            ubo_data: vec![
                UboObject {
                    projection: camera.proj_matrix(),
                    view      : camera.view_matrix(),
                    model     : Matrix4::identity(),
                },
            ],
            ubo_storage: HaBufferRepository::empty(),
            ubo_buffer: HaUniformBlock::uninitialize(),

            desc_storage: HaDescriptorRepository::empty(),
            ubo_set: DescriptorSetItem::unset(),

            depth_attachment: HaDepthStencilImage::uninitialize(),
            image_storage: HaImageRepository::empty(),

            command_pool: HaCommandPool::uninitialize(),
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

impl ProgramProc for DepthProcedure {

    fn assets(&mut self, kit: AllocatorKit) -> Result<(), ProcedureError> {

        // vertex, index buffer
        let mut device_buffer_allocator = kit.buffer(BufferStorageType::Device);

        let vertex_info = VertexBlockInfo::new(data_size!(self.vertex_data, Vertex));
        self.vertex_buffer = device_buffer_allocator.append_vertex(vertex_info)?;

        let index_info = IndexBlockInfo::new(data_size!(self.index_data, uint32_t));
        self.index_buffer = device_buffer_allocator.append_index(index_info)?;

        self.buffer_storage = device_buffer_allocator.allocate()?;
        self.buffer_storage.data_uploader()?
            .upload(&self.vertex_buffer, &self.vertex_data)?
            .upload(&self.index_buffer, &self.index_data)?
            .done()?;

        // uniform buffer
        let mut host_buffer_allocator = kit.buffer(BufferStorageType::Host);

        let uniform_info = UniformBlockInfo::new(0, 1, data_size!(self.ubo_data, UboObject));
        self.ubo_buffer = host_buffer_allocator.append_uniform(uniform_info)?;

        self.ubo_storage = host_buffer_allocator.allocate()?;

        self.ubo_storage.data_uploader()?
            .upload(&self.ubo_buffer, &self.ubo_data)?
            .done()?;

        // descriptor
        let mut descriptor_set_config = DescriptorSetConfig::init(&[]);
        let ubo_binding_index = descriptor_set_config.add_buffer_binding(&self.ubo_buffer, &[
            ShaderStageFlag::VertexStage,
        ])?;

        let mut descriptor_allocator = kit.descriptor(&[]);
        let (descriptor_set_item, descriptor_binding_items) = descriptor_allocator.attach_descriptor_set(descriptor_set_config);
        let ubo_descriptor_item = descriptor_binding_items[ubo_binding_index].clone();

        self.desc_storage = descriptor_allocator.allocate()?;
        self.desc_storage.update_descriptors(&[ubo_descriptor_item])?;
        self.ubo_set = descriptor_set_item;

        // depth attachment image
        let depth_attachment_info = DepthStencilImageInfo::new_attachment();
        let mut image_allocator = kit.image(ImageStorageType::Device);
        self.depth_attachment = image_allocator.attach_depth_stencil_image(depth_attachment_info, kit.swapchain_dimension())?;
        self.image_storage = image_allocator.allocate()?;
        self.image_storage.get_allocated_infos(&mut self.depth_attachment);

        Ok(())
    }

    fn pipelines(&mut self, kit: PipelineKit, swapchain: &HaSwapchain) -> Result<(), ProcedureError> {

        // shaders
        let vertex_shader = HaShaderInfo::from_source(
            ShaderStageFlag::VertexStage,
            Path::new(VERTEX_SHADER_SOURCE_PATH),
            None,
            "[Vertex Shader]");
        let fragment_shader = HaShaderInfo::from_source(
            ShaderStageFlag::FragmentStage,
            Path::new(FRAGMENT_SHADER_SOURCE_PATH),
            None,
            "[Fragment Shader]");
        let shader_infos = vec![
            vertex_shader,
            fragment_shader,
        ];
        let vertex_input_desc = Vertex::desc();

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
        let viewport = HaViewportState::single(ViewportStateInfo::new(swapchain.extent));
        let depth_stencil = HaDepthStencilState::setup(HaDepthStencilPrefab::EnableDepth);

        let pipeline_config = GraphicsPipelineConfig::new(shader_infos, vertex_input_desc, render_pass)
            .setup_viewport(ViewportStateType::Fixed { state: viewport })
            .setup_depth_stencil(depth_stencil)
            .add_descriptor_set(self.desc_storage.set_layout_at(&self.ubo_set))
            .finish_config();

        let mut pipeline_builder = kit.pipeline_builder(PipelineType::Graphics)?;
        pipeline_builder.add_config(pipeline_config);

        let mut graphics_pipelines = pipeline_builder.build()?;
        self.graphics_pipeline = graphics_pipelines.pop().unwrap();

        Ok(())
    }

    fn subresources(&mut self, device: &HaDevice) -> Result<(), ProcedureError> {

        // sync
        for _ in 0..self.graphics_pipeline.frame_count() {
            let present_available = HaSemaphore::setup(device)?;
            self.present_availables.push(present_available);
        }

        Ok(())
    }

    fn commands(&mut self, kit: CommandKit) -> Result<(), ProcedureError> {

        self.command_pool = kit.pool(DeviceQueueIdentifier::Graphics)?;

        let command_buffer_count = self.graphics_pipeline.frame_count();
        self.command_buffers = self.command_pool
            .allocate(CommandBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command_buffer) in self.command_buffers.iter().enumerate() {
            let recorder = command_buffer.setup_record();

            recorder.begin_record(&[CommandBufferUsageFlag::SimultaneousUseBit])?
                .begin_render_pass(&self.graphics_pipeline, frame_index)
                .bind_pipeline(&self.graphics_pipeline)
                .bind_vertex_buffers(0, &[&self.vertex_buffer])
                .bind_index_buffer(&self.index_buffer)
                .bind_descriptor_sets(&self.graphics_pipeline, 0, self.desc_storage.descriptor_binding_infos(&[&self.ubo_set]))
                .draw_indexed(self.index_data.len() as uint32_t, 1, 0, 0, 0)
                .end_render_pass()
                .end_record()?;
        }

        Ok(())
    }

    fn draw(&mut self, device: &HaDevice, device_available: &HaFence, image_available: &HaSemaphore, image_index: usize, _: f32) -> Result<&HaSemaphore, ProcedureError> {

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

    fn clean_resources(&mut self, _: &HaDevice) -> Result<(), ProcedureError> {

        self.present_availables.iter()
            .for_each(|semaphore| semaphore.cleanup());
        self.present_availables.clear();
        self.command_buffers.clear();

        self.graphics_pipeline.cleanup();
        self.command_pool.cleanup();

        Ok(())
    }

    fn cleanup(&mut self, _: &HaDevice) {

        self.present_availables.iter()
            .for_each(|semaphore| semaphore.cleanup());

        self.graphics_pipeline.cleanup();
        self.command_pool.cleanup();
        self.image_storage.cleanup();
        self.desc_storage.cleanup();
        self.ubo_storage.cleanup();
        self.buffer_storage.cleanup();
    }

    fn react_input(&mut self, inputer: &ActionNerve, delta_time: f32) -> SceneAction {

        if inputer.is_key_pressed(HaKeycode::Escape) {
            return SceneAction::Terminal
        }

        self.camera.react_input(inputer, delta_time);

        SceneAction::Rendering
    }
}
