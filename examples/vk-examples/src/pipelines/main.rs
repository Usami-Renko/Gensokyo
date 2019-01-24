
mod data;
mod program;

use gs::prelude::*;

const MANIFEST_PATH: &str = "src/pipelines/gensokyo.toml";

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
