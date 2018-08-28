
#[macro_use]
extern crate hakurei;

use hakurei::prelude::*;
use hakurei::resources::prelude::*;
use hakurei::pipeline::prelude::*;
use hakurei::pipeline::shader::prelude::*;

use std::path::Path;

const WINDOW_TITLE: &'static str = "Trangle Example";
const WINDOW_WIDTH:  u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

define_input! {
    #[binding = 0, rate = vertex]
    struct Vertex {
        #[location = 0, format = vec2]
        pos:   [f32; 2],
        #[location = 1, format = vec4]
        color: [f32; 4],
    }
}

const VERTEX_DATA: [Vertex; 3] = [
    Vertex { pos: [ 0.0, -0.5], color: [0.5, 1.0, 1.0, 1.0], },
    Vertex { pos: [ 0.5,  0.5], color: [0.0, 1.0, 0.0, 1.0], },
    Vertex { pos: [-0.5,  0.5], color: [0.0, 0.0, 1.0, 1.0], },
];

struct TriangleProcedure {

    vertex_data  : Vec<Vertex>,
    vertex_buffer: HaBufferRepository,
}

impl ProgramProc for TriangleProcedure {

    fn configure_shaders(&self) -> VertexContent {
        let vertex_shader = HaShaderInfo::setup(
            ShaderStageType::VertexStage,
            Path::new("src/triangle.vert.spv"),
            None);
        let fragment_shader = HaShaderInfo::setup(
            ShaderStageType::FragmentStage,
            Path::new("src/triangle.frag.spv"),
            None);

        let infos = vec![
            vertex_shader,
            fragment_shader,
        ];

        VertexContent {
            infos,
            description: Vertex::desc(),
        }
    }


    fn configure_buffers(&mut self, device: &HaLogicalDevice, generator: &ResourceGenerator) -> Result<(), AllocatorError> {

        let buffer_config = BufferConfig {
            data: &self.vertex_data,
            usage: BufferUsage::VertexBufferBit,
            buffer_flags: &[],
            memory_flags: &[
                MemoryPropertyFlag::HostVisibleBit,
                MemoryPropertyFlag::HostCoherentBit,
            ],
        };

        let mut allocator = generator.buffer_allocator();
        allocator.attach_buffer(buffer_config)?;

        let repository = allocator.allocate()?;
        repository.tranfer_data(device, &self.vertex_data, 0)?;

        self.vertex_buffer = repository;

        Ok(())
    }

    fn configure_commands(&self, buffer: &HaCommandRecorder, frame_index: usize) -> Result<(), CommandError> {

        let usage_flags = [
            CommandBufferUsageFlag::SimultaneousUseBit
        ];

        buffer.begin_record(&usage_flags)?
            .begin_render_pass(frame_index)
            .bind_pipeline()
            .bind_vertex_buffers(0, &self.vertex_buffer.binding_infos())
            .draw(self.vertex_data.len() as uint32_t, 1, 0, 0)
            .end_render_pass()
            .finish()
    }

    fn cleanup(&self, device: &HaLogicalDevice) {

        self.vertex_buffer.cleanup(device);
    }
}

fn main() {

    let procecure = TriangleProcedure {
        vertex_data  : VERTEX_DATA.to_vec(),
        vertex_buffer: HaBufferRepository::empty(),
    };
    let mut program = ProgramBuilder::new(procecure)
        .title(WINDOW_TITLE)
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .build();

    match program.launch() {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        }
    }
}
