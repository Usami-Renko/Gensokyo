
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
    vertex_item   : BufferSubItem,
    index_item    : BufferSubItem,

    graphics_pipeline: HaGraphicsPipeline,

    ubo_data   : Vec<UboObject>,
    ubo_buffer : HaBufferRepository,
    ubo_item   : BufferSubItem,
    ubo_storage: HaDescriptorRepository,
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
            vertex_item   : BufferSubItem::unset(),
            index_item    : BufferSubItem::unset(),

            graphics_pipeline: HaGraphicsPipeline::uninitialize(),

            ubo_data: vec![
                UboObject {
                    projection: camera.proj_matrix(),
                    view      : camera.view_matrix(),
                    model     : Matrix4::identity(),
                },
            ],
            ubo_buffer : HaBufferRepository::empty(),
            ubo_item   : BufferSubItem::unset(),
            ubo_storage: HaDescriptorRepository::empty(),
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

        self.ubo_buffer.data_updater()?
            .update(&self.ubo_item, &self.ubo_data)?
            .done()?;

        Ok(())
    }
}

impl ProgramProc for DepthProcedure {

    fn assets(&mut self, kit: AllocatorKit) -> Result<(), ProcedureError> {

        // vertex, index buffer
        let mut device_buffer_allocator = kit.buffer(BufferStorageType::Device);

        let mut vertex_buffer_config = DeviceBufferConfig::new(DeviceBufferUsage::VertexBuffer);
        vertex_buffer_config.add_item(data_size!(self.vertex_data, Vertex));

        let mut index_buffer_config = DeviceBufferConfig::new(DeviceBufferUsage::IndexBuffer);
        index_buffer_config.add_item(data_size!(self.index_data, uint32_t));

        self.vertex_item = device_buffer_allocator.attach_device_buffer(vertex_buffer_config)?.pop().unwrap();
        self.index_item  = device_buffer_allocator.attach_device_buffer(index_buffer_config)?.pop().unwrap();

        self.buffer_storage = device_buffer_allocator.allocate()?;
        self.buffer_storage.data_uploader()?
            .upload(&self.vertex_item, &self.vertex_data)?
            .upload(&self.index_item, &self.index_data)?
            .done()?;

        // uniform buffer
        let mut host_buffer_allocator = kit.buffer(BufferStorageType::Host);

        let mut uniform_buffer_config = HostBufferConfig::new(HostBufferUsage::UniformBuffer);
        uniform_buffer_config.add_item(data_size!(self.ubo_data, UboObject));

        self.ubo_item = host_buffer_allocator.attach_host_buffer(uniform_buffer_config)?.pop().unwrap();
        self.ubo_buffer = host_buffer_allocator.allocate()?;

        self.ubo_buffer.data_uploader()?
            .upload(&self.ubo_item, &self.ubo_data)?
            .done()?;

        // descriptor
        let ubo_info = DescriptorBufferBindingInfo {
            binding: 0,
            type_: BufferDescriptorType::UniformBuffer,
            count: 1,
            element_size: data_size!(self.ubo_data, UboObject),
            buffer: self.ubo_item.clone(),
        };
        let mut descriptor_set_config = DescriptorSetConfig::init(&[]);
        let ubo_binding_index = descriptor_set_config.add_buffer_binding(ubo_info, &[
            ShaderStageFlag::VertexStage,
        ]);

        let mut descriptor_allocator = kit.descriptor(&[]);
        let (descriptor_set_item, descriptor_binding_items) = descriptor_allocator.attach_descriptor_set(descriptor_set_config);
        let ubo_descriptor_item = descriptor_binding_items[ubo_binding_index].clone();

        self.ubo_storage = descriptor_allocator.allocate()?;
        self.ubo_storage.update_descriptors(&[ubo_descriptor_item])?;
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
        let first_subpass = render_pass_builder.new_subpass(SubpassType::Graphics);

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
        let viewport = HaViewport::setup(swapchain.extent);
        let depth_stencil = HaDepthStencil::setup(HaDepthStencilPrefab::EnableDepth);

        let pipeline_config = GraphicsPipelineConfig::new(shader_infos, vertex_input_desc, render_pass)
            .setup_viewport(viewport)
            .setup_depth_stencil(depth_stencil)
            .add_descriptor_set(self.ubo_storage.set_layout_at(&self.ubo_set))
            .finish_config();

        let mut pipeline_builder = kit.graphics_pipeline_builder()?;
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

        // command buffer
        let command_pool = kit.pool(DeviceQueueIdentifier::Graphics)?;

        let command_buffer_count = self.graphics_pipeline.frame_count();
        let command_buffers = command_pool
            .allocate(CommandBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command_buffer) in command_buffers.iter().enumerate() {
            let recorder = command_buffer.setup_record();

            recorder.begin_record(&[CommandBufferUsageFlag::SimultaneousUseBit])?
                .begin_render_pass(&self.graphics_pipeline, frame_index)
                .bind_pipeline(&self.graphics_pipeline)
                .bind_vertex_buffers(0, &self.buffer_storage.vertex_binding_infos(&[&self.vertex_item]))
                .bind_index_buffers(&self.buffer_storage.index_binding_info(&self.index_item))
                .bind_descriptor_sets(&self.graphics_pipeline, 0, &self.ubo_storage.descriptor_binding_infos(&[&self.ubo_set]))
                .draw_indexed(self.index_data.len() as uint32_t, 1, 0, 0, 0)
                .end_render_pass()
                .end_record()?;
        }
        self.command_pool    = command_pool;
        self.command_buffers = command_buffers;

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

        for semaphore in self.present_availables.iter() {
            semaphore.cleanup();
        }
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
        self.ubo_storage.cleanup();
        self.ubo_buffer.cleanup();
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
