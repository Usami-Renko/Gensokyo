
extern crate hakurei;

use hakurei::preinclude::*;

const WINDOW_TITLE: &'static str = "Trangle Example";
const WINDOW_WIDTH:  u32 = 1280;
const WINDOW_HEIGHT: u32 = 720;

struct TriangleProcedure {

}

impl ProgramProc for TriangleProcedure {

}

fn main() {

    let procecure = TriangleProcedure {};
    let mut program = ProgramBuilder::new(WINDOW_TITLE, procecure)
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .build().unwrap();

    program.launch();
}