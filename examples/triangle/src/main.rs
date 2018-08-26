
extern crate hakurei;

use hakurei::prelude::*;
use hakurei::pipeline::{ HaShaderInfo, ShaderStageType };
use hakurei::resources::command::{ HaCommandRecorder, CommandBufferUsageFlag };
use hakurei::resources::CommandError;

use std::path::Path;

const WINDOW_TITLE: &'static str = "Trangle Example";
const WINDOW_WIDTH:  u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

struct TriangleProcedure {

}

impl ProgramProc for TriangleProcedure {

    fn configure_shaders(&self) -> Vec<HaShaderInfo> {

        let vertex_shader = HaShaderInfo::setup(
            ShaderStageType::VertexStage,
            Path::new("src/triangle.vert.spv"),
            None);
        let fragment_shader = HaShaderInfo::setup(
            ShaderStageType::FragmentStage,
            Path::new("src/triangle.frag.spv"),
            None);

        vec![
            vertex_shader,
            fragment_shader,
        ]
    }

    fn configure_commands(&self, buffer: &HaCommandRecorder, frame_index: usize) -> Result<(), CommandError> {

        let usage_flags = [
            CommandBufferUsageFlag::SimultaneousUseBit
        ];

        buffer.begin_record(&usage_flags)?
            .begin_render_pass(frame_index)
            .bind_pipeline()
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
