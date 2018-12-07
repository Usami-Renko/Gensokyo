
use ash::vk;
use gs::prelude::*;
use gsvk::prelude::common::*;
use gsvk::prelude::buffer::*;
use gsvk::prelude::image::*;
use gsvk::prelude::descriptor::*;
use gsvk::prelude::pipeline::*;
use gsvk::command::*;
use gsvk::sync::*;

use gsma::data_size;

use super::data::{ Vertex, UboObject };
use super::data::{ VERTEX_DATA, INDEX_DATA };

use cgmath::{ Matrix4, SquareMatrix, Point3 };
use std::path::Path;

const VERTEX_SHADER_SOURCE_PATH  : &str = "src/06.depth/depth.vert";
const FRAGMENT_SHADER_SOURCE_PATH: &str = "src/06.depth/depth.frag";

pub struct DepthProcedure {

    index_data : Vec<vkuint>,
    ubo_data   : Vec<UboObject>,

    buffer_storage: GsBufferRepository<Host>,
    vertex_buffer : GsVertexBlock,
    index_buffer  : GsIndexBlock,
    ubo_buffer    : GsUniformBlock,

    pipeline: GsGraphicsPipeline,

    desc_storage: GsDescriptorRepository,
    ubo_set     : DescriptorSet,

    depth_attachment: GsDepthStencilAttachment,
    image_storage   : GsImageRepository<Device>,

    command_pool   : GsCommandPool,
    command_buffers: Vec<GsCommandBuffer>,

    camera: GsFlightCamera,

    present_availables: Vec<GsSemaphore>,
}

impl DepthProcedure {

    pub fn new(loader: AssetsLoader) -> Result<DepthProcedure, ProcedureError> {

        let screen_dimension = loader.screen_dimension();

        let camera = CameraConfigurator::config()
            .place_at(Point3::new(0.0, 0.0, 3.0))
            .screen_aspect_ratio(screen_dimension.width as f32 / screen_dimension.height as f32)
            .into_flight_camera();

        let vertex_data = VERTEX_DATA.to_vec();
        let index_data = INDEX_DATA.to_vec();

        let ubo_data = vec![
            UboObject {
                projection: camera.proj_matrix(),
                view      : camera.view_matrix(),
                model     : Matrix4::identity(),
            },
        ];

        let (vertex_buffer, index_buffer, ubo_buffer, buffer_storage) = loader.assets(|kit| {
            DepthProcedure::buffers(kit, &vertex_data, &index_data, &ubo_data)
        })?;

        let (depth_attachment, image_storage) = loader.assets(|kit| {
            DepthProcedure::image(kit)
        })?;

        let (ubo_set, desc_storage) = loader.assets(|kit| {
            DepthProcedure::ubo(kit, &ubo_buffer)
        })?;

        let pipeline = loader.pipelines(|kit| {
            DepthProcedure::pipelines(kit, &ubo_set, &depth_attachment)
        })?;

        let present_availables = loader.syncs(|kit| {
            DepthProcedure::sync_resources(kit, &pipeline)
        })?;

        let (command_pool, command_buffers) = loader.commands(|kit| {
            DepthProcedure::commands(kit, &pipeline, &vertex_buffer, &index_buffer, &ubo_set, index_data.len())
        })?;

        let procecure = DepthProcedure {
            index_data, ubo_data,
            buffer_storage, vertex_buffer, index_buffer, ubo_buffer,
            desc_storage, ubo_set,
            pipeline,
            depth_attachment, image_storage,
            command_pool, command_buffers,
            camera,
            present_availables,
        };

        Ok(procecure)
    }

    fn update_uniforms(&mut self) -> Result<(), ProcedureError> {

        self.ubo_data[0].view = self.camera.view_matrix();

        self.buffer_storage.data_updater()?
            .upload(&self.ubo_buffer, &self.ubo_data)?
            .finish()?;

        Ok(())
    }

    fn buffers(kit: AllocatorKit, vertex_data: &Vec<Vertex>, index_data: &Vec<vkuint>, ubo_data: &Vec<UboObject>) -> Result<(GsVertexBlock, GsIndexBlock, GsUniformBlock, GsBufferRepository<Host>), ProcedureError> {

        // vertex, index and uniform buffer
        let mut buffer_allocator = kit.buffer(BufferStorageType::HOST);

        let vertex_info = VertexBlockInfo::new(data_size!(vertex_data, Vertex));
        let vertex_index = buffer_allocator.append_buffer(vertex_info)?;

        let index_info = IndexBlockInfo::new(data_size!(index_data, vkuint));
        let index_index = buffer_allocator.append_buffer(index_info)?;

        let ubo_info = UniformBlockInfo::new(0, 1, data_size!(UboObject));
        let ubo_index = buffer_allocator.append_buffer(ubo_info)?;

        let buffer_distributor = buffer_allocator.allocate()?;

        let vertex_buffer = buffer_distributor.acquire_vertex(vertex_index);
        let index_buffer = buffer_distributor.acquire_index(index_index);
        let ubo_buffer = buffer_distributor.acquire_uniform(ubo_index)?;

        let mut buffer_storage = buffer_distributor.into_repository();
        buffer_storage.data_uploader()?
            .upload(&vertex_buffer, vertex_data)?
            .upload(&index_buffer, index_data)?
            .upload(&ubo_buffer, ubo_data)?
            .finish()?;

        Ok((vertex_buffer, index_buffer, ubo_buffer, buffer_storage))
    }

    fn image(kit: AllocatorKit) -> Result<(GsDepthStencilAttachment, GsImageRepository<Device>), ProcedureError> {

        // depth attachment image
        let mut image_allocator = kit.image(ImageStorageType::DEVICE);

        let mut depth_attachment_info = DepthStencilAttachmentInfo::new(kit.swapchain_dimension(), DepthStencilImageFormat::Depth32Bit);
        image_allocator.append_depth_stencil_image(&mut depth_attachment_info)?;

        let image_distributor = image_allocator.allocate()?;
        let depth_attachment = image_distributor.acquire_depth_stencil_image(depth_attachment_info)?;
        let image_storage = image_distributor.into_repository();

        Ok((depth_attachment, image_storage))
    }

    fn ubo(kit: AllocatorKit, ubo_buffer: &GsUniformBlock) -> Result<(DescriptorSet, GsDescriptorRepository), ProcedureError> {

        // descriptor
        let mut descriptor_set_config = DescriptorSetConfig::init(vk::DescriptorSetLayoutCreateFlags::empty());
        descriptor_set_config.add_buffer_binding(ubo_buffer, GsDescBindingStage::VERTEX);

        let mut descriptor_allocator = kit.descriptor(vk::DescriptorPoolCreateFlags::empty());
        let desc_index = descriptor_allocator.append_set(descriptor_set_config);

        let mut descriptor_distributor = descriptor_allocator.allocate()?;
        let ubo_set = descriptor_distributor.acquire_set(desc_index);

        let desc_storage = descriptor_distributor.into_repository();

        Ok((ubo_set, desc_storage))
    }

    fn pipelines(kit: PipelineKit, ubo_set: &DescriptorSet, depth_image: &GsDepthStencilAttachment) -> Result<GsGraphicsPipeline, ProcedureError> {

        // shaders
        let vertex_shader = GsShaderInfo::from_source(
            vk::ShaderStageFlags::VERTEX,
            Path::new(VERTEX_SHADER_SOURCE_PATH),
            None,
            "[Vertex Shader]");
        let fragment_shader = GsShaderInfo::from_source(
            vk::ShaderStageFlags::FRAGMENT,
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
        let first_subpass = render_pass_builder.new_subpass();

        let color_attachment = kit.present_attachment();
        let depth_attachment = depth_image.to_subpass_attachment();

        let _ = render_pass_builder.add_attachemnt(color_attachment, first_subpass);
        let _ = render_pass_builder.add_attachemnt(depth_attachment, first_subpass);

        render_pass_builder.set_depth_attachment(depth_image);

        let dependency = kit.subpass_dependency(SubpassStage::External, SubpassStage::AtIndex(first_subpass))
            .stage(vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT, vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT)
            .access(vk::AccessFlags::empty(), vk::AccessFlags::COLOR_ATTACHMENT_READ | vk::AccessFlags::COLOR_ATTACHMENT_WRITE);
        render_pass_builder.add_dependenty(dependency);

        let render_pass = render_pass_builder.build()?;
        let depth_stencil = GsDepthStencilState::setup(GsDepthStencilPrefab::EnableDepth);

        let pipeline_config = kit.pipeline_config(shader_infos, vertex_input_desc, render_pass)
            .setup_depth_stencil(depth_stencil)
            .add_descriptor_set(ubo_set)
            .finish();

        let mut pipeline_builder = kit.graphics_pipeline_builder()?;
        let pipeline_index = pipeline_builder.add_config(pipeline_config);

        let mut pipelines = pipeline_builder.build()?;
        let graphics_pipeline = pipelines.take_at(pipeline_index)?;

        Ok(graphics_pipeline)
    }

    fn sync_resources(kit: SyncKit, graphics_pipeline: &GsGraphicsPipeline) -> Result<Vec<GsSemaphore>, ProcedureError> {

        // sync
        let mut present_availables = vec![];
        for _ in 0..graphics_pipeline.frame_count() {
            let present_available = kit.semaphore()?;
            present_availables.push(present_available);
        }

        Ok(present_availables)
    }

    fn commands(kit: CommandKit, graphics_pipeline: &GsGraphicsPipeline, vertex_buffer: &GsVertexBlock, index_buffer: &GsIndexBlock, ubo_set: &DescriptorSet, index_count: usize) -> Result<(GsCommandPool, Vec<GsCommandBuffer>), ProcedureError> {

        let command_pool = kit.pool(DeviceQueueIdentifier::Graphics)?;
        let mut command_buffers = vec![];

        let command_buffer_count = graphics_pipeline.frame_count();
        let raw_commands = command_pool
            .allocate(CmdBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command) in raw_commands.into_iter().enumerate() {
            let mut recorder = kit.recorder(command);

            recorder.begin_record(vk::CommandBufferUsageFlags::SIMULTANEOUS_USE)?
                .begin_render_pass(graphics_pipeline, frame_index)
                .bind_pipeline(graphics_pipeline)
                .bind_vertex_buffers(0, &[vertex_buffer])
                .bind_index_buffer(index_buffer, 0)
                .bind_descriptor_sets(graphics_pipeline, 0, &[ubo_set])
                .draw_indexed(index_count as vkuint, 1, 0, 0, 0)
                .end_render_pass();

            let command_recorded = recorder.end_record()?;
            command_buffers.push(command_recorded);
        }

        Ok((command_pool, command_buffers))
    }
}

impl GraphicsRoutine for DepthProcedure {

    fn draw(&mut self, device: &GsDevice, device_available: &GsFence, image_available: &GsSemaphore, image_index: usize, _: f32) -> Result<&GsSemaphore, ProcedureError> {

        self.update_uniforms()?;

        let submit_infos = [
            QueueSubmitBundle {
                wait_semaphores: &[image_available],
                sign_semaphores: &[&self.present_availables[image_index]],
                wait_stages    : &[vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT],
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
        self.command_pool.cleanup();
        self.pipeline.cleanup();

        Ok(())
    }

    fn reload_res(&mut self, loader: AssetsLoader) -> Result<(), ProcedureError> {

        self.pipeline = loader.pipelines(|kit| {
            DepthProcedure::pipelines(kit, &self.ubo_set, &self.depth_attachment)
        })?;

        self.present_availables = loader.syncs(|kit| {
            DepthProcedure::sync_resources(kit, &self.pipeline)
        })?;

        let (command_pool, command_buffers) = loader.commands(|kit| {
            DepthProcedure::commands(kit, &self.pipeline, &self.vertex_buffer, &self.index_buffer, &self.ubo_set, self.index_data.len())
        })?;
        self.command_pool = command_pool;
        self.command_buffers = command_buffers;

        Ok(())
    }

    fn clean_routine(&mut self, _: &GsDevice) {

        self.present_availables.iter()
            .for_each(|semaphore| semaphore.cleanup());
        self.pipeline.cleanup();
        self.command_pool.cleanup();
        self.image_storage.cleanup();

        self.desc_storage.cleanup();
        self.buffer_storage.cleanup();
    }

    fn react_input(&mut self, inputer: &ActionNerve, delta_time: f32) -> SceneAction {

        if inputer.is_key_pressed(GsKeycode::ESCAPE) {
            return SceneAction::Terminal
        }

        self.camera.react_input(inputer, delta_time);

        SceneAction::Rendering
    }
}
