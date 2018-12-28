
// TODO: Rename crate in Cargo.toml.
extern crate gensokyo as gs;
extern crate gensokyo_vulkan as gsvk;
extern crate gensokyo_macros as gsma;

mod data;
mod program;

use gs::prelude::*;

const MANIFEST_PATH: &str = "src/06.depth/gensokyo.toml";

use self::program::DepthProcedure;
use std::path::PathBuf;

fn main() {

    let manifest = PathBuf::from(MANIFEST_PATH);
    let mut program_env = ProgramEnv::new(Some(manifest)).unwrap();

    let builder = program_env.routine().unwrap();

    let asset_loader = builder.assets_loader();
    let routine = DepthProcedure::new(asset_loader).unwrap();
    let routine_flow = builder.build(routine);

    match routine_flow.launch(program_env) {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        },
    }
}
