
mod data;
mod program;

use gs::prelude::*;

const MANIFEST_PATH: &str = "src/05.cube/Gensokyo.toml";

use self::program::CubeProcedure;
use std::path::PathBuf;

fn main() {

    let manifest = PathBuf::from(MANIFEST_PATH);
    let mut program_context = ProgramContext::new(Some(manifest)).unwrap();

    let builder = program_context.routine().unwrap();

    let asset_loader = builder.assets_loader();
    let routine = CubeProcedure::new(asset_loader).unwrap();
    let routine_flow = builder.build(routine);

    match routine_flow.launch(program_context) {
        | Ok(_) => (),
        | Err(err) => {
            panic!("[Error] {}", err)
        },
    }
}
