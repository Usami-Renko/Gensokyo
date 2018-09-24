
mod data;

pub use self::data::{ Vertex, UboObject };

#[macro_use]
extern crate hakurei_macros;
extern crate hakurei;
extern crate cgmath;

use hakurei::prelude::*;
use hakurei::prelude::config::*;
use hakurei::prelude::queue::*;
use hakurei::prelude::pipeline::*;
use hakurei::prelude::resources::*;
use hakurei::prelude::sync::*;
use hakurei::prelude::input::*;
use hakurei::prelude::utility::*;

use cgmath::{ Matrix4, SquareMatrix, Point3 };

use std::path::Path;

const WINDOW_TITLE: &'static str = "05.Cube";
const WINDOW_WIDTH:  u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const VERTEX_SHADER_PATH  : &'static str = "shaders/cube.vert.spv";
const FRAGMENT_SHADER_PATH: &'static str = "shaders/cube.frag.spv";

struct CubeProcedure {

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

    command_pool   : HaCommandPool,
    command_buffers: Vec<HaCommandBuffer>,

    camera: HaStageCamera,

    present_availables: Vec<HaSemaphore>,
}

impl CubeProcedure {

    fn new() -> CubeProcedure {
        let camera = CameraConfigurator::config()
            .place_at(Point3::new(0.0, 0.0, 3.0))
            .screen_dimension(WINDOW_WIDTH, WINDOW_HEIGHT)
            .for_stage_camera();

        CubeProcedure {
            vertex_data: data::VERTEX_DATA.to_vec(),
            index_data : data::INDEX_DATA.to_vec(),

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

            command_pool: HaCommandPool::uninitialize(),
            command_buffers: vec![],

            camera,

            present_availables: vec![],
        }
    }

    fn update_uniforms(&mut self) -> Result<(), ProcedureError> {

        self.ubo_data[0].model = self.camera.object_model_transformation();
        self.ubo_data[0].view  = self.camera.view_matrix();

        self.ubo_buffer.data_updater()?
            .update(&self.ubo_item, &self.ubo_data)?
            .done()?;

        Ok(())
    }
}

impl ProgramProc for CubeProcedure {

    fn assets(&mut self, _device: &HaDevice, kit: AllocatorKit) -> Result<(), ProcedureError> {

        // vertex, index buffer
        let mut device_buffer_allocator = kit.device_buffer();

        let mut vertex_buffer_config = DeviceBufferConfig::new(DeviceBufferUsage::VertexBuffer);
        vertex_buffer_config.add_item(data_size!(self.vertex_data, Vertex));

        let mut index_buffer_config = DeviceBufferConfig::new(DeviceBufferUsage::IndexBuffer);
        index_buffer_config.add_item(data_size!(self.index_data, uint32_t));

        self.vertex_item = device_buffer_allocator.attach_buffer(vertex_buffer_config)?.pop().unwrap();
        self.index_item  = device_buffer_allocator.attach_buffer(index_buffer_config)?.pop().unwrap();

        self.buffer_storage = device_buffer_allocator.allocate()?;
        self.buffer_storage.data_uploader()?
            .upload(&self.vertex_item, &self.vertex_data)?
            .upload(&self.index_item, &self.index_data)?
            .done()?;

        // uniform buffer
        let mut host_buffer_allocator = kit.host_buffer();

        let mut uniform_buffer_config = HostBufferConfig::new(HostBufferUsage::UniformBuffer);
        uniform_buffer_config.add_item(data_size!(self.ubo_data, UboObject));

        self.ubo_item = host_buffer_allocator.attach_buffer(uniform_buffer_config)?.pop().unwrap();
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
        self.ubo_storage.update_descriptors(&[ubo_descriptor_item]);
        self.ubo_set = descriptor_set_item;

        Ok(())
    }

    fn pipelines(&mut self, kit: PipelineKit, swapchain: &HaSwapchain) -> Result<(), ProcedureError> {
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
        let mut render_pass_builder = kit.pass_builder();
        let first_subpass = render_pass_builder.new_subpass(SubpassType::Graphics);

        let color_attachment = RenderAttachement::setup(RenderAttachementPrefab::BackColorAttachment, swapchain.format);
        let _attachment_index = render_pass_builder.add_attachemnt(color_attachment, first_subpass, AttachmentType::Color);

        let mut dependency = RenderDependency::setup(RenderDependencyPrefab::Common, SUBPASS_EXTERAL, first_subpass);
        dependency.set_stage(PipelineStageFlag::ColorAttachmentOutputBit, PipelineStageFlag::ColorAttachmentOutputBit);
        dependency.set_access(&[], &[
            AccessFlag::ColorAttachmentReadBit,
            AccessFlag::ColorAttachmentWriteBit,
        ]);
        render_pass_builder.add_dependenty(dependency);

        let render_pass = render_pass_builder.build(swapchain)?;
        let viewport = HaViewport::setup(swapchain.extent);

        let pipeline_config = GraphicsPipelineConfig::new(shader_infos, vertex_input_desc, render_pass)
            .setup_viewport(viewport)
            .add_descriptor_set(self.ubo_storage.set_layout_at(&self.ubo_set))
            .finish_config();

        let mut pipeline_builder = kit.graphics_pipeline_builder();
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

    fn commands(&mut self, device: &HaDevice) -> Result<(), ProcedureError> {
        // command buffer
        let command_pool = HaCommandPool::setup(&device, DeviceQueueIdentifier::Graphics, &[])?;

        let command_buffer_count = self.graphics_pipeline.frame_count();
        let command_buffers = command_pool
            .allocate(device, CommandBufferUsage::UnitaryCommand, command_buffer_count)?;

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
                .finish()?;
        }
        self.command_pool    = command_pool;
        self.command_buffers = command_buffers;

        Ok(())
    }

    fn ready(&mut self, device: &HaDevice) -> Result<(), ProcedureError> {

        self.ubo_buffer.ready_update(device)?;

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

    fn closure(&mut self, device: &HaDevice) -> Result<(), ProcedureError> {

        self.ubo_buffer.shut_update(device)?;

        Ok(())
    }

    fn clean_resources(&mut self, device: &HaDevice) -> Result<(), ProcedureError> {

        for semaphore in self.present_availables.iter() {
            semaphore.cleanup();
        }
        self.present_availables.clear();
        self.command_buffers.clear();

        self.graphics_pipeline.cleanup();
        self.command_pool.cleanup(device);

        Ok(())
    }

    fn cleanup(&mut self, device: &HaDevice) {

        for semaphore in self.present_availables.iter() {
            semaphore.cleanup();
        }

        self.graphics_pipeline.cleanup();
        self.command_pool.cleanup(device);
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

fn main() {

    let procecure = CubeProcedure::new();
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
