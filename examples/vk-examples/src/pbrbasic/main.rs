
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
    let mut program_env = ProgramEnv::new(Some(manifest)).unwrap();

    let builder = program_env.routine().unwrap();

    let asset_loader = builder.assets_loader();
    let routine = VulkanExample::new(asset_loader).unwrap();
    let routine_flow = builder.build(routine);

    match routine_flow.launch(program_env) {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        },
    }
}
