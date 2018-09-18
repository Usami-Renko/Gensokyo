
#[macro_use]
extern crate hakurei_macros;
extern crate hakurei;

extern crate cgmath;
use cgmath::{ Matrix4, Vector3, Deg };

use hakurei::prelude::*;
use hakurei::prelude::config::*;
use hakurei::prelude::queue::*;
use hakurei::prelude::pipeline::*;
use hakurei::prelude::resources::*;
use hakurei::prelude::sync::*;
use hakurei::prelude::input::*;

use std::path::Path;

const WINDOW_TITLE: &'static str = "04.Unifrom";
const WINDOW_WIDTH:  u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const VERTEX_SHADER_PATH  : &'static str = "shaders/uniform.vert.spv";
const FRAGMENT_SHADER_PATH: &'static str = "shaders/uniform.frag.spv";

define_input! {
    #[binding = 0, rate = vertex]
    struct Vertex {
        #[location = 0, format = vec2]
        pos:   [f32; 2],
        #[location = 1, format = vec4]
        color: [f32; 4],
    }
}

#[derive(Debug, Clone, Copy)]
struct UboObject {
    rotate: Matrix4<f32>,
}

struct UniformBufferProcedure {

    vertex_data   : Vec<Vertex>,
    vertex_storage: HaBufferRepository,
    vertex_item   : BufferSubItem,

    ubo_data      : Vec<UboObject>,
    ubo_storage   : HaDescriptorRepository,
    ubo_set       : DescriptorSetItem,

    graphics_pipeline: HaGraphicsPipeline,

    command_pool   : HaCommandPool,
    command_buffers: Vec<HaCommandBuffer>,

    present_availables: Vec<HaSemaphore>,
}

impl UniformBufferProcedure {

    fn new() -> UniformBufferProcedure {
        UniformBufferProcedure {
            vertex_data: vec![
                Vertex { pos: [ 0.0, -0.5], color: [1.0, 0.0, 0.0, 1.0], },
                Vertex { pos: [ 0.5,  0.5], color: [0.0, 1.0, 0.0, 1.0], },
                Vertex { pos: [-0.5,  0.5], color: [0.0, 0.0, 1.0, 1.0], },
            ],
            vertex_storage: HaBufferRepository::empty(),
            vertex_item: BufferSubItem::unset(),

            ubo_data: vec![
                UboObject {
                    rotate: Matrix4::from_axis_angle(Vector3::new(0.0, 0.0, 1.0), Deg(90.0))
                },
            ],
            ubo_storage: HaDescriptorRepository::empty(),
            ubo_set: DescriptorSetItem::unset(),

            graphics_pipeline: HaGraphicsPipeline::uninitialize(),

            command_pool: HaCommandPool::uninitialize(),
            command_buffers: vec![],

            present_availables: vec![],
        }
    }
}

impl ProgramProc for UniformBufferProcedure {

    fn assets(&mut self, device: &HaLogicalDevice, generator: &ResourceGenerator) -> Result<(), ProcedureError> {

        // vertex and uniform buffer
        let mut vertex_buffer_config = BufferConfig::init(
            &[BufferUsageFlag::VertexBufferBit],
            &[
                MemoryPropertyFlag::HostVisibleBit,
                MemoryPropertyFlag::HostCoherentBit,
            ],
            &[]
        );
        let _ = vertex_buffer_config.add_item(data_size!(self.vertex_data, Vertex));

        let mut uniform_buffer_config = BufferConfig::init(
            &[BufferUsageFlag::UniformBufferBit],
            &[
                MemoryPropertyFlag::HostVisibleBit,
                MemoryPropertyFlag::HostCoherentBit,
            ],
            &[],
        );
        let _ = uniform_buffer_config.add_item(data_size!(self.ubo_data, UboObject));

        let mut vertex_allocator = generator.buffer();
        self.vertex_item = vertex_allocator.attach_buffer(vertex_buffer_config)?.pop().unwrap();
        let ubo_buffer_item = vertex_allocator.attach_buffer(uniform_buffer_config)?.pop().unwrap();

        self.vertex_storage = vertex_allocator.allocate()?;
        self.vertex_storage.tranfer_data(device, &self.vertex_data, &self.vertex_item)?;
        self.vertex_storage.tranfer_data(device, &self.ubo_data, &ubo_buffer_item)?;

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
            Path::new(VERTEX_SHADER_PATH),
            None);
        let fragment_shader = HaShaderInfo::setup(
            ShaderStageFlag::FragmentStage,
            Path::new(FRAGMENT_SHADER_PATH),
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

        let pipeline_config = GraphicsPipelineConfig::new(shader_infos, vertex_input_desc, render_pass)
            .setup_viewport(viewport)
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
                .bind_vertex_buffers(0, &self.vertex_storage.vertex_binding_infos(&[&self.vertex_item]))
                .bind_descriptor_sets(&self.graphics_pipeline, 0, &self.ubo_storage.descriptor_binding_infos(&[&self.ubo_set]))
                .draw(self.vertex_data.len() as uint32_t, 1, 0, 0)
                .end_render_pass()
                .finish()?;
        }
        self.command_pool    = command_pool;
        self.command_buffers = command_buffers;

        Ok(())
    }

    fn draw(&mut self, device: &HaLogicalDevice, device_available: &HaFence, image_available: &HaSemaphore, image_index: usize, _: f32) -> Result<&HaSemaphore, ProcedureError> {

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
        self.vertex_storage.cleanup(device);
    }

    fn react_input(&mut self, inputer: &ActionNerve, _: f32) -> SceneAction {

        if inputer.is_key_pressed(HaKeycode::Escape) {
            return SceneAction::Terminal
        }

        SceneAction::Rendering
    }
}

fn main() {

    let procecure = UniformBufferProcedure::new();
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
