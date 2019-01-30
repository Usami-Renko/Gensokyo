
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

mod data;
mod program;

use gs::prelude::*;

const MANIFEST_PATH: &'static str = "src/pbrbasic/Gensokyo.toml";

use self::program::VulkanExample;

use std::path::PathBuf;

fn main() {

let manifest = PathBuf::from(MANIFEST_PATH);
    let mut program_context = ProgramContext::new(Some(manifest)).unwrap();

    let builder = program_context.routine().unwrap();

    let initializer = builder.assets_loader();
    let routine = VulkanExample::new(initializer).unwrap();
    let routine_flow = builder.build(routine);

    match routine_flow.launch(program_context) {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        },
    }
}
