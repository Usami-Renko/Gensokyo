
#[macro_use]
extern crate hakurei;

use hakurei::prelude::*;
use hakurei::resources::prelude::*;
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


struct TriangleProcedure {


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

    fn configure_buffers(&mut self, allocator: &mut HaBufferAllocator) -> Result<(), AllocatorError> {

        let data = vec![
            Vertex { pos: [ 0.0, -0.5], color: [1.0, 1.0, 1.0, 1.0], },
            Vertex { pos: [ 0.5,  0.5], color: [0.0, 1.0, 0.0, 1.0], },
            Vertex { pos: [-0.5,  0.5], color: [0.0, 0.0, 1.0, 1.0], },
        ];

        let buffer_config = BufferConfig {
            data: &data,
            usage: BufferUsage::VertexBufferBit,
            buffer_flags: &[],
            memory_flags: &[
                MemoryPropertyFlag::HostVisibleBit,
                MemoryPropertyFlag::HostCoherentBit,
            ],
        };

        allocator.attach_buffer(buffer_config)?;
        allocator.allocate()?;
        allocator.tranfer_data(&data)?;

        Ok(())
    }

    fn configure_commands(&self, allocator: &HaBufferAllocator, buffer: &HaCommandRecorder, frame_index: usize) -> Result<(), CommandError> {

        let usage_flags = [
            CommandBufferUsageFlag::SimultaneousUseBit
        ];

        buffer.begin_record(&usage_flags)?
            .begin_render_pass(frame_index)
            .bind_pipeline()
//            .bind_vertex_buffers(0, &self.binding_infos.as_ref().unwrap())
            .bind_vertex_buffers(0, &allocator.binding_infos())
            .draw(3, 1, 0, 0)
            .end_render_pass()
            .finish()
    }

}

fn main() {

    let procecure = TriangleProcedure {};
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
