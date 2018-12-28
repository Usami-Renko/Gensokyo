
// TODO: Rename crate in Cargo.toml.
extern crate gensokyo as gs;
extern crate gensokyo_vulkan as gsvk;
extern crate gensokyo_macros as gsma;

mod program;

extern crate ash;
#[macro_use]
extern crate gensokyo_macros;
extern crate gensokyo_vulkan as gsvk;
extern crate gensokyo as gs;
extern crate cgmath;

use gs::prelude::*;

const MANIFEST_PATH: &str = "src/07.model/hakurei.toml";

use self::program::ModelProcedure;
use std::path::PathBuf;

fn main() {

    let manifest = PathBuf::from(MANIFEST_PATH);
    let mut program_env = ProgramEnv::new(Some(manifest)).unwrap();

    let mut routine_flow = {
        let builder = program_env.routine().unwrap();

        let asset_loader = builder.assets_loader();
        let routine = ModelProcedure::new(asset_loader).unwrap();
        builder.build(routine)
    };

    match routine_flow.launch(program_env) {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        },
    }
}
