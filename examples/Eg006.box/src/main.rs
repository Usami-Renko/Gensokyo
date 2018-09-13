
mod data;

pub use self::data::{ Vertex, UboObject };

#[macro_use]
extern crate hakurei_macros;
extern crate hakurei;
extern crate cgmath;

use hakurei::prelude::*;
use hakurei::pipeline::shader::prelude::*;
use hakurei::pipeline::graphics::prelude::*;
use hakurei::pipeline::pass::prelude::*;
use hakurei::pipeline::state::prelude::*;
use hakurei::resources::command::*;
use hakurei::resources::allocator::*;
use hakurei::resources::buffer::*;
use hakurei::resources::memory::*;
use hakurei::resources::repository::*;
use hakurei::resources::descriptor::*;
use hakurei::sync::prelude::*;

use cgmath::{ Matrix4, Vector3, Deg, SquareMatrix };

use std::path::Path;

const WINDOW_TITLE: &'static str = "Box Example";
const WINDOW_WIDTH:  u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

struct BoxProcedure {

    vertex_data  : Vec<Vertex>,
    index_data   : Vec<uint32_t>,
    vertex_buffer: HaBufferRepository,
    index_buffer : HaBufferRepository,
    vertex_item  : BufferSubItem,
    index_item   : BufferSubItem,

    graphics_pipeline: HaGraphicsPipeline,

    ubo_data      : Vec<UboObject>,
    ubo_buffer    : HaBufferRepository,
    ubo_storage   : HaDescriptorRepository,
    ubo_set       : DescriptorSetItem,

    command_pool   : HaCommandPool,
    command_buffers: Vec<HaCommandBuffer>,

    present_availables: Vec<HaSemaphore>,
}

impl BoxProcedure {

    fn new() -> BoxProcedure {
        BoxProcedure {
            vertex_data  : data::VERTEX_DATA.to_vec(),
            index_data   : data::INDEX_DATA.to_vec(),
            vertex_buffer: HaBufferRepository::empty(),
            index_buffer : HaBufferRepository::empty(),
            vertex_item  : BufferSubItem::unset(),
            index_item   : BufferSubItem::unset(),

            graphics_pipeline: HaGraphicsPipeline::uninitialize(),

            ubo_data: vec![
                UboObject {
                    translate: Matrix4::identity(),
                    scale    : Matrix4::from_scale(0.5),
                    rotate   : Matrix4::from_angle_x(Deg(45.0)) * Matrix4::from_angle_y(Deg(45.0)),
                },
            ],
            ubo_buffer: HaBufferRepository::empty(),
            ubo_storage: HaDescriptorRepository::empty(),
            ubo_set: DescriptorSetItem::unset(),

            command_pool: HaCommandPool::uninitialize(),
            command_buffers: vec![],

            present_availables: vec![],
        }
    }
}

impl ProgramProc for BoxProcedure {

    fn assets(&mut self, device: &HaLogicalDevice, generator: &ResourceGenerator) -> Result<(), ProcedureError> {

        // vertex buffer
        let mut staging_buffer_config = BufferConfig::init(
            &[BufferUsageFlag::TransferSrcBit],
            &[
                MemoryPropertyFlag::HostVisibleBit,
                MemoryPropertyFlag::HostCoherentBit,
            ],
            &[]
        );
        let _ =  staging_buffer_config.add_item(data_size!(self.vertex_data, Vertex));

        let mut vertex_buffer_config = BufferConfig::init(
            &[
                BufferUsageFlag::TransferDstBit,
                BufferUsageFlag::VertexBufferBit,
            ],
            &[MemoryPropertyFlag::DeviceLocalBit],
            &[]
        );
        let _ = vertex_buffer_config.add_item(data_size!(self.vertex_data, Vertex));

        let mut staging_allocator = generator.buffer();
        let staging_buffer_item = staging_allocator.attach_buffer(staging_buffer_config)?.pop().unwrap();

        let mut staging_repository = staging_allocator.allocate()?;
        staging_repository.tranfer_data(device, &self.vertex_data, &staging_buffer_item)?;

        let mut vertex_allocator = generator.buffer();
        self.vertex_item = vertex_allocator.attach_buffer(vertex_buffer_config)?.pop().unwrap();
        self.vertex_buffer = vertex_allocator.allocate()?;
        self.vertex_buffer.copy_buffer_to_buffer(device, &staging_buffer_item, &self.vertex_item)?;
        staging_repository.cleanup(device);

        // index buffer
        let mut staging_buffer_config = BufferConfig::init(
            &[BufferUsageFlag::TransferSrcBit],
            &[
                MemoryPropertyFlag::HostVisibleBit,
                MemoryPropertyFlag::HostCoherentBit,
            ],
            &[]
        );
        let _ = staging_buffer_config.add_item(data_size!(self.index_data, uint32_t));

        let mut index_buffer_config = BufferConfig::init(
            &[
                BufferUsageFlag::TransferDstBit,
                BufferUsageFlag::IndexBufferBit,
            ],
            &[MemoryPropertyFlag::DeviceLocalBit],
            &[]
        );
        let _ = index_buffer_config.add_item(data_size!(self.index_data, uint32_t));

        staging_allocator.reset();
        let staging_buffer_item = staging_allocator.attach_buffer(staging_buffer_config)?.pop().unwrap();

        let mut staging_repository = staging_allocator.allocate()?;
        staging_repository.tranfer_data(device, &self.index_data, &staging_buffer_item)?;

        let mut index_allocator = generator.buffer();
        self.index_item = index_allocator.attach_buffer(index_buffer_config)?.pop().unwrap();
        self.index_buffer = index_allocator.allocate()?;
        self.index_buffer.copy_buffer_to_buffer(device, &staging_buffer_item, &self.index_item)?;
        staging_repository.cleanup(device);

        // uniform buffer
        let mut uniform_buffer_config = BufferConfig::init(
            &[BufferUsageFlag::UniformBufferBit],
            &[
                MemoryPropertyFlag::HostVisibleBit,
                MemoryPropertyFlag::HostCoherentBit,
            ],
            &[],
        );
        let _ = uniform_buffer_config.add_item(data_size!(self.ubo_data, UboObject));

        let mut uniform_allocator = generator.buffer();
        let ubo_buffer_item = uniform_allocator.attach_buffer(uniform_buffer_config)?.pop().unwrap();
        self.ubo_buffer = uniform_allocator.allocate()?;
        self.ubo_buffer.tranfer_data(device, &self.ubo_data, &ubo_buffer_item)?;

        // descriptor
        let ubo_info = DescriptorBufferBindingInfo {
            binding: 0,
            type_: BufferDescriptorType::UniformBuffer,
            count: 1,
            element_size: data_size!(self.ubo_data, UboObject),
            buffer: ubo_buffer_item.clone(),
        };
        let mut descriptor_set_config = DescriptorSetConfig::init(&[]);
        let ubo_binding_index = descriptor_set_config.add_buffer_binding(ubo_info, &[
            ShaderStageFlag::VertexStage,
        ]);

        let mut descriptor_allocator = generator.descriptor(&[]);
        let (descriptor_set_item, descriptor_binding_items) = descriptor_allocator.attach_descriptor_set(descriptor_set_config);
        let ubo_descriptor_item = descriptor_binding_items[ubo_binding_index].clone();

        self.ubo_storage = descriptor_allocator.allocate()?;
        self.ubo_storage.update_descriptors(device, &[ubo_descriptor_item]);
        self.ubo_set = descriptor_set_item;

        Ok(())
    }

    fn pipelines(&mut self, device: &HaLogicalDevice, swapchain: &HaSwapchain) -> Result<(), ProcedureError> {
        // shaders
        let vertex_shader = HaShaderInfo::setup(
            ShaderStageFlag::VertexStage,
            Path::new("shaders/box.vert.spv"),
            None);
        let fragment_shader = HaShaderInfo::setup(
            ShaderStageFlag::FragmentStage,
            Path::new("shaders/box.frag.spv"),
            None);
        let shader_infos = vec![
            vertex_shader,
            fragment_shader,
        ];
        let vertex_input_desc = Vertex::desc();

        // pipeline
        let mut render_pass_builder = RenderPassBuilder::new();
        let first_subpass = render_pass_builder.new_subpass(SubpassType::Graphics);

        let color_attachment = RenderAttachement::setup(RenderAttachementPrefab::Present, swapchain.format);
        let _attachment_index = render_pass_builder.add_attachemnt(color_attachment, first_subpass, AttachmentType::Color);

        let mut dependency = RenderDependency::setup(RenderDependencyPrefab::Common, SUBPASS_EXTERAL, first_subpass);
        dependency.set_stage(PipelineStageFlag::ColorAttachmentOutputBit, PipelineStageFlag::ColorAttachmentOutputBit);
        dependency.set_access(&[], &[
            AccessFlag::ColorAttachmentReadBit,
            AccessFlag::ColorAttachmentWriteBit,
        ]);
        render_pass_builder.add_dependenty(dependency);

        let render_pass = render_pass_builder.build(device, swapchain)?;
        let viewport = HaViewport::setup(swapchain.extent);
        let depthstencil = HaDepthStencil::setup(HaDepthStencilPrefab::Disable);

        let pipeline_config = GraphicsPipelineConfig::init(shader_infos, vertex_input_desc, render_pass)
            .setup_viewport(viewport)
            .setup_depth_stencil(depthstencil)
            .add_descriptor_set(self.ubo_storage.set_layout_at(&self.ubo_set))
            .finish_config();

        let mut pipeline_builder = GraphicsPipelineBuilder::init();
        pipeline_builder.add_config(pipeline_config);

        let mut graphics_pipelines = pipeline_builder.build(device)?;
        self.graphics_pipeline = graphics_pipelines.pop().unwrap();

        Ok(())
    }

    fn subresources(&mut self, device: &HaLogicalDevice) -> Result<(), ProcedureError> {

        // sync
        for _ in 0..self.graphics_pipeline.frame_count() {
            let present_available = HaSemaphore::setup(device)?;
            self.present_availables.push(present_available);
        }

        Ok(())
    }

    fn commands(&mut self, device: &HaLogicalDevice) -> Result<(), ProcedureError> {
        // command buffer
        let command_pool = HaCommandPool::setup(&device, DeviceQueueIdentifier::Graphics, &[])?;

        let command_buffer_count = self.graphics_pipeline.frame_count();
        let command_buffers = command_pool
            .allocate(device, CommandBufferUsage::UnitaryCommand, command_buffer_count)?;

        for (frame_index, command_buffer) in command_buffers.iter().enumerate() {
            let recorder = command_buffer.setup_record(device);

            recorder.begin_record(&[CommandBufferUsageFlag::SimultaneousUseBit])?
                .begin_render_pass(&self.graphics_pipeline, frame_index)
                .bind_pipeline(&self.graphics_pipeline)
                .bind_vertex_buffers(0, &self.vertex_buffer.vertex_binding_infos(&[&self.vertex_item]))
                .bind_index_buffers(&self.index_buffer.index_binding_info(&self.index_item))
                .bind_descriptor_sets(&self.graphics_pipeline, 0, &self.ubo_storage.descriptor_binding_infos(&[&self.ubo_set]))
                .draw_indexed(self.index_data.len() as uint32_t, 1, 0, 0, 0)
                .end_render_pass()
                .finish()?;
        }
        self.command_pool    = command_pool;
        self.command_buffers = command_buffers;

        Ok(())
    }

    fn draw(&mut self, device: &HaLogicalDevice, device_available: &HaFence, image_available: &HaSemaphore, image_index: usize)
            -> Result<&HaSemaphore, ProcedureError> {

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

    fn clean_resources(&mut self, device: &HaLogicalDevice) -> Result<(), ProcedureError> {

        for semaphore in self.present_availables.iter() {
            semaphore.cleanup(device);
        }
        self.present_availables.clear();
        self.command_buffers.clear();

        self.graphics_pipeline.cleanup(device);
        self.command_pool.cleanup(device);

        Ok(())
    }

    fn cleanup(&mut self, device: &HaLogicalDevice) {

        for semaphore in self.present_availables.iter() {
            semaphore.cleanup(device);
        }

        self.graphics_pipeline.cleanup(device);
        self.command_pool.cleanup(device);
        self.ubo_storage.cleanup(device);
        self.ubo_buffer.cleanup(device);
        self.index_buffer.cleanup(device);
        self.vertex_buffer.cleanup(device);
    }

    fn react_input(&mut self, inputer: &ActionNerve) -> SceneAction {

        if inputer.is_key_pressed(HaKeycode::Escape) {
            return SceneAction::Terminal
        }

        SceneAction::Rendering
    }
}

fn main() {

    let procecure = BoxProcedure::new();
    let mut config = EngineConfig::default();
    config.window.dimension = Dimension2D {
        width : WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
    };
    config.window.title = String::from(WINDOW_TITLE);

    let mut program = ProgramEnv::new(config, procecure);

    match program.launch() {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        }
    }
}
